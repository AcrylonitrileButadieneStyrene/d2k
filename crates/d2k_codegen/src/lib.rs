use std::ops::Deref;

use d2k_parser::types::{
    Assignment, AssignmentDestination, AssignmentSwitchValue, AssignmentVariableOperation,
    AssignmentVariableValue, Condition, ConditionOperation, ConditionValue, Statement, Value,
};
use lcf::raw::lmu::event::{command::Command, commands::Commands, instruction::Instruction};

pub fn build(ast: d2k_parser::AST, codepage: &'static encoding_rs::Encoding) -> Commands {
    Commands(
        convert_vec(&ast, &ast.statements)
            .into_iter()
            .scan(0, |indentation, (instruction, string)| {
                let indent = *indentation;
                *indentation = ((*(indentation as &mut u32)).cast_signed()
                    + instruction.indentation_change())
                .cast_unsigned();
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

fn convert_vec(
    ast: &d2k_parser::AST,
    statements: &[Statement],
) -> Vec<(Instruction, Option<String>)> {
    statements
        .iter()
        .flat_map(|items| convert(ast, items))
        .collect::<Vec<_>>()
}

fn convert(ast: &d2k_parser::AST, statement: &Statement) -> Vec<(Instruction, Option<String>)> {
    match statement {
        Statement::If {
            condition,
            block,
            r#else,
        } => {
            let mut block = convert_vec(ast, block);
            let mut out = Vec::with_capacity(block.len() + 3);
            out.push((
                Instruction::ConditionalBranch {
                    mode: match condition {
                        Condition::SwitchComparison(_, _) => 0,
                        Condition::VariableComparison(_, _, _) => 1,
                    },
                    field1: match condition {
                        Condition::SwitchComparison(id, _) => *id,
                        Condition::VariableComparison(id, _, _) => *id,
                    },
                    field2: match &condition {
                        Condition::SwitchComparison(_, state) => match state {
                            true => 0,
                            false => 1,
                        },
                        Condition::VariableComparison(_, _, val) => match val {
                            ConditionValue::Constant(_) => 0,
                            ConditionValue::Variable(_) => 1,
                        },
                    },
                    field3: match &condition {
                        Condition::VariableComparison(_, _, val) => match val {
                            ConditionValue::Constant(val) => val.cast_unsigned(),
                            ConditionValue::Variable(id) => *id,
                        },
                        _ => 0,
                    },
                    field4: match &condition {
                        Condition::VariableComparison(_, op, _) => match op {
                            ConditionOperation::Eq => 0,
                            ConditionOperation::Ge => 1,
                            ConditionOperation::Le => 2,
                            ConditionOperation::Gt => 3,
                            ConditionOperation::Lt => 4,
                            ConditionOperation::Ne => 5,
                        },
                        _ => 0,
                    },
                    has_else: u32::from(r#else.is_some()),
                },
                None,
            ));
            out.append(&mut block);
            out.push((Instruction::End, None));
            if let Some(r#else) = r#else {
                out.push((Instruction::ElseBranch, None));
                out.append(&mut convert_vec(ast, r#else));
                out.push((Instruction::End, None));
            }
            out.push((Instruction::EndBranch, None));
            out
        }
        Statement::Loop(statements) => {
            let mut instructions = convert_vec(ast, statements);
            let mut vec = Vec::with_capacity(instructions.len() + 3);
            vec.push((Instruction::Loop, None));
            vec.append(&mut instructions);
            vec.push((Instruction::End, None));
            vec.push((Instruction::EndLoop, None));
            vec
        }
        Statement::Assign(assignment) => match assignment {
            Assignment::Switch(dest, val) => single((
                Instruction::ControlSwitches {
                    mode: match dest {
                        AssignmentDestination::Single(_) => 0,
                        AssignmentDestination::Range(_, _) => 1,
                        AssignmentDestination::Pointer(_) => 2,
                    },
                    start: match dest {
                        AssignmentDestination::Single(val)
                        | AssignmentDestination::Pointer(val)
                        | AssignmentDestination::Range(val, _) => *val,
                    },
                    end: match dest {
                        AssignmentDestination::Single(val)
                        | AssignmentDestination::Pointer(val)
                        | AssignmentDestination::Range(_, val) => *val,
                    },
                    operation: match val {
                        AssignmentSwitchValue::On => 0,
                        AssignmentSwitchValue::Off => 1,
                        AssignmentSwitchValue::Toggle => 2,
                    },
                },
                None,
            )),
            Assignment::Variable(dest, op, val) => single((
                Instruction::ControlVariables {
                    mode: match dest {
                        AssignmentDestination::Single(_) => 0,
                        AssignmentDestination::Range(_, _) => 1,
                        AssignmentDestination::Pointer(_) => 2,
                    },
                    start: match dest {
                        AssignmentDestination::Single(x)
                        | AssignmentDestination::Range(x, _)
                        | AssignmentDestination::Pointer(x) => *x,
                    },
                    end: match dest {
                        AssignmentDestination::Single(x)
                        | AssignmentDestination::Range(_, x)
                        | AssignmentDestination::Pointer(x) => *x,
                    },
                    operation: match op {
                        AssignmentVariableOperation::Set => 0,
                        AssignmentVariableOperation::Add => 1,
                        AssignmentVariableOperation::Sub => 2,
                        AssignmentVariableOperation::Mul => 3,
                        AssignmentVariableOperation::Div => 4,
                        AssignmentVariableOperation::Mod => 5,
                    },
                    operand: match val {
                        AssignmentVariableValue::Constant(_) => 0,
                        AssignmentVariableValue::Variable(_) => 1,
                        AssignmentVariableValue::Pointer(_) => 2,
                        AssignmentVariableValue::Random(_, _) => 3,
                    },
                    value1: match val {
                        AssignmentVariableValue::Constant(x)
                        | AssignmentVariableValue::Random(x, _) => x.cast_unsigned(),
                        AssignmentVariableValue::Variable(x)
                        | AssignmentVariableValue::Pointer(x) => *x,
                    },
                    value2: match val {
                        AssignmentVariableValue::Random(_, x) => x.cast_unsigned(),
                        AssignmentVariableValue::Constant(_)
                        | AssignmentVariableValue::Variable(_)
                        | AssignmentVariableValue::Pointer(_) => 0,
                    },
                },
                None,
            )),
        },
        Statement::Label(label) => single((
            Instruction::Label {
                value: ast.labels.iter().position(|x| x == label).unwrap() as u32,
            },
            None,
        )),
        Statement::GoTo(label) => single((
            Instruction::JumpToLabel {
                value: ast.labels.iter().position(|x| x == label).unwrap() as u32,
            },
            None,
        )),
        Statement::Comment(string) => single((Instruction::Comment, Some(string.clone()))),
        Statement::Destroy => single((Instruction::EraseEvent, None)),
        Statement::Return => single((Instruction::EndEventProcessing, None)),
        Statement::Call(command, values) => match (command.as_ref(), values.deref()) {
            ("wait", [Value::Number(deciseconds)]) if !deciseconds.is_negative() => single((
                Instruction::Wait {
                    deciseconds: deciseconds.cast_unsigned(),
                    unknown: None,
                },
                None,
            )),
            (x, args) => panic!("Unknown command `{x}/{}`", args.len()),
        },
        Statement::CallCommonEvent(event) => single((
            Instruction::CallEvent {
                mode: 0,
                index: *event,
                page: 0,
            },
            None,
        )),
        Statement::CallMapEventConstant(event, page) => single((
            Instruction::CallEvent {
                mode: 1,
                index: *event,
                page: *page,
            },
            None,
        )),
        Statement::CallMapEventVariable(event_var, page_var) => single((
            Instruction::CallEvent {
                mode: 2,
                index: *event_var,
                page: *page_var,
            },
            None,
        )),
    }
}

fn single<T>(item: T) -> Vec<T> {
    vec![item]
}
