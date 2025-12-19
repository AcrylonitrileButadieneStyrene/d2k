use crate::{Atom, Expr, parse};

pub fn ident(parser: &mut parse::Parser, ident: String) -> Result<Expr, crate::ConvertError> {
    match parser.next() {
        d2k_lexer::R2KSToken::ParenOpen => {
            let mut args = Vec::new();
            loop {
                args.push(match parser.next() {
                    d2k_lexer::R2KSToken::Number(val) => Expr::Atom(Atom::I32(val)),
                    d2k_lexer::R2KSToken::String(str) => Expr::Atom(Atom::Text(str)),
                    _ => return Err(crate::Expected::multiple(vec!["Number", "String"]).into()),
                });

                match parser.next() {
                    d2k_lexer::R2KSToken::Comma => (),
                    d2k_lexer::R2KSToken::ParenClose => break,
                    _ => return Err(crate::Expected::multiple(vec!["Comma", "ParenClose"]).into()),
                }
            }

            let mut list = Vec::with_capacity(args.len() + 1);
            list.push(Expr::Atom(Atom::symbol(ident)));
            list.append(&mut args);
            Ok(Expr::List(list))
        }
        _ => Err(
            crate::Expected::multiple(vec!["If", "Switch", "Variable", "Pointer", "Loop"]).into(),
        ),
    }
}
