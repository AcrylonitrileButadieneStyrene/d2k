use crate::{parse, types};

pub fn ident(
    parser: &mut parse::Parser,
    ident: String,
) -> Result<types::Statement, crate::ParseError> {
    match parser.next() {
        d2k_lexer::Token::Colon => {
            parser.ast.labels.push(ident.clone());
            Ok(types::Statement::Label(ident))
        }
        d2k_lexer::Token::ParenOpen => {
            let mut args = Vec::new();
            loop {
                args.push(match parser.next() {
                    d2k_lexer::Token::Number(val) => types::Value::Number(val),
                    d2k_lexer::Token::String(str) => types::Value::String(str),
                    _ => return Err(crate::Expected::multiple(vec!["Number", "String"]).into()),
                });

                match parser.next() {
                    d2k_lexer::Token::Comma => (),
                    d2k_lexer::Token::ParenClose => break,
                    _ => return Err(crate::Expected::multiple(vec!["Comma", "ParenClose"]).into()),
                }
            }
            Ok(types::Statement::Call(ident, args))
        }
        _ => Err(
            crate::Expected::multiple(vec!["If", "Switch", "Variable", "Pointer", "Loop"]).into(),
        ),
    }
}
