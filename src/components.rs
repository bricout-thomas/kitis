use bevy_math::*;
use bracket_terminal::prelude::*;

pub struct Position {
    coord: Vec2,
}

#[derive(Default, Copy, Clone)]
pub struct TileRenderable {
    glyph: FontCharType,
    pub bg: RGBA,
    fg: RGBA,
}

impl TileRenderable {
    pub fn render(&self, x: i32, y: i32, ctx: &mut BTerm) {
        ctx.set(x, y, self.fg, self.bg, self.glyph);
    } 
}