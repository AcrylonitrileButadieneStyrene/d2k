use crate::{
    il2k::{Atom, Expr},
    r2ks::{self},
};

pub fn assign_switch_value(token: &d2k_lexer::R2KSToken) -> Result<Expr, r2ks::ParseError> {
    Ok(Expr::Atom(match token {
        d2k_lexer::R2KSToken::True => Atom::symbol("switch.enable"),
        d2k_lexer::R2KSToken::False => Atom::symbol("switch.disable"),
        d2k_lexer::R2KSToken::Toggle => Atom::symbol("switch.toggle"),
        _ => return Err(r2ks::Expected::multiple(vec!["True", "False", "Toggle"]).into()),
    }))
}
