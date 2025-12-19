use crate::{Atom, ConvertError, Expected, Expr, parse};

pub mod assign;

crate::export!(r#if);
crate::export!(pointer);

pub fn stmt(parser: &mut parse::Parser) -> Result<Expr, ConvertError> {
    match parser.next() {
        d2k_lexer::R2KSToken::If => parse::stmt::r#if(parser),
        d2k_lexer::R2KSToken::Loop => parser.parse_block(Some(Expr::Atom(Atom::symbol("loop")))),
        d2k_lexer::R2KSToken::Switch(start @ end)
        | d2k_lexer::R2KSToken::SwitchRange((start, end)) => {
            parse::stmt::assign::switch(parser, start, end)
        }
        d2k_lexer::R2KSToken::Variable(start @ end)
        | d2k_lexer::R2KSToken::VariableRange((start, end)) => {
            parse::stmt::assign::variable(parser, start, end)
        }
        d2k_lexer::R2KSToken::Pointer(val) => parse::stmt::pointer(parser, val),
        d2k_lexer::R2KSToken::Identifier(ident) => parse::ident(parser, ident),
        d2k_lexer::R2KSToken::Label(label) => Ok(Expr::List(vec![
            Expr::Atom(Atom::symbol("label")),
            Expr::Atom(Atom::label(label)),
        ])),
        d2k_lexer::R2KSToken::GoTo => match parser.next() {
            d2k_lexer::R2KSToken::Identifier(ident) => Ok(Expr::List(vec![
                Expr::Atom(Atom::symbol("goto")),
                Expr::Atom(Atom::label(ident)),
            ])),
            _ => Err(Expected::single("Identifier").into()),
        },
        d2k_lexer::R2KSToken::Comment(str) => Ok(Expr::Atom(Atom::Text(str))),
        d2k_lexer::R2KSToken::Destroy => Ok(Expr::Atom(Atom::symbol("erase"))),
        d2k_lexer::R2KSToken::Return => Ok(Expr::Atom(Atom::symbol("return"))),
        d2k_lexer::R2KSToken::CommonEvent(event) => {
            parser.expect(d2k_lexer::R2KSToken::ParenOpen)?;
            parser.expect(d2k_lexer::R2KSToken::ParenClose)?;

            Ok(Expr::List(vec![
                Expr::Atom(Atom::symbol("call.common")),
                Expr::Atom(Atom::U32(event)),
            ]))
        }
        d2k_lexer::R2KSToken::Event(event) => {
            let d2k_lexer::R2KSToken::Index(index) = parser.next() else {
                return Err(Expected::single("Index").into());
            };
            parser.expect(d2k_lexer::R2KSToken::ParenOpen)?;
            parser.expect(d2k_lexer::R2KSToken::ParenClose)?;

            Ok(Expr::List(vec![
                Expr::Atom(Atom::symbol("call.direct")),
                Expr::Atom(Atom::U32(event)),
                Expr::Atom(Atom::U32(index)),
            ]))
        }
        _ => Err(Expected::single("statement").into()),
    }
}
