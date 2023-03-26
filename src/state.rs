use bevy_math::IVec2;
use bracket_terminal::prelude::*;
use bevy_ecs::prelude::*;
use rand;

use crate::{display::Camera, map::Map};

pub struct State {
    pub seed: u64,
    pub camera: Camera,
    pub world: World,
    pub map: Map,
}

impl State {
    pub fn new(seed: u64) -> State {
        State {
            seed,
            camera: Camera::new(),
            world: World::new(),
            map: Map::new(seed), // todo: Generate map
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
        self.camera.view(ctx, &mut self.map);
    }
}