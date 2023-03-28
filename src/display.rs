use bevy_math::prelude::*;
use bracket_terminal::prelude::*;

use crate::{SCREEN_HEIGHT, SCREEN_WIDTH, CHUNK_SIZE, map::{Map, Chunk}, debug::DebugMode};

// Camera should be placed directly above target
pub struct Camera {
    pub position: IVec2,
}

struct IterOverChunks {
    left_side: i32,
    current: IVec2,
    last: IVec2,
}

impl From<&Camera> for IterOverChunks {
    fn from(item: &Camera) -> IterOverChunks {
        let top_side = (item.position.y - SCREEN_HEIGHT) / CHUNK_SIZE as i32;
        let left_side = (item.position.x - SCREEN_WIDTH) / CHUNK_SIZE as i32;
        let bottom_side = (item.position.y + SCREEN_HEIGHT) / CHUNK_SIZE as i32;
        let right_side = (item.position.x + SCREEN_WIDTH) / CHUNK_SIZE as i32;

        // println!("top left: {}, {}", left_side, top_side);
        // println!("bottom right: {}, {}", right_side, bottom_side);
        
        IterOverChunks {
            left_side,
            current: IVec2::new(left_side - 1, top_side),
            last: IVec2::new(right_side, bottom_side),
        }
    }
}

impl Iterator for IterOverChunks {
    type Item = IVec2;
    fn next(&mut self) -> Option<IVec2> {
        if self.current == self.last { return None; }
        if self.current.x == self.last.x {
            self.current = IVec2::new(self.left_side, self.current.y + 1)
        } else {
            self.current += IVec2::X
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

    pub fn view(&self, ctx: &mut BTerm, map: &mut Map, debug_mode: &DebugMode, camera_position_changed: bool) {
        // set offset doesn't seem to work, but it is probably doing as intended
        // the documentation is unclear. instead I use - self.position
        // ctx.set_offset(self.position.x as f32, self.position.y as f32);
        for chunk_coord in IterOverChunks::from(self) {
            // get access to the chunk or create it
            // creation to be handled somewhere else
            let chunk = match map.chunks.get_mut(&chunk_coord) {
                Some(chunk) => { chunk }
                None => {
                    let new_chunk = Chunk::grass(&map.grass_color_gen, chunk_coord);
                    map.chunks.insert(chunk_coord, new_chunk);
                    map.chunks.get_mut(&chunk_coord).unwrap()
                }
            };

            // display tiles
            if camera_position_changed || chunk.updated_tiles {
                chunk.updated_tiles = false;
                let proj_chunk_coord = chunk_coord * CHUNK_SIZE as i32 - self.position;
                for (loc_y, line) in chunk.tiles.iter().enumerate() {
                    for (loc_x, tile) in line.iter().enumerate() {
                        let proj_x = proj_chunk_coord.x + loc_x as i32;
                        let proj_y = proj_chunk_coord.y + loc_y as i32;
                        if proj_x < 0 || proj_y < 0 { continue; } // avoids crash from calling into::<usize> on neg value
                        tile.render.render(proj_x, proj_y, ctx);
                        if debug_mode.display_chunk {
                            ctx.print(proj_chunk_coord.x, proj_chunk_coord.y, format!("X: {}, Y: {}", chunk_coord.x, chunk_coord.y));
                        }
                    }
                }
            }

            // display entities
            for entity in chunk.entities.iter() {

            }
        }
    }
}