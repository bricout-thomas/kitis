#[cfg(test)]
mod tests;

pub mod state;
pub use state::State;

pub mod map;
pub mod display;

pub const SCREEN_HEIGHT: i32 = 80;
pub const SCREEN_WIDTH: i32 = 80;
pub const CHUNK_SIZE: usize = 16;
