use crate::{parse, read, types};

pub fn variable(
    parser: &mut parse::Parser,
    start: u32,
    end: u32,
) -> Result<types::Statement, crate::ParseError> {
    Ok(types::Statement::Assign(types::Assignment::Variable(
        if start == end {
            types::AssignmentDestination::Single(start)
        } else {
            types::AssignmentDestination::Range(start, end)
        },
        read::assign_op(parser.next())?,
        {
            let token = parser.next();
            read::assign_variable_value(parser, &token)?
        },
    )))
}
