use crate::prelude::*;
const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
}

pub struct Map {
    pub tiles: Vec<TileType>,   
}

pub fn map_idx(x: i32, y: i32) -> usize {
    (y * SCREEN_WIDTH + x) as usize
}

impl Map {
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES],
        }
    }
    pub fn render(&self, ctx: &mut BTerm) {
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                let idx = map_idx(x, y);
                match self.tiles[idx] {
                    TileType::Floor => {
                        ctx.set(x, y, RGB::from_f32(0.5, 0.5, 0.5), RGB::from_f32(0., 0., 0.), to_cp437('.'));
                    }
                    TileType::Wall => {
                        ctx.set(x, y, RGB::from_f32(0., 1.0, 0.), RGB::from_f32(0., 0., 0.), to_cp437('#'));
                    }
                }
            }
        }
    }
}