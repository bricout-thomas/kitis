use bracket_terminal::prelude::*;
use bevy_ecs::prelude::*;
use rand;

use crate::{display::Camera, map::Map};

pub struct State {
    pub seed: u32,
    pub camera: Camera,
    pub world: World,
    pub map: Map,
}

impl State {
    pub fn new() -> State {
        State {
            seed: rand::random(),
            camera: Camera::new(),
            world: World::new(),
            map: Map::default(), // todo: Generate map
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        self.camera.view(ctx, &mut self.map);
    }
}