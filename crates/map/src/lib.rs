use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std_utils::{Error, Result};

#[derive(Serialize, Deserialize, Debug)]
pub struct Rect {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
    pub o: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Tile {
    pub id: u32,
    pub tag: String,
    pub rect: Rect,
    pub solid: bool,
    pub transparent: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Tiles {
    tiles: Vec<Tile>,
}

impl Tiles {
    pub fn from_file(path: PathBuf) -> Result<Self> {
        let file = std::fs::read_to_string(path)?;
        let tiles: Vec<Tile> = serde_yaml::from_str(&file)?;
        Ok(Self { tiles })
    }

    pub fn len(&self) -> usize {
        self.tiles.len()
    }

    pub fn iter(&self) -> std::slice::Iter<Tile> {
        self.tiles.iter()
    }

    pub fn get(&self, index: u32) -> Result<&Tile> {
        if index >= self.tiles.len() as u32 {
            return Err(Error::msg("Invalid tile index"));
        }
        Ok(&self.tiles[index as usize])
    }
}

pub struct Layer<'a> {
    tiles: &'a Tiles,
    pub data: Vec<u32>,
    pub width: u32,
    pub height: u32,
}

impl<'a> Layer<'a> {
    pub fn new(tiles: &'a Tiles, width: u32, height: u32) -> Self {
        let mut data: Vec<u32> = Vec::new();
        data.resize((width * height) as usize, 0);
        Self {
            tiles,
            data,
            width,
            height,
        }
    }

    pub fn healh_check(&self) -> Result<()> {
        // Check if the layer size is valid
        if self.tiles.len() != (self.width * self.height) as usize {
            return Err(Error::msg("Invalid layer size"));
        }

        // Check if the tile indexes are valid
        for (i, tile) in self.data.iter().enumerate() {
            if *tile as usize >= self.tiles.len() {
                return Err(Error::msg(format!(
                    "Invalid tile index \"{}\" at \"{}x\", \"{}y\"",
                    *tile,
                    (i % self.width as usize),
                    (i / self.width as usize)
                )));
            }
        }

        Ok(())
    }
}
