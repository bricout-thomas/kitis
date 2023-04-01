use std::collections::{HashMap, HashSet};

use bevy_math::{IVec2, Vec2};
use bracket_color::rgba::RGBA;
use bracket_noise::prelude::{FastNoise, NoiseType};
use rand::random;

use crate::{CHUNK_SIZE, components::TileRenderable, entities::{EntityWrapper, EntityRef}};

pub struct Map {
    pub chunks: HashMap<IVec2, Chunk>,
    pub grass_color_gen: FastNoise,
}

impl Map {
    pub fn new(seed: u64) -> Map { 
        let mut grass_color_gen = FastNoise::seeded(seed);
        grass_color_gen.set_frequency(0.05);
        grass_color_gen.set_fractal_octaves(2);
        grass_color_gen.set_noise_type(NoiseType::Simplex);
        Map {
            grass_color_gen,
            chunks: HashMap::with_capacity(200),
        }
    }

    // pub fn access_chunk_mut<'a>(&'a mut self, chunk_coord: IVec2, sim_step: u64) -> &'a mut Chunk {
    //     match self.chunks.get_mut(&chunk_coord) {
    //         Some(chunk) => { return chunk },
    //         None => {
    //         }
    //     }
    //     let new_chunk = Chunk::grass(&self.grass_color_gen, chunk_coord, sim_step);
    //     self.chunks.insert(chunk_coord, new_chunk);
    //     self.chunks.get_mut(&chunk_coord).unwrap()
    // }
}

pub struct Chunk {
    pub updated_tiles: bool,
    pub tiles: [[Tile; CHUNK_SIZE]; CHUNK_SIZE],
    pub entities: HashSet<EntityRef>,
}

impl Chunk {
    // A completely void of anything chunk
    pub fn empty() -> Chunk {
        Chunk {
            updated_tiles: true,
            tiles: [[Tile::default(); CHUNK_SIZE]; CHUNK_SIZE],
            entities: HashSet::new(),
        }
    }
    // A chunk with grass colored thanks to noise
    pub fn grass(grass_color_gen: &FastNoise, chunk_coord: IVec2, sim_step: u64) -> Chunk {
        let mut tiles = [[Tile::default(); CHUNK_SIZE]; CHUNK_SIZE];
        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                tiles[y][x].render.bg = RGBA::from_f32(0., 
                    grass_color_gen.get_noise((chunk_coord.x * CHUNK_SIZE as i32 + x as i32) as f32, (chunk_coord.y * CHUNK_SIZE as i32 + y as i32) as f32) / 3. + 0.7
                    , 0., 1.);
            }
        }

        let mut entities: HashSet<EntityRef> = HashSet::new();

        if rand::random::<u8>() >= 128 {
            entities.insert(
                EntityRef::new(EntityWrapper::new_drone(random_Vec2_inside_chunk(chunk_coord), sim_step))
            );
        } 

        Chunk {
            updated_tiles: true,
            tiles,
            entities,
        }
    }
}

#[derive(Default, Copy, Clone)]
pub struct Tile {
    pub render: TileRenderable,
}

#[allow(non_snake_case)]
fn random_Vec2_inside_chunk(chunk_coord: IVec2) -> Vec2 {
    let topleft: IVec2 = chunk_coord * CHUNK_SIZE as i32;
    let topleft: Vec2 = Vec2::new(topleft.x as f32, topleft.y as f32);
    let random_small_vec = Vec2::new(random::<f32>() * CHUNK_SIZE as f32, random::<f32>() * CHUNK_SIZE as f32);
    // println!("random_small_vec = {:?}", random_small_vec);
    topleft + random_small_vec
}

pub fn map_position_to_chunk(position: Vec2) -> IVec2 {
    let position = IVec2::new(position.x as i32, position.y as i32);
    let chunk = position / CHUNK_SIZE as i32;
    chunk
}