use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub ip: String,
    pub port: Option<u16>,
    pub keys: Keys,
}

#[derive(Deserialize)]
pub struct Keys {
    pub github: String,
    pub travis: Option<String>,
}
