use bevy_math::prelude::*;
use bracket_terminal::prelude::*;

use crate::{SCREEN_HEIGHT, SCREEN_WIDTH, CHUNK_SIZE, map::{Map, Chunk}};

// Camera should be placed directly above target
pub struct Camera {
    pub position: IVec2,
}

struct IterOverChunks {
    left_most: i32,
    current: IVec2,
    last: IVec2,
}

impl From<&Camera> for IterOverChunks {
    fn from(item: &Camera) -> IterOverChunks {
        let left_most = (item.position.x - SCREEN_WIDTH) / CHUNK_SIZE as i32;
        IterOverChunks {
            left_most,
            current: IVec2::new(left_most - 1, (item.position.y - SCREEN_HEIGHT) / CHUNK_SIZE as i32),
            last: IVec2::new(item.position.x + SCREEN_WIDTH / 2 + 1, (item.position.y + SCREEN_WIDTH) / CHUNK_SIZE as i32),
        }
    }
}

impl Iterator for IterOverChunks {
    type Item = IVec2;
    fn next(&mut self) -> Option<IVec2> {
        if self.current == self.last { return None; }
        if self.current.x == self.last.x {
            self.current = IVec2::new(self.left_most, self.current.y + 1)
        } else {
            self.current += IVec2::new(1, 0);
        }
        Some(self.current)
    }
}

impl Camera {
    pub fn new() -> Self {
        Self {
            position: IVec2::ZERO,
        }
    }

    pub fn view(&self, ctx: &mut BTerm, map: &mut Map) {
        'chunk: for chunk_coord in IterOverChunks::from(self) {
            // get access to the chunk or create it
            // creation to be handled somewhere else
            let chunk = match map.chunks.get(&chunk_coord) {
                Some(chunk) => { chunk }
                None => {
                    let new_chunk = Chunk::grass(&map.grass_color_gen, chunk_coord);
                    map.chunks.insert(chunk_coord, new_chunk);
                    map.chunks.get(&chunk_coord).unwrap()
                }
            };

            // display tiles
            for (loc_y, line) in chunk.tiles.iter().enumerate() {
                for (loc_x, tile) in line.iter().enumerate() {
                    let proj_x = chunk_coord.x * CHUNK_SIZE as i32 - self.position.x + loc_x as i32;
                    let proj_y = chunk_coord.y * CHUNK_SIZE as i32 - self.position.y + loc_y as i32;
                    if proj_x < 0 || proj_y < 0 { continue; } // avoids crash from calling into::<usize> on neg value
                    tile.render.render(proj_x, proj_y, ctx);
                }
            }

        }
    }
}