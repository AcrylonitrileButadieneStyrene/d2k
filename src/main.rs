#![feature(result_option_map_or_default)]

use std::io::Write;

use lcf::{ConvertExt as _, raw::lmu::event::commands::Commands};

mod args;
mod codepage;
mod stage;

pub(crate) use codepage::CodePage;

fn main() {
    tracing_subscriber::fmt().init();

    let args = <args::Args as clap::Parser>::parse();
    match args.command {
        args::Command::Build {
            input,
            outputs,
            codepage,
        } => {
            let stages = crate::stage::Stages::from(outputs);

            if input.is_file() {
                let ext = input.extension().and_then(|ext| ext.to_str());
                match ext {
                    Some("r2ks") => {
                        vec![
                            stages.compile.as_ref().zip(Some("compile")),
                            stages.serialize.as_ref().zip(Some("serialize")),
                        ]
                        .into_iter()
                        .filter_map(|x| x)
                        .for_each(|(path, name)| {
                            log::warn!("Stage {name} is incompatible with single file r2ks mode. {} will not be written to.", path.display());
                        });

                        let src = std::fs::read_to_string(&input).unwrap();
                        let tokens = d2k_lexer::R2KSToken::from_file(input.to_str().unwrap(), &src);

                        if let Some(path) = stages.lex() {
                            let tokens = tokens.iter().map(|(token, span)| {
                                format!("{span:?}: {token:?} ({})", &src[span.clone()])
                            });

                            if path.eq("-") {
                                tokens.for_each(|line| println!("{line}"));
                            } else {
                                std::fs::write(path, tokens.collect::<Vec<_>>().join("\n"))
                                    .unwrap();
                            }
                        }

                        if let Some(path) = stages.parse {
                            let ast = match d2k_transpiler::parse(tokens) {
                                Ok(ast) => ast,
                                Err(diagnostic) => {
                                    let name = input.to_str().unwrap();
                                    let file =
                                        codespan_reporting::files::SimpleFile::new(&name, &src);
                                    d2k_common::emit(&file, &diagnostic).unwrap();
                                    return;
                                }
                            };

                            if path.eq("-") {
                                println!("{ast:#?}");
                            } else {
                                std::fs::write(path, format!("{ast:#?}")).unwrap();
                            }
                        }
                    }
                    Some("il2k") => {
                        todo!();
                    }
                    x => {
                        log::error!(
                            "Unrecognized input file extension {}",
                            x.unwrap_or("<none>")
                        );
                    }
                }
            } else if input.is_dir() {
                let expr = d2k_compose::compose(&input);

                if let Some(path) = &stages.reduce {
                    if path.eq("-") {
                        println!("{expr}");
                    } else {
                        std::fs::write(path, expr.to_string()).unwrap();
                    }
                }

                if let Some(path) = stages.compile() {
                    todo!();
                }

                // let manifest = d2k_compose::Manifest::parse(
                //     &std::fs::read_to_string(input.join("D2K.toml")).unwrap(),
                // );

                // let mut events = d2k_compose::build(
                //     &std::fs::read_to_string(input.join("Events.ron")).unwrap(),
                //     codepage.to_encoding(),
                //     &gather_commands(&input, codepage.to_encoding()),
                // )
                // .collect::<Vec<_>>();

                // let mut map = match manifest.map {
                //     Some(ManifestMap {
                //         extends: Some(extends),
                //         ..
                //     }) if std::fs::exists(&extends).unwrap_or_default() => {
                //         let buf = std::fs::read(&extends).unwrap();
                //         lcf::lmu::LcfMapUnit::read(&mut std::io::Cursor::new(buf)).unwrap()
                //     }
                //     Some(ManifestMap {
                //         width,
                //         height,
                //         chipset,
                //         ..
                //     }) => lcf::lmu::LcfMapUnit {
                //         width: width.unwrap_or(20),
                //         height: height.unwrap_or(15),
                //         chipset: chipset.unwrap_or(1),
                //         ..Default::default()
                //     },
                //     None => lcf::lmu::LcfMapUnit::default(),
                // };
                // map.events.append(&mut events);

                // let size = map.width as usize * map.height as usize;
                // map.lower.resize(size, 0);
                // map.upper.resize(size, 10000);

                // if let Some(path) = stages.compile() {
                //     let output = format!("{map:?}");
                //     if path.eq("-") {
                //         println!("{output}");
                //     } else {
                //         std::fs::write(path, output).unwrap();
                //     }
                // }

                // if let Some(path) = stages.serialize {
                //     let mut buf = std::io::Cursor::new(Vec::new());
                //     map.write(&mut buf).unwrap();

                //     if path.eq("-") {
                //         std::io::stdout().write_all(&buf.into_inner()).unwrap();
                //     } else {
                //         std::fs::write(path, buf.into_inner()).unwrap();
                //     }
                // }
            } else {
                log::error!("Input path was not a file nor a symlink");
            }
        }
    }
}
