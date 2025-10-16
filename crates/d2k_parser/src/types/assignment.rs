#[derive(Debug)]
pub enum Assignment {
    Switch(Destination, SwitchValue),
    Variable(Destination, VariableOperation, VariableValue),
}

#[derive(Debug)]
pub enum Destination {
    Single(u32),
    Range(u32, u32),
    Pointer(u32),
}

#[derive(Debug)]
pub enum VariableOperation {
    Set,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}

#[derive(Debug)]
pub enum VariableValue {
    Constant(i32),
    Variable(u32),
    Pointer(u32),
    Random(i32, i32),
}

#[derive(Debug)]
pub enum SwitchValue {
    On,
    Off,
    Toggle,
}
