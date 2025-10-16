use crate::types;

pub fn assign_switch_value(
    token: &d2k_lexer::Token,
) -> Result<types::AssignmentSwitchValue, crate::ParseError> {
    Ok(match token {
        d2k_lexer::Token::True => types::AssignmentSwitchValue::On,
        d2k_lexer::Token::False => types::AssignmentSwitchValue::Off,
        d2k_lexer::Token::Toggle => types::AssignmentSwitchValue::Toggle,
        _ => return Err(crate::Expected::multiple(vec!["True", "False", "Toggle"]).into()),
    })
}
