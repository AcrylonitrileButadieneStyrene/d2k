use crate::{convert, grammar};

enum Left {
    Constant(u32),
    Range(u32, u32),
    Pointer(u32),
}

pub fn convert_assignment_switch(
    mut assignment: pest::iterators::Pairs<'_, grammar::Rule>,
) -> Vec<crate::Inst> {
    let lhs = assignment.next().unwrap();
    let rhs = assignment.next().unwrap();

    let lhs = match lhs.as_rule() {
        grammar::Rule::switch => Left::Constant(convert::term(lhs).switch().unwrap()),
        grammar::Rule::range => {
            let mut iter = lhs.into_inner();
            Left::Range(
                convert::term(iter.next().unwrap()).switch().unwrap(),
                convert::term(iter.next().unwrap()).switch().unwrap(),
            )
        }
        grammar::Rule::pointer => Left::Pointer(
            convert::term(lhs.into_inner().next().unwrap())
                .variable()
                .unwrap(),
        ),
        _ => unreachable!(),
    };

    crate::single((
        lcf::raw::lmu::event::instruction::Instruction::ControlSwitches {
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
            operation: match rhs.as_str().to_lowercase().as_ref() {
                "true" | "on" => 0,
                "false" | "off" => 1,
                "toggle" => 2,
                _ => unreachable!(),
            },
        },
        None,
    ))

    // let op = match op.as_str() {

    //     _ => unreachable!(),
    // };
}
