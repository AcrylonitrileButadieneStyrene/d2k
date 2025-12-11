use crate::{
    il2k::{Atom, Expr},
    r2ks::{self, parse},
};

pub fn pointer(parser: &mut parse::Parser, target: u32) -> Result<Expr, r2ks::ParseError> {
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
