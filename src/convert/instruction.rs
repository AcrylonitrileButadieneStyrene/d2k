use crate::grammar;

pub fn convert_instruction(
    mut instruction: pest::iterators::Pairs<'_, grammar::Rule>,
) -> crate::Inst {
    let operation = instruction.next().unwrap();

    match operation.as_str().to_lowercase().as_str() {
        x => panic!("unknown instruction {}", x),
    }
}
