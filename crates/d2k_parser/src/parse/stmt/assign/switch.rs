use crate::{parse, read, types};

pub fn switch(
    parser: &mut parse::Parser,
    start: u32,
    end: u32,
) -> Result<types::Statement, crate::ParseError> {
    parser.expect(d2k_lexer::Token::AssignSet)?;

    Ok(types::Statement::Assign(types::Assignment::Switch(
        if start == end {
            types::AssignmentDestination::Single(start)
        } else {
            types::AssignmentDestination::Range(start, end)
        },
        read::assign_switch_value(&parser.next())?,
    )))
}
