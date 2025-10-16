use std::path::PathBuf;

#[derive(clap::Parser)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(clap::Subcommand)]
pub enum Command {
    Lex {
        #[arg(short)]
        input: PathBuf,
    },
    Parse {
        #[arg(short)]
        input: PathBuf,
    },
    Build {
        #[arg(short, default_value = ".")]
        input: PathBuf,

        #[arg(short, long, default_value = "-")]
        out: PathBuf,

        #[arg(short, long, value_enum, default_value = "shift-jis")]
        codepage: crate::CodePage,
    },
}
