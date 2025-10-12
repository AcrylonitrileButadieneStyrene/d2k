#![warn(clippy::nursery)]
#![warn(clippy::pedantic)]

mod codepage;

#[derive(clap::Parser)]
struct Args {
    #[arg(short)]
    input: std::path::PathBuf,
    #[arg(short, long, value_enum)]
    codepage: codepage::CodePage,
}

fn main() {
    let args = <Args as clap::Parser>::parse();

    let string = std::fs::read_to_string(args.input).unwrap();
    let output = d2k_script::parse(&string, args.codepage.to_encoding());

    println!("{output:#?}");
}
