use crate::{Atom, ConvertError, Expr};

pub fn assign_op(token: d2k_lexer::R2KSToken) -> Result<Expr, ConvertError> {
    Ok(match token {
        d2k_lexer::R2KSToken::AssignSet => Expr::Atom(Atom::symbol("variable.set")),
        d2k_lexer::R2KSToken::AssignAdd => Expr::Atom(Atom::symbol("variable.add")),
        d2k_lexer::R2KSToken::AssignSub => Expr::Atom(Atom::symbol("variable.sub")),
        d2k_lexer::R2KSToken::AssignMul => Expr::Atom(Atom::symbol("variable.mul")),
        d2k_lexer::R2KSToken::AssignDiv => Expr::Atom(Atom::symbol("variable.div")),
        d2k_lexer::R2KSToken::AssignMod => Expr::Atom(Atom::symbol("variable.mod")),
        _ => {
            return Err(crate::Expected::multiple(vec!["=", "+=", "-=", "*=", "/=", "%="]).into());
        }
    })
}
