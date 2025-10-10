use lcf::raw::lmu::event::instruction::Instruction;
use pest::iterators::Pair;

use crate::grammar;

pub fn r#if(r#if: Pair<'_, crate::grammar::Rule>, has_else: bool) -> Vec<crate::Inst> {
    let mut pairs = r#if.into_inner();
    let mut condition = pairs.next().unwrap().into_inner();
    let block = pairs.next().unwrap().into_inner();
    assert_eq!(pairs.next(), None);

    let variable = condition.next().unwrap();
    let variable_id = variable.as_str()[1..].parse().unwrap();

    let comparison = condition.next().unwrap();
    let value = condition.next().unwrap();

    let condition = match variable.as_rule() {
        grammar::Rule::switch => (
            Instruction::ConditionalBranch {
                mode: 0,
                field1: variable_id,
                field2: match value.as_str().to_lowercase().as_ref() {
                    "off" | "false" => 0,
                    "on" | "true" => 1,
                    _ => unreachable!(),
                } ^ match comparison.as_str() {
                    "==" => 0,
                    "!=" => 1,
                    _ => unreachable!(),
                },
                field3: 0,
                field4: 0,
                has_else: u32::from(has_else),
            },
            None,
        ),
        grammar::Rule::variable => (
            Instruction::ConditionalBranch {
                mode: 1,
                field1: variable_id,
                field2: match comparison.as_str() {
                    "==" => 0,
                    ">=" => 1,
                    "<=" => 2,
                    ">" => 3,
                    "<" => 4,
                    "!=" => 5,
                    _ => unreachable!(),
                },
                field3: match value.as_rule() {
                    grammar::Rule::number => 0,
                    grammar::Rule::variable => 1,
                    _ => unreachable!(),
                },
                field4: match value.as_rule() {
                    grammar::Rule::variable => &value.as_str()[1..],
                    _ => value.as_str(),
                }
                .parse()
                .unwrap(),
                has_else: u32::from(has_else),
            },
            None,
        ),
        _ => unreachable!(),
    };

    let mut commands = block
        .flat_map(super::expression::convert_expression)
        .collect::<Vec<_>>();

    let mut out = Vec::with_capacity(commands.len() + 3);
    out.push(condition);
    out.append(&mut commands);
    out.push((Instruction::End, None));
    if !has_else {
        out.push((Instruction::EndBranch, None));
    }
    out
}
