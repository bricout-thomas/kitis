use std::{collections::HashMap};

use bevy_ecs::prelude::*;
use bevy_math::IVec2;
use bracket_color::rgba::RGBA;
use bracket_noise::prelude::{FastNoise, NoiseType};

use crate::{CHUNK_SIZE, components::TileRenderable};

#[derive(Resource)]
pub struct Map {
    pub chunks: HashMap<IVec2, Chunk>,
    pub grass_color_gen: FastNoise,
}

impl Map {
    pub fn new(seed: u64) -> Map { 
        let mut grass_color_gen = FastNoise::seeded(seed);
        grass_color_gen.set_frequency(0.05);
        grass_color_gen.set_fractal_octaves(400);
        grass_color_gen.set_noise_type(NoiseType::Value);
        Map {
            grass_color_gen,
            chunks: Default::default(),
        }
    }
}

pub struct Chunk {
    pub tiles: [[Tile; CHUNK_SIZE]; CHUNK_SIZE],
    pub entities: Vec<Entity>,
}

impl Chunk {
    // A completely void of anything chunk
    pub fn empty() -> Chunk {
        Chunk {
            tiles: [[Tile::default(); CHUNK_SIZE]; CHUNK_SIZE],
            entities: Vec::new(),
        }
    }
    // A chunk with grass colored thanks to noise
    pub fn grass(grass_color_gen: &FastNoise, chunk_coord: IVec2) -> Chunk {
        let mut tiles = [[Tile::default(); CHUNK_SIZE]; CHUNK_SIZE];
        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                tiles[y][x].render.bg = RGBA::from_f32(0., 
                    grass_color_gen.get_noise((chunk_coord.x * CHUNK_SIZE as i32 + x as i32) as f32, (chunk_coord.y * CHUNK_SIZE as i32 + y as i32) as f32)
                    , 0., 1.);
            }
        }

        Chunk {
            tiles,
            entities: Vec::new(),
        }
    }
}

#[derive(Default, Copy, Clone)]
pub struct Tile {
    pub render: TileRenderable,
}