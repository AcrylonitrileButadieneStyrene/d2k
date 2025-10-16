use crate::types;

pub fn assign_op(
    token: d2k_lexer::Token,
) -> Result<types::AssignmentVariableOperation, crate::ParseError> {
    Ok(match token {
        d2k_lexer::Token::AssignSet => types::AssignmentVariableOperation::Set,
        d2k_lexer::Token::AssignAdd => types::AssignmentVariableOperation::Add,
        d2k_lexer::Token::AssignSub => types::AssignmentVariableOperation::Sub,
        d2k_lexer::Token::AssignMul => types::AssignmentVariableOperation::Mul,
        d2k_lexer::Token::AssignDiv => types::AssignmentVariableOperation::Div,
        d2k_lexer::Token::AssignMod => types::AssignmentVariableOperation::Mod,
        _ => {
            return Err(crate::Expected::multiple(vec!["=", "+=", "-=", "*=", "/=", "%="]).into());
        }
    })
}
