use crate::{parse, types};

pub mod assign;

crate::export!(r#if);
crate::export!(pointer);

pub fn stmt(parser: &mut parse::Parser) -> Result<types::Statement, crate::ParseError> {
    match parser.next() {
        d2k_lexer::Token::If => parse::stmt::r#if(parser),
        d2k_lexer::Token::Loop => parser.parse_block().map(types::Statement::Loop),
        d2k_lexer::Token::Switch(start @ end) | d2k_lexer::Token::SwitchRange((start, end)) => {
            parse::stmt::assign::switch(parser, start, end)
        }
        d2k_lexer::Token::Variable(start @ end) | d2k_lexer::Token::VariableRange((start, end)) => {
            parse::stmt::assign::variable(parser, start, end)
        }
        d2k_lexer::Token::Pointer(val) => parse::stmt::pointer(parser, val),
        d2k_lexer::Token::Identifier(ident) => parse::ident(parser, ident),
        d2k_lexer::Token::Label(label) => {
            parser.ast.labels.push(label.clone());
            Ok(types::Statement::Label(label))
        }
        d2k_lexer::Token::GoTo => match parser.next() {
            d2k_lexer::Token::Identifier(ident) => Ok(types::Statement::GoTo(ident)),
            _ => Err(crate::Expected::single("Identifier").into()),
        },
        d2k_lexer::Token::Comment(str) => Ok(types::Statement::Comment(str)),
        d2k_lexer::Token::Destroy => Ok(types::Statement::Destroy),
        d2k_lexer::Token::Return => Ok(types::Statement::Return),
        d2k_lexer::Token::CommonEvent(event) => {
            parser.expect(d2k_lexer::Token::ParenOpen)?;
            parser.expect(d2k_lexer::Token::ParenClose)?;
            Ok(types::Statement::CallCommonEvent(event))
        }
        d2k_lexer::Token::Event(event) => {
            let d2k_lexer::Token::Index(index) = parser.next() else {
                return Err(crate::Expected::single("Index").into());
            };
            parser.expect(d2k_lexer::Token::ParenOpen)?;
            parser.expect(d2k_lexer::Token::ParenClose)?;

            Ok(types::Statement::CallMapEventConstant(event, index))
        }
        _ => Err(crate::Expected::single("statement").into()),
    }
}
