use bevy_math::IVec2;
use bracket_terminal::prelude::*;

use crate::{display::Camera, map::Map, debug::DebugMode, simulate::run_simulation_step};

pub struct State {
    pub seed: u64,
    pub camera: Camera,
    pub map: Map,
    pub debug_mode: DebugMode,
    pub sim_distance: i32,
    pub sim_step: u64,
}

impl State {
    pub fn new(seed: u64, debug_mode: DebugMode, sim_distance: i32) -> State {
        State {
            seed,
            camera: Camera::new(),
            map: Map::new(seed), // todo: Generate map
            debug_mode,
            sim_distance,
            sim_step: 1,
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        
        // camera control
        let mut camera_position_changed: bool = true; // avoids having to put = true in all the branches
        if let Some(key) = ctx.key {
            self.camera.position += match key {
                VirtualKeyCode::Up =>       IVec2::NEG_Y,
                VirtualKeyCode::Down =>     IVec2::Y,
                VirtualKeyCode::Left =>     IVec2::NEG_X,
                VirtualKeyCode::Right =>    IVec2::X,
                _ =>                        { camera_position_changed = false; IVec2::ZERO},
            }
        }
        // update sim_step
        self.sim_step += 1;

        // run the simulation
        run_simulation_step(&mut self.map, &self.camera, self.sim_distance, self.sim_step);
        // display the world
        self.camera.view(ctx, &mut self.map, &self.debug_mode, camera_position_changed, self.sim_step);
    }
}