#![warn(clippy::nursery)]
#![warn(clippy::pedantic)]

use lcf::raw::lmu::event::{command::Command, commands::Commands, instruction::Instruction};

mod convert;
mod grammar;

type Inst = (Instruction, Option<String>);
type Pair<'a> = pest::iterators::Pair<'a, crate::grammar::Rule>;

fn single<T>(item: T) -> Vec<T> {
    vec![item]
}

pub fn parse(input: &str, codepage: &'static encoding_rs::Encoding) -> Commands {
    let commands =
        <grammar::Parser as pest::Parser<_>>::parse(grammar::Rule::commands, &input).unwrap();

    Commands(
        commands
            .flat_map(convert::expression)
            .scan(0, |indentation, (instruction, string)| {
                let indent = *indentation;
                *indentation = (*indentation as i32 + instruction.indentation_change()) as u32;
                Some(Command {
                    indent,
                    instruction,
                    string: string
                        .map(|str| codepage.encode(&str).0.to_vec())
                        .unwrap_or_default(),
                })
            })
            .collect::<Vec<_>>(),
    )
}
