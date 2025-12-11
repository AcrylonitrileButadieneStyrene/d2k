use std::path::PathBuf;

use crate::stage::Stage;

#[derive(clap::Parser)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(clap::Subcommand)]
pub enum Command {
    Build {
        /// Path to either the project folder containing `D2K.toml` or an r2ks file.
        #[arg(index = 1, default_value = ".")]
        input: PathBuf,

        /// Multiple outputs can be specified. Type is inferred from file extension or can be assigned. A dash indicates writing to stdout.
        ///
        /// Examples: 
        /// - `d2k build example.r2ks -o tokens.tt -o st=syntax.tree`
        /// - `d2k build example/ -o il=il.lisp -o rmu=- -o Map0001.lmu`
        #[arg(short, action = clap::ArgAction::Append, value_parser = |x: &str| Ok::<_, std::convert::Infallible>(parse_outputs(x)))]
        outputs: Vec<(Stage, PathBuf)>,

        #[arg(short, long, value_enum, default_value = "shift-jis")]
        codepage: crate::CodePage,
    },
}

fn parse_outputs(output: &str) -> (Stage, PathBuf) {
    let convert = |x| match x {
        "tt" => Stage::Lex,
        "st" => Stage::Parse,
        "il" => Stage::Reduce,
        "rmu" => Stage::Compile, // rusty map unit because i am not using xml
        "lmu" => Stage::Serialize,
        x => {
            log::error!("Unrecognized extension `{x}` in output path `{output}`");
            std::process::exit(1);
        }
    };

    if let Some((ext, path)) = output.split_once('=') {
        (convert(ext), std::path::PathBuf::from(path))
    } else {
        let path = std::path::PathBuf::from(output);
        (
            path.extension()
                .and_then(|ext| ext.to_str())
                .map(convert)
                .unwrap(),
            path,
        )
    }
}
