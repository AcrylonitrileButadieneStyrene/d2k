#![feature(result_option_map_or_default)]

use d2k_mapgen::ManifestMap;
use lcf::{ConvertExt as _, raw::lmu::event::commands::Commands};

mod args;
mod codepage;

pub(crate) use codepage::CodePage;

fn main() {
    let args = <args::Args as clap::Parser>::parse();
    match args.command {
        args::Command::Lex { input } => {
            let src = std::fs::read_to_string(&input).unwrap();
            for (token, span) in &d2k_lexer::lex(input.to_str().unwrap(), &src) {
                println!("{span:?}: {token:?} ({})", &src[span.clone()]);
            }
        }
        args::Command::Parse { input } => {
            let src = std::fs::read_to_string(&input).unwrap();
            let name = input.to_str().unwrap();
            let file = codespan_reporting::files::SimpleFile::new(&name, &src);

            let tokens = d2k_lexer::lex(name, &src);
            match d2k_parser::parse(tokens) {
                Ok(ast) => println!("{ast:#?}"),
                Err(diagnostic) => d2k_errors::emit(&file, &diagnostic).unwrap(),
            }
        }
        args::Command::Build {
            input,
            out,
            codepage,
        } => {
            let encoding = codepage.to_encoding();
            let manifest = d2k_mapgen::Manifest::parse(
                &std::fs::read_to_string(input.join("D2K.toml")).unwrap(),
            );
            let src = std::fs::read_to_string(input.join("Events.ron")).unwrap();
            let commands = gather_commands(&input, encoding);

            let mut events = d2k_mapgen::build(&src, encoding, &commands).collect::<Vec<_>>();
            let mut map = match manifest.map {
                Some(ManifestMap {
                    extends: Some(extends),
                    ..
                }) if std::fs::exists(&extends).unwrap_or_default() => {
                    let buf = std::fs::read(&extends).unwrap();
                    lcf::lmu::LcfMapUnit::read(&mut std::io::Cursor::new(buf)).unwrap()
                }
                Some(ManifestMap {
                    width,
                    height,
                    chipset,
                    ..
                }) => lcf::lmu::LcfMapUnit {
                    width: width.unwrap_or(20),
                    height: height.unwrap_or(15),
                    chipset: chipset.unwrap_or(1),
                    ..Default::default()
                },
                None => lcf::lmu::LcfMapUnit::default(),
            };
            map.events.append(&mut events);

            if out.to_str().map_or_default(|str| str == "-") {
                println!("{map:?}");
            } else {
                let mut buf = std::io::Cursor::new(Vec::new());
                map.write(&mut buf).unwrap();
                std::fs::write(out, buf.into_inner()).unwrap();
            }
        }
    }
}

fn gather_commands(
    base: &std::path::Path,
    codepage: &'static encoding_rs::Encoding,
) -> std::collections::HashMap<std::sync::Arc<str>, std::sync::Arc<Commands>> {
    let mut will_terminate = false;

    let mut commands = std::collections::HashMap::new();
    for entry in std::fs::read_dir(base.join("Commands"))
        .unwrap()
        .filter_map(Result::ok)
    {
        let name = entry.file_name().to_str().unwrap().to_owned();
        let src = std::fs::read_to_string(entry.path()).unwrap();
        let tokens = d2k_lexer::lex(&name, &src);
        let ast = match d2k_parser::parse(tokens) {
            Ok(ast) => ast,
            Err(diagnostic) => {
                d2k_errors::emit(
                    &codespan_reporting::files::SimpleFile::new(&name, &src),
                    &diagnostic,
                )
                .unwrap();
                will_terminate = true;
                continue;
            }
        };

        commands.insert(
            entry.path().file_stem().unwrap().to_str().unwrap().into(),
            std::sync::Arc::new(d2k_codegen::build(ast, codepage)),
        );
    }

    if will_terminate {
        std::process::exit(1);
    }

    commands
}
