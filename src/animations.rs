use bevy_math::IVec2;
use bracket_terminal::prelude::*;

trait AnimatedLoop {
    fn display(&mut self, proj_coord: IVec2, ctx: &mut BTerm);
}

struct AnimateDrone { step: u8 }

impl AnimatedLoop for AnimateDrone {
    fn display(&mut self, proj_coord: IVec2, ctx: &mut BTerm) {
        let (x, y) = proj_coord.try_into().unwrap();
        self.step = self.step.wrapping_add(2);
        let palm_char = match self.step {
            0..=63 => '-',
            64..=127 => '\\',
            128..=161 => '|',
            162..=255 => '/',
        };
        ctx.print(x, y, 'x');
        ctx.print(x-1, y-1, palm_char);
        ctx.print(x+1, y-1, palm_char);
        ctx.print(x+1, y+1, palm_char);
        ctx.print(x-1, y+1, palm_char);
    }
}