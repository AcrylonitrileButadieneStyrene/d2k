use crate::{
    Atom, ConvertError, Expr,
    parse::{self, Parser},
};

pub fn pointer(parser: &mut Parser, target: u32) -> Result<Expr, ConvertError> {
    match parser.peek() {
        Some(d2k_lexer::R2KSToken::Pointer(page)) => {
            parser.forward();
            parser.expect(d2k_lexer::R2KSToken::ParenOpen)?;
            parser.expect(d2k_lexer::R2KSToken::ParenClose)?;
            Ok(Expr::List(vec![
                Expr::Atom(Atom::symbol("call.direct")),
                Expr::Atom(Atom::U32(target)),
                Expr::Atom(Atom::U32(page)),
            ]))
        }
        _ => parse::stmt::assign::pointer(parser, target),
    }
}
