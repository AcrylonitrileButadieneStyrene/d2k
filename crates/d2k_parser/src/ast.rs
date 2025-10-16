use crate::types;

#[derive(Debug, Default)]
pub struct AST {
    pub statements: Vec<types::Statement>,
    pub labels: Vec<String>,
}
