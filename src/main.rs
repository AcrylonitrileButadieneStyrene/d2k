#![warn(clippy::nursery)]
#![warn(clippy::pedantic)]

use lcf::raw::lmu::event::{command::Command, commands::Commands, instruction::Instruction};

mod codepage;
mod convert;
mod grammar;

type Inst = (Instruction, Option<String>);

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
    let commands =
        <grammar::Parser as pest::Parser<_>>::parse(grammar::Rule::commands, &string).unwrap();

    let output = Commands(
        commands
            .flat_map(convert::expression)
            .scan(0, |indentation, (instruction, string)| {
                let indent = *indentation;
                *indentation = (*indentation as i32 + instruction.indentation_change()) as u32;
                Some(Command {
                    indent,
                    instruction,
                    string: string
                        .map(|str| args.codepage.to_encoding().encode(&str).0.to_vec())
                        .unwrap_or_default(),
                })
            })
            .collect::<Vec<_>>(),
    );

    println!("{output:#?}");
}

fn single<T>(item: T) -> Vec<T> {
    vec![item]
}
