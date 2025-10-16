#[derive(Debug)]
pub enum Condition {
    SwitchComparison(u32, bool),
    VariableComparison(u32, Operation, Value),
}

#[derive(Debug)]
pub enum Operation {
    Eq,
    Le,
    Ge,
    Lt,
    Gt,
    Ne,
}

#[derive(Debug)]
pub enum Value {
    Constant(i32),
    Variable(u32),
}
