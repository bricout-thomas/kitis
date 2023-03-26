use bracket_terminal::prelude::*;

struct State {
    
}

impl State {
    pub fn new() -> State {
        State {
            
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.print(0, 0, "Hello world");
    }
}