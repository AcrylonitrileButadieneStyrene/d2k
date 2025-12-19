#[derive(Debug, thiserror::Error)]
pub enum ConvertError {
    #[error("expected {0:?}")]
    Expected(super::Expected),
}

impl From<super::Expected> for ConvertError {
    fn from(value: super::Expected) -> Self {
        Self::Expected(value)
    }
}
