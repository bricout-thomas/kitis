use std::{collections::HashMap, default};

use bevy_ecs::prelude::*;
use bevy_math::IVec2;

use crate::CHUNK_SIZE;

#[derive(Default, Resource)]
pub struct Map {
    pub chunks: HashMap<IVec2, Chunk>,
}

pub struct Chunk {
    pub tiles: [[Tile; CHUNK_SIZE]; CHUNK_SIZE],
    pub entities: Vec<Entity>,
}

impl Chunk {
    pub fn empty() -> Chunk {
        Chunk {
            tiles: [[Tile::default(); 16]; 16],
            entities: Vec::new(),
        }
    }
}

#[derive(Default, Copy, Clone)]
pub struct Tile {
    
}

impl Tile {
    pub fn ascii(&self) -> char {
        '0'
    }
}