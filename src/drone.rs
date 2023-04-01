use bevy_math::Vec2;
use bracket_terminal::prelude::*;
use rand::random;

use crate::{entities::{Entity, EntityStatus}, utils::chunk_location};

pub struct Drone {
    animation_step: u8, 
    position: Vec2,
    velocity: Vec2,
}

impl Drone {
    pub fn new(position: Vec2) -> Box<dyn Entity> {
        Box::new( Self {
            position,
            animation_step: random(),
            velocity: Vec2::new(random::<f32>()-0.5, random::<f32>()-0.5)
        })
    }
}

impl Entity for Drone {
    fn display(&mut self, ctx: &mut BTerm, camera_position: Vec2) {
        let (x, y) = (self.position - camera_position).try_into().unwrap();
        let x = x as i32; let y = y as i32;
        self.animation_step = self.animation_step.wrapping_add(16);
        let palm_char = match self.animation_step {
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

    fn everything_solo(&mut self) -> EntityStatus {
        let former_location = chunk_location(self.position);
        self.position += self.velocity;
        let new_location = chunk_location(self.position);
        if former_location != new_location {
            EntityStatus::UpdateSpatialPartitioning { 
                remove_from: vec!(former_location), 
                add_to: vec!(new_location), 
            }
        } else {
            EntityStatus::RedrawBg
        }
    }
}