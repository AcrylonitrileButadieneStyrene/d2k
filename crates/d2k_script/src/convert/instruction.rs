use lcf::raw::lmu::event::instruction::Instruction;

use crate::{convert, grammar};

pub fn convert_instruction(
    ctx: &crate::Context,
    mut instruction: pest::iterators::Pairs<'_, grammar::Rule>,
) -> crate::Inst {
    let operation = instruction.next().unwrap();
    let arguments = instruction.next();

    #[allow(clippy::match_single_binding)]
    match operation.as_str().to_lowercase().as_str() {
        "return" => (Instruction::EndEventProcessing, None),
        "break" => (Instruction::BreakLoop, None),
        "erase" => (Instruction::EraseEvent, None),
        "wait" => (
            Instruction::Wait {
                deciseconds: (crate::next(arguments.unwrap())
                    .as_str()
                    .parse::<f32>()
                    .unwrap()
                    * 10.0) as u32,
                unknown: None,
            },
            None,
        ),
        "call" => {
            let mut args = <grammar::Parser as pest::Parser<_>>::parse(
                grammar::Rule::instruction_call,
                arguments.unwrap().into_inner().as_str(),
            )
            .unwrap();
            let target = args.next().unwrap();
            let arg = args.next();

            (
                match target.as_rule() {
                    grammar::Rule::common_event => Instruction::CallEvent {
                        mode: 0,
                        index: convert::term(target).common_event().unwrap(),
                        page: 0,
                    },
                    grammar::Rule::event => Instruction::CallEvent {
                        mode: 1,
                        index: convert::term(target).event().unwrap(),
                        page: arg.unwrap().as_str().parse().unwrap(),
                    },
                    grammar::Rule::pointer => Instruction::CallEvent {
                        mode: 2,
                        index: convert::term(crate::next(target)).variable().unwrap(),
                        page: crate::next(crate::next(arg.unwrap()))
                            .as_str()
                            .parse()
                            .unwrap(),
                    },
                    _ => unreachable!(),
                },
                None,
            )
        }
        "goto" => {
            let str = crate::next(arguments.clone().unwrap()).as_str();
            (
                Instruction::JumpToLabel {
                    value: ctx.labels.iter().position(|label| &**label == str).unwrap() as u32,
                },
                None,
            )
        }
        x => panic!("unknown instruction {x}"),
    }
}
