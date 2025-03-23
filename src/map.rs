use crate::config::Renderer;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std_utils::{errors::*, Error, Result};

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

    pub fn get(&self, index: u32) -> Result<&Tile> {
        if index >= self.tiles.len() as u32 {
            return Err(Error::msg("Invalid tile index"));
        }
        Ok(&self.tiles[index as usize])
    }
}

pub struct Layer<'a> {
    tiles: &'a Tiles,
    pub z: u32,
    pub data: Vec<u32>,
    pub width: u32,
    pub height: u32,
}

impl<'a> Layer<'a> {
    pub fn new(tiles: &'a Tiles, z: u32, width: u32, height: u32) -> Self {
        let mut data: Vec<u32> = Vec::new();
        data.resize((width * height) as usize, 0);
        Self {
            tiles,
            z,
            data,
            width,
            height,
        }
    }

    pub fn healh_check(&self) -> Result<()> {
        // Check if the layer size is valid
        if self.data.len() != (self.width * self.height) as usize {
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

    pub fn pattern(&mut self) {
        if self.tiles.len() < 2 {
            return;
        }

        for i in 0..self.data.len() {
            let x = i as i32 % self.width as i32;
            let y = i as i32 / self.width as i32;
            if x < self.width as i32 - self.z as i32 && y < self.height as i32 - self.z as i32 {
                let tile = (i as u32 % (self.tiles.len() as i32 - 2 as i32) as u32) + 2;
                self.data[i] = tile;
            } else {
                self.data[i] = 1;
            }
        }
    }

    pub fn draw(
        &self,
        texture: &sdl2::render::Texture,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
        x_offset: i32,
        y_offset: i32,
        config: &Renderer,
    ) -> Result<()> {
        for (i, tile) in self.data.iter().enumerate() {
            let tile = self.tiles.get(*tile)?;
            let x = i as i32 % self.width as i32;
            let y = i as i32 / self.width as i32;
            let rect = sdl2::rect::Rect::new(
                tile.rect.x as i32 - tile.rect.o as i32,
                tile.rect.y as i32,
                tile.rect.w as u32,
                tile.rect.h as u32,
            );
            let x = x * config.zoom as i32;
            let y = y * config.zoom as i32;
            let dest = sdl2::rect::Rect::new(
                x * tile.rect.w as i32 - (x * config.xxo) - (y * config.xyo) + x_offset,
                y * tile.rect.h as i32 + (x * config.yxo) - (y * config.yyo) + y_offset,
                tile.rect.w * config.zoom,
                tile.rect.h * config.zoom,
            );
            canvas.copy(texture, rect, dest).anyhow()?;
        }
        Ok(())
    }
}
