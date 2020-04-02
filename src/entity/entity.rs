use crate::stridert::Stridert;
use std::sync::{Arc, Mutex};
use crate::util::uuid;

pub struct Entity {
	id: i32,
	uuid: uuid::UUID,
	entity_type: EntityType,
	position: (f64, f64, f64)
}

#[derive(Copy, Clone)]
pub enum EntityType {
	PLAYER
}

impl Entity {
	pub fn new(entity_type: EntityType, server: Arc<Mutex<Stridert>>, x: f64, y: f64, z: f64) -> Entity {
		return Entity {
			id: server.lock().unwrap().free_eid(),
			uuid: uuid::UUID::random(),
			entity_type,
			position: (x, y, z)
		}
	}
	pub fn get_id(&self) -> i32 { self.id }
	pub fn get_type(&self) -> EntityType { self.entity_type }
	pub fn get_uuid(&self) -> &uuid::UUID { &self.uuid }
}