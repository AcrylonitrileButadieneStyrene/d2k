use lcf::raw::lmu::event::instruction::Instruction;

use crate::{convert, grammar};

pub fn convert_expression(ctx: &crate::Context, expression: crate::Pair) -> Vec<crate::Inst> {
    match expression.as_rule() {
        grammar::Rule::conditional => {
            let mut pairs = expression.into_inner();
            match (pairs.next(), pairs.next()) {
                (Some(r#if), None) => convert::conditional::r#if(ctx, r#if, false),
                (Some(r#if), Some(r#else)) => {
                    let mut commands = convert::conditional::r#if(ctx, r#if, true);
                    commands.push((Instruction::ElseBranch, None));
                    commands.extend(
                        r#else
                            .into_inner()
                            .next()
                            .unwrap()
                            .into_inner()
                            .flat_map(|expression| convert::expression(ctx, expression)),
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
                .flat_map(|command| convert::expression(ctx, command))
                .collect::<Vec<_>>();
            let mut vec = Vec::with_capacity(inner.len() + 3);
            vec.push((Instruction::Loop, None));
            vec.append(&mut inner);
            vec.push((Instruction::End, None));
            vec.push((Instruction::EndLoop, None));
            vec
        }
        grammar::Rule::assignment => convert::assigment(expression.into_inner().next().unwrap()),
        grammar::Rule::label => {
            let str = expression.into_inner().next().unwrap().as_str();
            crate::single((
                Instruction::Label {
                    value: ctx.labels.iter().position(|label| &**label == str).unwrap() as u32,
                },
                None,
            ))
        }
        grammar::Rule::comment => crate::single((
            Instruction::Comment,
            Some(expression.as_str()[1..].trim().to_string()),
        )),
        grammar::Rule::instruction => {
            crate::single(convert::instruction(&ctx, expression.into_inner()))
        }
        grammar::Rule::ignored | grammar::Rule::EOI => Vec::new(),
        _ => unreachable!(),
    }
}
