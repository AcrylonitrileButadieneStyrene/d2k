use crate::{convert, grammar};

enum Left {
    Constant(u32),
    Range(u32, u32),
    Pointer(u32),
}

enum Right {
    Constant(u32),
    Variable(u32),
    Pointer(u32),
    Random(u32, u32),
}

pub fn convert_assignment_variable(
    mut assignment: pest::iterators::Pairs<'_, grammar::Rule>,
) -> Vec<crate::Inst> {
    let lhs = assignment.next().unwrap();
    let op = assignment.next().unwrap();
    let rhs = assignment.next().unwrap();

    let lhs = match lhs.as_rule() {
        grammar::Rule::variable => Left::Constant(convert::term(lhs).variable().unwrap()),
        grammar::Rule::range => {
            let mut iter = lhs.into_inner();
            Left::Range(
                convert::term(iter.next().unwrap()).variable().unwrap(),
                convert::term(iter.next().unwrap()).variable().unwrap(),
            )
        }
        grammar::Rule::pointer => Left::Pointer(
            convert::term(lhs.into_inner().next().unwrap())
                .variable()
                .unwrap(),
        ),
        _ => unreachable!(),
    };

    let rhs = match rhs.as_rule() {
        grammar::Rule::number => {
            Right::Constant(rhs.as_str().parse::<i32>().unwrap().cast_unsigned())
        }
        grammar::Rule::variable => Right::Variable(convert::term(rhs).variable().unwrap()),
        grammar::Rule::pointer => Right::Pointer(
            convert::term(rhs.into_inner().next().unwrap())
                .variable()
                .unwrap(),
        ),
        grammar::Rule::random => {
            let mut iter = rhs.into_inner();
            Right::Random(
                iter.next().unwrap().as_str().parse().unwrap(),
                iter.next().unwrap().as_str().parse().unwrap(),
            )
        }
        _ => unreachable!(),
    };

    crate::single((
        lcf::raw::lmu::event::instruction::Instruction::ControlVariables {
            mode: match lhs {
                Left::Constant(_) => 0,
                Left::Range(_, _) => 1,
                Left::Pointer(_) => 2,
            },
            start: match lhs {
                Left::Constant(x) | Left::Range(x, _) | Left::Pointer(x) => x,
            },
            end: match lhs {
                Left::Constant(x) | Left::Range(_, x) | Left::Pointer(x) => x,
            },
            operation: match op.as_str() {
                "=" => 0,
                "+=" => 1,
                "-=" => 2,
                "*=" => 3,
                "/=" => 4,
                "%=" => 5,
                _ => unreachable!(),
            },
            operand: match rhs {
                Right::Constant(_) => 0,
                Right::Variable(_) => 1,
                Right::Pointer(_) => 2,
                Right::Random(_, _) => 3,
            },
            value1: match rhs {
                Right::Constant(x)
                | Right::Variable(x)
                | Right::Pointer(x)
                | Right::Random(x, _) => x,
            },
            value2: match rhs {
                Right::Constant(_) | Right::Variable(_) | Right::Pointer(_) => 0,
                Right::Random(_, x) => x,
            },
        },
        None,
    ))

    // let op = match op.as_str() {

    //     _ => unreachable!(),
    // };
}
