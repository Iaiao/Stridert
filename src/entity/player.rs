use crate::network::connection::Connection;
use crate::entity::entity::Entity;
use crate::registry::entitytypes::EntityType;
use crate::registry::gamemodes::GameMode;
use std::sync::{Arc, Mutex};

pub struct Player {
	name: String,
	pub connection: Arc<Mutex<Connection>>,
	entity: Entity,
	gamemode: GameMode,
	view_distance: u8,
	tracked_chunks: Vec<(i32, i32)>
}

impl Player {
	pub fn new(name: String, connection: Connection) -> Player {
		let entity = Entity::new(EntityType::Player, connection.server.clone(), 0.0, 0.0, 0.0);
		let conn = Arc::new(Mutex::new(connection));
		return Player {
			name,
			connection: conn.clone(),
			entity,
			gamemode: GameMode::CREATIVE,
			view_distance: conn.clone().lock().unwrap().server.lock().unwrap().get_view_distance(),
			tracked_chunks: Vec::new()
		}
	}
	pub fn get_name(&self) -> String { self.name.clone() }
	pub fn get_entity(&self) -> &Entity { &self.entity }
	pub fn get_id(&self) -> i32 { self.entity.get_id() }
	pub fn get_gamemode(&self) -> GameMode { self.gamemode }
	pub fn set_view_distance(&mut self, view_distance: u8) { self.view_distance = view_distance }
	pub fn get_view_distance(&self) -> u8 { self.view_distance }
}

impl PartialEq<Player> for Player {
	fn eq(&self, other: &Player) -> bool {
		return self.name == other.name
	}
}