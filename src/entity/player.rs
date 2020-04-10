use crate::network::connection::Connection;
use crate::entity::entity::Entity;
use crate::registry::entitytypes::EntityType;
use crate::registry::gamemodes::GameMode;
use crate::util::uuid;
use crate::entity::property::Property;
use crate::world::world::World;
use serde::Deserialize;
use reqwest::blocking::get;

use std::sync::{Arc, Mutex};

#[derive(Deserialize)]
struct UuidResponse {
	id: String,
	name: String
}

#[derive(Deserialize)]
struct PropertiesResponse {
	id: String,
	name: String,
	properties: Vec<Property>
}

pub struct Player {
	name: String,
pub connection: Arc<Mutex<Connection>>,
	entity: Entity,
	gamemode: GameMode,
	uuid: uuid::UUID,
	view_distance: u8,
	tracked_chunks: Vec<(i32, i32)>,
	properties: Vec<Property>
}

impl Player {
	pub fn new(name: String, connection: Connection) -> Player {
		let entity = Entity::new(EntityType::Player, 0.0, 0.0, 0.0, 0.0, 0.0);
		let conn = Arc::new(Mutex::new(connection));

		let mut res = get(&format!("https://api.mojang.com/users/profiles/minecraft/{}", name)).unwrap();
		let uuid = if res.status().is_success() {
			match res.json::<UuidResponse>() {
				Ok(json) => uuid::UUID::new(u64::from_str_radix(&json.id[..16], 16).unwrap(), u64::from_str_radix(&json.id[16..], 16).unwrap()),
				_ => uuid::UUID::random()
			}
		} else { uuid::UUID::random() };
		res = get(&format!("https://sessionserver.mojang.com/session/minecraft/profile/{}?unsigned=false", uuid.to_string().replace("-", ""))).unwrap();
		let properties = if res.status().is_success() {
			match res.json::<PropertiesResponse>() {
				Ok(json) => json.properties,
				_ => Vec::new()
			}
		} else { Vec::new() };

		return Player {
			name,
			connection: conn.clone(),
			entity,
			gamemode: GameMode::CREATIVE,
			uuid,
			view_distance: (*crate::SERVER).lock().unwrap().get_view_distance(),
			tracked_chunks: Vec::new(),
			properties
		}
	}
	pub fn get_name(&self) -> String { self.name.clone() }
	pub fn get_entity(&self) -> &Entity { &self.entity }
	pub fn get_id(&self) -> i32 { self.entity.get_id() }
	pub fn get_gamemode(&self) -> GameMode { self.gamemode }
	pub fn get_uuid(&self) -> &uuid::UUID { &self.uuid }
	pub fn set_view_distance(&mut self, view_distance: u8) { self.view_distance = view_distance }
	pub fn get_view_distance(&self) -> u8 { self.view_distance }
	pub fn get_properties(&self) -> &Vec<Property> { &self.properties }
	pub fn get_world(&self) -> Option<Arc<Mutex<World>>> {
		for world in (*crate::SERVER).lock().unwrap().get_worlds() {
			for p in world.lock().unwrap().get_players() {
				if &*p.lock().unwrap() == self {
					return Some(world.clone());
				}
			}
		}
		return None;
	}
}

impl PartialEq<Player> for Player {
	fn eq(&self, other: &Player) -> bool {
		return self.name == other.name
	}
}

impl std::ops::Deref for Player {
    type Target = Entity;
    fn deref(&self) -> &Entity {
        &self.entity
    }
}