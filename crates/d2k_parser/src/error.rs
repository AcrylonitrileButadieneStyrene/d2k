#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("expected {0:?}")]
    Expected(crate::Expected),
}

impl From<crate::Expected> for ParseError {
    fn from(value: crate::Expected) -> Self {
        Self::Expected(value)
    }
}
