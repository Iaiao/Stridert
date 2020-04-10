use crate::world::chunk::Chunk;
use crate::entity::entity::Entity;
use crate::entity::player::Player;
use crate::world::block::Block;
use crate::registry::blocks::BlockType;
use crate::registry::{dimensions::Dimension, leveltypes::LevelType, difficulties::Difficulty};
use std::sync::{Arc, Mutex};

pub struct World {
	name       : String,
	dimension  : Dimension,
	seed       : i64,
	level_type : LevelType,
	hardcore   : bool,
	chunks     : Vec<Chunk>,
	entities   : Vec<Entity>,
	players    : Vec<Arc<Mutex<Player>>>,
	difficulty : Difficulty
}

impl World {
	pub fn new(name: String, dimension: Dimension, seed: i64, difficulty: Difficulty) -> World {
		return World {
			name,
			dimension,
			seed,
			level_type : LevelType::Default,
			hardcore   : false,
			chunks     : Vec::new(),
			entities   : Vec::new(),
			players    : Vec::new(),
			difficulty
		}
	}
	pub fn add_player(&mut self, player: Arc<Mutex<Player>>) { self.players.push(player) }
	pub fn get_entities(&self) -> &Vec<Entity> { return &self.entities }
	pub fn get_players(&self) -> Vec<Arc<Mutex<Player>>> { return self.players.clone() }
	pub fn get_player_count(&self) -> usize { self.players.len() }
	pub fn is_hardcore(&self) -> bool { self.hardcore }
	pub fn get_type(&self) -> LevelType { self.level_type }
	pub fn get_dimension(&self) -> Dimension { self.dimension }
	pub fn get_seed(&self) -> i64 { self.seed }
	pub fn get_difficulty(&self) -> Difficulty { self.difficulty }
	pub fn get_player<S: AsRef<str>>(&self, name: S) -> Option<Arc<Mutex<Player>>> {
		let name = name.as_ref();
		for player in &self.players {
			if player.lock().unwrap().get_name() == name {
				return Some(player.clone())
			}
		}
		return None
	}
	pub fn get_loaded_chunks(&self) -> &Vec<Chunk> { &self.chunks }
	pub fn get_chunk(&mut self, x: i32, z: i32) -> &Chunk {
		if self.get_loaded_chunks().iter().any(|ch| ch.get_x() == x && ch.get_z() == z) {
			return self.get_loaded_chunks().iter().filter(|ch| ch.get_x() == x && ch.get_z() == z).collect::<Vec<&Chunk>>()[0];
		} else {
			// Генерация или загрузка чанка
			self.chunks.push(Chunk::new(x, z, [Block::new(BlockType::Air); 65536]));
			return self.get_chunk(x, z);
		}
	}
}