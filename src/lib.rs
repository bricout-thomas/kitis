#[cfg(test)]
mod tests;

pub mod state;
pub use state::State;

pub mod map;
pub mod display;
pub mod components;
pub mod debug;
pub mod animations;
pub mod entities;

pub const SCREEN_HEIGHT: i32 = 80;
pub const SCREEN_WIDTH: i32 = 80;
pub const CHUNK_SIZE: usize = 16;
