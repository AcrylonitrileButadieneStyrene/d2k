#[derive(serde::Deserialize)]
pub struct Manifest {
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub chipset: Option<u32>,
}

impl Manifest {
    pub fn parse(str: &str) -> Self {
        toml::from_str::<Manifest>(str).unwrap()
    }
}
