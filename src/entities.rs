use std::{rc::Rc, cell::{RefCell, RefMut}, collections::HashSet};

use bevy_math::{Vec2, IVec2};
use std::hash::Hash;
use bracket_terminal::prelude::BTerm;

use crate::{drone::Drone, ID_COUNTER, map::map_position_to_chunk};

pub trait Entity {
    fn display(&mut self, ctx: &mut BTerm, camera_position: Vec2);
    fn everything_solo(&mut self) -> EntityStatus;
    // return the new chunk_coord if the entity moved so that it can be moved
    // doesn't require any outside world info
}

pub struct EntityWrapper {
    pub entity: Box<dyn Entity>,
    pub last_seen: u64,
    pub last_updated: u64,

    chunks: HashSet<IVec2>,
    pub updated_chunks: bool,
}

impl EntityWrapper {
    pub fn new_drone(position: Vec2, sim_step: u64) -> EntityWrapper {
        let mut chunks = HashSet::with_capacity(1);
        chunks.insert(map_position_to_chunk(position));

        EntityWrapper {
            entity: Drone::new(position),
            last_seen: sim_step-1,
            last_updated: sim_step-1,

            chunks,
            updated_chunks: true,
        }
    }
    pub fn in_chunk(&self, chunk_coord: &IVec2) -> bool {
        self.chunks.contains(chunk_coord)
    }

}

pub enum EntityStatus {
    Nothing,
    RedrawBg,
    UpdateSpatialPartitioning {
        remove_from: Vec<IVec2>,
        add_to: Vec<IVec2>,
    },
}

impl Eq for EntityRef {
    fn assert_receiver_is_total_eq(&self) {
        // IDK what that's supposed to do like crash?
    }
}

#[derive(Clone)]
pub struct EntityRef {
    pub entity_wrapper: Rc<RefCell<EntityWrapper>>,
    pub id: u32,
}

impl EntityRef {
    pub fn new(entity_wrapper: EntityWrapper) -> Self {
        let id;
        let entity_wrapper = Rc::new( RefCell::new( entity_wrapper));
        unsafe { // not unsafe thanks to Mutex
            let mut id_counter = ID_COUNTER.lock().unwrap();
            *id_counter += 1;
            id = *id_counter;
        }
        Self {
            entity_wrapper,
            id,
        }
    }

    pub fn access_wrapper(&self) -> RefMut<EntityWrapper> {
        self.entity_wrapper.as_ref().borrow_mut()
    }
}

impl PartialEq for EntityRef {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Hash for EntityRef {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}