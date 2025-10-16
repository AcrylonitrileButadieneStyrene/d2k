use crate::{parse, read, types};

pub fn switch(
    parser: &mut parse::Parser,
    switch: u32,
) -> Result<types::Statement, crate::ParseError> {
    let destination = read::assign_destination(parser, switch, false)?;
    parser.expect(d2k_lexer::Token::AssignSet)?;
    let value = read::assign_switch_value(&parser.next())?;

    Ok(types::Statement::Assign(types::Assignment::Switch(
        destination,
        value,
    )))
}
