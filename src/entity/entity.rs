use crate::stridert::Stridert;
use std::sync::{Arc, Mutex};
use crate::util::uuid;
use crate::registry::entitytypes::EntityType;

pub struct Entity {
	id: i32,
	uuid: uuid::UUID,
	entity_type: EntityType,
	position: (f64, f64, f64, f32, f32)
}

impl Entity {
	pub fn new(entity_type: EntityType, server: Arc<Mutex<Stridert>>, x: f64, y: f64, z: f64, yaw: f32, pitch: f32) -> Entity {
		return Entity {
			id: server.lock().unwrap().free_eid(),
			uuid: uuid::UUID::random(),
			entity_type,
			position: (x, y, z, yaw, pitch)
		}
	}
	pub fn get_x(&self) -> f64 { self.position.0 }
	pub fn get_y(&self) -> f64 { self.position.1 }
	pub fn get_z(&self) -> f64 { self.position.2 }
	pub fn get_yaw(&self) -> f32 { self.position.3 }
	pub fn get_pitch(&self) -> f32 { self.position.4 }
	pub fn get_id(&self) -> i32 { self.id }
	pub fn get_type(&self) -> EntityType { self.entity_type }
	pub fn get_uuid(&self) -> &uuid::UUID { &self.uuid }
}