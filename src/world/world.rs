use crate::world::chunk::Chunk;
use crate::entity::entity::Entity;
use crate::entity::player::Player;
use crate::registry::{dimensions::Dimension, leveltypes::LevelType, difficulties::Difficulty};
use std::sync::{Arc, Mutex};

pub struct World {
	name       : String,
	dimension  : Dimension,
	seed       : i64,
	level_type : LevelType,
	hardcore   : bool,
	chunks     : Vec<Arc<Mutex<Chunk>>>,
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
			level_type : LevelType::DEFAULT,
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
}