#[derive(Debug)]
pub enum Statement {
    If {
        condition: super::conditional::Condition,
        block: Vec<Statement>,
        r#else: Option<Vec<Statement>>,
    },
    Loop(Vec<Statement>),
    Assign(super::assignment::Assignment),
    Label(String),
    GoTo(String),
    Comment(String),
    Destroy,
    Return,
    Call(String, Vec<super::Value>),
    CallCommonEvent(u32),
    CallMapEventConstant(u32, u32),
    CallMapEventVariable(u32, u32),
}
