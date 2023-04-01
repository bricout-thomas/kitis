use bevy_math::{Vec2, IVec2};

pub fn chunk_location(position: Vec2) -> IVec2 {
    let (x, y) = position.try_into().unwrap();
    let x = x as i32 / 16;
    let y = y as i32 / 16;
    IVec2::new(x, y)
}
