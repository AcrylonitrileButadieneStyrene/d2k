use crate::{Atom, ConvertError, Expected, Expr};

pub fn assign_switch_value(token: &d2k_lexer::R2KSToken) -> Result<Expr, ConvertError> {
    Ok(Expr::Atom(match token {
        d2k_lexer::R2KSToken::True => Atom::symbol("switch.enable"),
        d2k_lexer::R2KSToken::False => Atom::symbol("switch.disable"),
        d2k_lexer::R2KSToken::Toggle => Atom::symbol("switch.toggle"),
        _ => return Err(Expected::multiple(vec!["True", "False", "Toggle"]).into()),
    }))
}
