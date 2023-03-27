use bevy_math::IVec2;
use bracket_terminal::prelude::*;
use bevy_ecs::prelude::*;

use crate::{display::Camera, map::Map, debug::DebugMode};

pub struct State {
    pub seed: u64,
    pub camera: Camera,
    pub world: World,
    pub map: Map,
    pub debug_mode: DebugMode,
}

impl State {
    pub fn new(seed: u64, debug_mode: DebugMode) -> State {
        State {
            seed,
            camera: Camera::new(),
            world: World::new(),
            map: Map::new(seed), // todo: Generate map
            debug_mode,
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        
        // camera control
        if let Some(key) = ctx.key {
            self.camera.position += match key {
                VirtualKeyCode::Up =>       IVec2::NEG_Y,
                VirtualKeyCode::Down =>     IVec2::Y,
                VirtualKeyCode::Left =>     IVec2::NEG_X,
                VirtualKeyCode::Right =>    IVec2::X,
                _ =>                        IVec2::ZERO,
            }
        }

        // display the world
        self.camera.view(ctx, &mut self.map, &self.debug_mode);
    }
}