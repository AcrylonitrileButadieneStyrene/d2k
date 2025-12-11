#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("expected {0:?}")]
    Expected(super::Expected),
}

impl From<super::Expected> for ParseError {
    fn from(value: super::Expected) -> Self {
        Self::Expected(value)
    }
}
