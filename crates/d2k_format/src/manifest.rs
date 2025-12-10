#[derive(Debug, serde::Deserialize)]
pub struct Manifest {
    pub map: Option<ManifestMap>,
}

#[derive(Debug, Default, serde::Deserialize)]
pub struct ManifestMap {
    pub extends: Option<String>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub chipset: Option<u32>,
}

impl Manifest {
    pub fn parse(str: &str) -> Self {
        toml::from_str::<Manifest>(str).unwrap()
    }
}
