use lcf::raw::lmu::event::instruction::Instruction;
use pest::iterators::Pair;

use crate::{convert, grammar};

pub fn convert_expression(expression: Pair<'_, grammar::Rule>) -> Vec<crate::Inst> {
    match expression.as_rule() {
        grammar::Rule::conditional => {
            let mut pairs = expression.into_inner();
            match (pairs.next(), pairs.next()) {
                (Some(r#if), None) => convert::conditional::r#if(r#if, false),
                (Some(r#if), Some(r#else)) => {
                    let mut commands = convert::conditional::r#if(r#if, true);
                    commands.push((Instruction::ElseBranch, None));
                    commands.extend(
                        r#else
                            .into_inner()
                            .next()
                            .unwrap()
                            .into_inner()
                            .flat_map(convert::expression),
                    );
                    commands.push((Instruction::End, None));
                    commands.push((Instruction::EndBranch, None));
                    commands
                }
                _ => unreachable!(),
            }
        }
        grammar::Rule::r#loop => {
            let mut inner = expression
                .into_inner()
                .next()
                .unwrap()
                .into_inner()
                .flat_map(convert::expression)
                .collect::<Vec<_>>();
            let mut vec = Vec::with_capacity(inner.len() + 3);
            vec.push((Instruction::Loop, None));
            vec.append(&mut inner);
            vec.push((Instruction::End, None));
            vec.push((Instruction::EndLoop, None));
            vec
        }
        grammar::Rule::instruction => crate::single(convert::instruction(expression.into_inner())),
        grammar::Rule::assignment => convert::assigment(expression.into_inner().next().unwrap()),
        grammar::Rule::comment => crate::single((
            Instruction::Comment,
            Some(expression.as_str()[1..].trim().to_string()),
        )),
        grammar::Rule::ignored | grammar::Rule::EOI => Vec::new(),
        _ => unreachable!(),
    }
}
