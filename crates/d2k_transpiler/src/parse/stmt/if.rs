use crate::{Atom, ConvertError, Expr, parse};

pub fn r#if(parser: &mut parse::Parser) -> Result<Expr, ConvertError> {
    let mut exprs = Vec::with_capacity(4);
    exprs.push(Expr::Atom(Atom::symbol("if")));
    exprs.push(Expr::List(crate::switch!(parser.next(),
        d2k_lexer::R2KSToken::Switch(val) => vec![
            Expr::Atom(Atom::symbol("switch.equals")),
            Expr::Atom(Atom::U32(val)),
            Expr::Atom(Atom::Bool(crate::switch!(parser.next(),
                d2k_lexer::R2KSToken::True => true,
                d2k_lexer::R2KSToken::False => false,
            )))
        ],
        d2k_lexer::R2KSToken::Variable(val) => vec![
            Expr::Atom(Atom::symbol(crate::switch!(parser.next(),
                d2k_lexer::R2KSToken::Eq => "variable.eq",
                d2k_lexer::R2KSToken::Le => "variable.le",
                d2k_lexer::R2KSToken::Ge => "variable.ge",
                d2k_lexer::R2KSToken::Lt => "variable.lt",
                d2k_lexer::R2KSToken::Gt => "variable.gt",
                d2k_lexer::R2KSToken::Ne => "variable.ne",
            ))),
            Expr::List(vec![
                Expr::Atom(Atom::symbol("single")),
                Expr::Atom(Atom::U32(val))
            ]),
            Expr::List(crate::switch!(parser.next(),
                d2k_lexer::R2KSToken::Number(val) => vec![
                    Expr::Atom(Atom::symbol("const")),
                    Expr::Atom(Atom::I32(val))
                ],
                d2k_lexer::R2KSToken::Variable(val) => vec![
                    Expr::Atom(Atom::symbol("single")),
                    Expr::Atom(Atom::U32(val)),
                ],
            )),
        ]
    )));

    exprs.push(parser.parse_block(Some(Expr::Atom(Atom::symbol("then"))))?);

    if matches!(parser.peek(), Some(d2k_lexer::R2KSToken::Else)) {
        parser.forward();
        exprs.push(parser.parse_block(Some(Expr::Atom(Atom::symbol("else"))))?);
    }

    Ok(Expr::List(exprs))
}
