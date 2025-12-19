use crate::{Atom, ConvertError, Expected, Expr, parse};

pub fn assign_variable_value(
    parser: &mut parse::Parser,
    token: &d2k_lexer::R2KSToken,
) -> Result<Expr, ConvertError> {
    Ok(match token {
        d2k_lexer::R2KSToken::Number(val) => Expr::List(vec![
            Expr::Atom(Atom::symbol("const")),
            Expr::Atom(Atom::I32(*val)),
        ]),
        d2k_lexer::R2KSToken::Variable(val) => Expr::List(vec![
            Expr::Atom(Atom::symbol("single")),
            Expr::Atom(Atom::U32(*val)),
        ]),
        d2k_lexer::R2KSToken::Pointer(val) => Expr::List(vec![
            Expr::Atom(Atom::symbol("pointer")),
            Expr::Atom(Atom::U32(*val)),
        ]),
        d2k_lexer::R2KSToken::Random => {
            parser.expect(d2k_lexer::R2KSToken::ParenOpen)?;
            let d2k_lexer::R2KSToken::Number(val1) = parser.next() else {
                return Err(parser.expected("Number".to_string()));
            };
            parser.expect(d2k_lexer::R2KSToken::Comma)?;
            let d2k_lexer::R2KSToken::Number(val2) = parser.next() else {
                return Err(parser.expected("Number".to_string()));
            };
            parser.expect(d2k_lexer::R2KSToken::ParenClose)?;
            Expr::List(vec![
                Expr::Atom(Atom::symbol("random")),
                Expr::Atom(Atom::I32(val1)),
                Expr::Atom(Atom::I32(val2)),
            ])
        }
        _ => {
            return Err(Expected::Multiple(
                vec!["Number", "Variable", "Switch", "Random"]
                    .into_iter()
                    .map(ToOwned::to_owned)
                    .collect(),
            )
            .into());
        }
    })
}
