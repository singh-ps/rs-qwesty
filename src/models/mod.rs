use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Asset {
    pub url: String,
    pub hash: String,
    pub name: String,
    pub version: String,
}

#[derive(Deserialize, Debug)]
pub struct Assets {
    pub assets: Vec<Asset>,
}
