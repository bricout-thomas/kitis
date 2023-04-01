#[cfg(test)]
mod tests;

pub mod state;
use std::sync::Mutex;

pub use state::State;

pub mod map;
pub mod display;
pub mod components;
pub mod debug;
pub mod drone;
pub mod entities;
pub mod simulate;
pub mod utils;

pub const SCREEN_HEIGHT: i32 = 80;
pub const SCREEN_WIDTH: i32 = 80;
pub const CHUNK_SIZE: usize = 16;

static mut ID_COUNTER: Mutex<u32> = Mutex::new(0);
