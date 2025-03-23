use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std_utils::Result;

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigPair {
    pub config: String,
    pub file: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Defaults {
    pub fps: i32,
    pub width: u32,
    pub height: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Renderer {
    pub zoom: u32,
    pub xxo: i32,
    pub xyo: i32,
    pub yxo: i32,
    pub yyo: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub tiles: ConfigPair,
    pub defaults: Defaults,
    pub renderer: Renderer,
}

impl Config {
    pub fn from_file(path: PathBuf) -> Result<Self> {
        let file = std::fs::read_to_string(path)?;
        let config: Self = serde_yaml::from_str(&file)?;
        Ok(config)
    }
}
