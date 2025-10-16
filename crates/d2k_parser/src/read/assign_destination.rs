use crate::{parse, types};

pub fn assign_destination(
    parser: &mut parse::Parser,
    start: u32,
    is_var: bool,
) -> Result<types::AssignmentDestination, crate::ParseError> {
    let operator_or_colon_token = parser.peek();
    match operator_or_colon_token {
        d2k_lexer::Token::Colon => {
            parser.forward();
            let end = match parser.next() {
                d2k_lexer::Token::Variable(end) if is_var => end,
                d2k_lexer::Token::Switch(end) if !is_var => end,
                _ => {
                    return Err(crate::Expected::single(if is_var {
                        "Variable"
                    } else {
                        "Switch"
                    })
                    .into());
                }
            };

            Ok(types::AssignmentDestination::Range(start, end))
        }
        d2k_lexer::Token::AssignSet => Ok(types::AssignmentDestination::Single(start)),
        d2k_lexer::Token::AssignAdd
        | d2k_lexer::Token::AssignSub
        | d2k_lexer::Token::AssignMul
        | d2k_lexer::Token::AssignDiv
        | d2k_lexer::Token::AssignMod
            if is_var =>
        {
            Ok(types::AssignmentDestination::Single(start))
        }
        _ => Err(crate::Expected::multiple(if is_var {
            vec![":", "=", "+=", "-=", "*=", "/=", "%="]
        } else {
            vec![":", "="]
        })
        .into()),
    }
}
