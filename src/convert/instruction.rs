use lcf::raw::lmu::event::instruction::Instruction;
use pest::{Parser, iterators::Pairs};

use crate::grammar;

pub fn convert_instruction(mut instruction: Pairs<'_, grammar::Rule>) -> crate::Inst {
    let operation = instruction.next().unwrap();

    match operation.as_str().to_lowercase().as_str() {
        "set" => {
            let destination = grammar::Parser::parse(
                grammar::Rule::instruction_set_destination,
                instruction.next().unwrap().as_str(),
            )
            .unwrap()
            .next()
            .unwrap();

            let dest = destination.as_str()[1..].parse().unwrap();
            let value = instruction.next().unwrap().as_str();

            (
                match destination.as_rule() {
                    grammar::Rule::switch => Instruction::ControlSwitches {
                        mode: 0,
                        start: dest,
                        end: dest,
                        operation: match value.to_lowercase().as_str() {
                            "on" | "true" => 0,
                            "off" | "false" => 1,
                            "invert" => 2,
                            _ => panic!(),
                        },
                    },
                    grammar::Rule::variable => Instruction::ControlVariables {
                        mode: 0,
                        start: dest,
                        end: dest,
                        operation: 0,
                        operand: 0,
                        value1: value.trim().parse().unwrap(),
                        value2: 0,
                    },
                    _ => unreachable!(),
                },
                None,
            )
        }
        x => panic!("unknown instruction {}", x),
    }
}
