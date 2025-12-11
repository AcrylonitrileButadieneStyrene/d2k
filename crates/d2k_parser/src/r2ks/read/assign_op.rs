use crate::{
    il2k::{Atom, Expr},
    r2ks::{self},
};

pub fn assign_op(token: d2k_lexer::R2KSToken) -> Result<Expr, r2ks::ParseError> {
    Ok(match token {
        d2k_lexer::R2KSToken::AssignSet => Expr::Atom(Atom::symbol("variable.set")),
        d2k_lexer::R2KSToken::AssignAdd => Expr::Atom(Atom::symbol("variable.add")),
        d2k_lexer::R2KSToken::AssignSub => Expr::Atom(Atom::symbol("variable.sub")),
        d2k_lexer::R2KSToken::AssignMul => Expr::Atom(Atom::symbol("variable.mul")),
        d2k_lexer::R2KSToken::AssignDiv => Expr::Atom(Atom::symbol("variable.div")),
        d2k_lexer::R2KSToken::AssignMod => Expr::Atom(Atom::symbol("variable.mod")),
        _ => {
            return Err(r2ks::Expected::multiple(vec!["=", "+=", "-=", "*=", "/=", "%="]).into());
        }
    })
}
