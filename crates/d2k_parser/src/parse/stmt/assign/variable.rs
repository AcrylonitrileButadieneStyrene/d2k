use crate::{parse, read, types};

pub fn variable(
    parser: &mut parse::Parser,
    variable: u32,
) -> Result<types::Statement, crate::ParseError> {
    Ok(types::Statement::Assign(types::Assignment::Variable(
        read::assign_destination(parser, variable, true)?,
        read::assign_op(parser.next())?,
        {
            let token = parser.next();
            read::assign_variable_value(parser, &token)?
        },
    )))
}
