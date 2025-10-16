use crate::{parse, types};

pub fn r#if(parser: &mut parse::Parser) -> Result<types::Statement, crate::ParseError> {
    Ok(types::Statement::If {
        condition: crate::switch!(parser.next(),
            d2k_lexer::Token::Switch(val) => types::Condition::SwitchComparison(
                val,
                crate::switch!(parser.next(),
                    d2k_lexer::Token::True => true,
                    d2k_lexer::Token::False => false,
                ),
            ),
            d2k_lexer::Token::Variable(val) => types::Condition::VariableComparison(
                val,
                crate::switch!(parser.next(),
                    d2k_lexer::Token::Eq => types::ConditionOperation::Eq,
                    d2k_lexer::Token::Le => types::ConditionOperation::Le,
                    d2k_lexer::Token::Ge => types::ConditionOperation::Ge,
                    d2k_lexer::Token::Lt => types::ConditionOperation::Lt,
                    d2k_lexer::Token::Gt => types::ConditionOperation::Gt,
                    d2k_lexer::Token::Ne => types::ConditionOperation::Ne,
                ),
                crate::switch!(parser.next(),
                    d2k_lexer::Token::Number(val) => types::ConditionValue::Constant(val),
                    d2k_lexer::Token::Variable(val) => types::ConditionValue::Variable(val),
                ),
            ),
        ),
        block: parser.parse_block()?,
        r#else: {
            if matches!(parser.peek(), d2k_lexer::Token::Else) {
                parser.forward();
                Some(parser.parse_block()?)
            } else {
                None
            }
        },
    })
}
