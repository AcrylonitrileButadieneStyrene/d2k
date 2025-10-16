use crate::{parse, types};

pub fn assign_variable_value(
    parser: &mut parse::Parser,
    token: &d2k_lexer::Token,
) -> Result<types::AssignmentVariableValue, crate::ParseError> {
    Ok(match token {
        d2k_lexer::Token::Number(val) => types::AssignmentVariableValue::Constant(*val),
        d2k_lexer::Token::Variable(val) => types::AssignmentVariableValue::Variable(*val),
        d2k_lexer::Token::Pointer(val) => types::AssignmentVariableValue::Pointer(*val),
        d2k_lexer::Token::Random => {
            parser.expect(d2k_lexer::Token::ParenOpen)?;
            let d2k_lexer::Token::Number(val1) = parser.next() else {
                return Err(parser.expected("Number".to_string()));
            };
            parser.expect(d2k_lexer::Token::Comma)?;
            let d2k_lexer::Token::Number(val2) = parser.next() else {
                return Err(parser.expected("Number".to_string()));
            };
            parser.expect(d2k_lexer::Token::ParenClose)?;
            types::AssignmentVariableValue::Random(val1, val2)
        }
        _ => {
            return Err(crate::Expected::Multiple(
                vec!["Number", "Variable", "Switch", "Random"]
                    .into_iter()
                    .map(ToOwned::to_owned)
                    .collect(),
            )
            .into());
        }
    })
}
