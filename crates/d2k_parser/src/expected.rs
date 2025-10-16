#[derive(Debug)]
pub enum Expected {
    Single(String),
    Multiple(Vec<String>),
}

impl Expected {
    pub fn single(item: &str) -> Self {
        Self::Single(item.to_owned())
    }

    pub fn multiple(items: Vec<&str>) -> Self {
        Self::Multiple(items.into_iter().map(ToOwned::to_owned).collect())
    }
}
