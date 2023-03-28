use bevy_math::{Vec2, IVec2};
use bracket_terminal::prelude::BTerm;

pub trait Entity {
    fn display(&mut self, ctx: &mut BTerm, camera_position: Vec2);
    fn everything_else(&mut self) -> Option<IVec2>;
    // return the new chunk_coord if the entity moved so that it can be moved
}