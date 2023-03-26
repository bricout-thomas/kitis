use bracket_terminal::prelude::*;
use bevy_ecs::prelude::*;

pub struct State {
    pub world: World,
}

impl State {
    pub fn new() -> State {
        State {
            world: World::new(),
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.print(0, 0, "Hello world");
    }
}