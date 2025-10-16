use crate::{parse, types};

pub fn pointer(
    parser: &mut parse::Parser,
    target: u32,
) -> Result<types::Statement, crate::ParseError> {
    match parser.peek() {
        d2k_lexer::Token::Pointer(page) => {
            parser.forward();
            parser.expect(d2k_lexer::Token::ParenOpen)?;
            parser.expect(d2k_lexer::Token::ParenClose)?;
            Ok(types::Statement::CallMapEventVariable(target, page))
        }
        _ => parse::stmt::assign::pointer(parser, target),
    }
}
