use crate::{parse, read, types};

pub fn pointer(
    parser: &mut parse::Parser,
    target: u32,
) -> Result<types::Statement, crate::ParseError> {
    let destination = types::AssignmentDestination::Pointer(target);
    let operation = read::assign_op(parser.next())?;
    let value_token = parser.next();

    Ok(types::Statement::Assign(
        if matches!(operation, types::AssignmentVariableOperation::Set)
            && let Ok(value) = read::assign_switch_value(&value_token)
        {
            types::Assignment::Switch(destination, value)
        } else {
            types::Assignment::Variable(
                destination,
                operation,
                read::assign_variable_value(parser, &value_token)?,
            )
        },
    ))
}
