use std::collections::HashSet;
use bevy_math::IVec2;

use crate::{map::{Map, Chunk}, display::{Camera, IterOverChunks}, entities::{EntityStatus, EntityRef}};


pub fn run_simulation_step(map: &mut Map, camera: &Camera, sim_distance: i32, sim_step: u64) {
    let mut updated_chunks_entity_list = HashSet::<EntityRef>::new();

    // set every entity's updated false
    for chunk_coord in IterOverChunks::loaded_chunks(camera, sim_distance) {
        let chunk = match map.chunks.get_mut(&chunk_coord) {
            Some(chunk) => { chunk }
            None => { 
                let new_chunk = Chunk::grass(&map.grass_color_gen, chunk_coord, sim_step);
                map.chunks.insert(chunk_coord, new_chunk);
                map.chunks.get_mut(&chunk_coord).unwrap()
            }
        };
        for (i, entity_ptr) in chunk.entities.iter().enumerate() {
            let mut entity = entity_ptr.access_wrapper();

            if entity.updated_chunks {
                updated_chunks_entity_list.insert(entity_ptr.clone());
            }
        }
    }

    let mut spatial_updates = Vec::<SpatialPartitioningUpdate>::new();
    // update every entity

    for chunk_coord in IterOverChunks::loaded_chunks(camera, sim_distance) {
        let chunk = match map.chunks.get_mut(&chunk_coord) {
            Some(chunk) => { chunk }
            None => { unreachable!() }
        };
        
        for entity_ref in chunk.entities.iter() {
            let mut entity = entity_ref.access_wrapper();
            if entity.last_updated < sim_step {
                let status = entity.entity.everything_solo();
                entity.last_updated = sim_step;

                match status {
                    EntityStatus::Nothing => {},
                    EntityStatus::RedrawBg => { chunk.updated_tiles = true; },
                    EntityStatus::UpdateSpatialPartitioning { remove_from, add_to }
                        => { spatial_updates.push( SpatialPartitioningUpdate { 
                            entity: entity_ref.clone(),
                            remove_from, 
                            add_to,
                        }) },
                }
            }
        }
    }

    for spatial_update in spatial_updates {
        
    }
}

struct SpatialPartitioningUpdate {
    entity: EntityRef,
    remove_from: Vec<IVec2>,
    add_to: Vec<IVec2>,
}