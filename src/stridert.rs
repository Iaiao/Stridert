use crate::network::packets::packet::ClientboundPacket;
use crate::entity::player::Player;
use crate::world::world::World;
use crate::config;
use crate::registry::{dimensions::Dimension, difficulties::Difficulty, identifier::Identifier};
use crate::network::{self, packets};
use crate::inventory::recipe::Recipe;
use crate::registry::recipes;
use crate::registry::entitystatuses;
use std::fs;
use std::sync::{Arc, Mutex};

pub struct Stridert {
	mod_name: String,
	version: String,
	protocol: i32,
	icon: String,
	worlds: Vec<Arc<Mutex<World>>>,
	max_players: usize,
	view_distance: u8,
	recipes: Vec<Recipe>
}

impl Stridert {
	pub fn new() -> Stridert {
		let buf = fs::read("server-icon.png");
		let icon = format!("data:image/png;base64,{}", base64::encode(&buf.as_ref().unwrap()));
		let mut server = Stridert {
			mod_name: String::from("stridert"),
			version: String::from(config::VERSION),
			protocol: 709,
			icon,
			worlds: vec!(
				Arc::new(Mutex::new(World::new(
					String::from("world"),
					Dimension::Overworld,
					0,
					Difficulty::Easy
				))),
				Arc::new(Mutex::new(World::new(
					String::from("world"),
					Dimension::Nether,
					0,
					Difficulty::Easy
				))),
				Arc::new(Mutex::new(World::new(
					String::from("world"),
					Dimension::TheEnd,
					0,
					Difficulty::Easy
				)))
			),
			max_players: config::MAX_PLAYERS,
			view_distance: config::VIEW_DISTANCE,
			recipes: Vec::new()
		};
		recipes::fill_recipes(&mut server.recipes);
		return server;
	}
	pub fn get_worlds(&self) -> &Vec<Arc<Mutex<World>>> { &self.worlds }
	pub fn get_mod_name(&self) -> String { self.mod_name.clone() }
	pub fn get_version(&self) -> String { self.version.clone() }
	pub fn get_protocol(&self) -> i32 { self.protocol }
	pub fn get_view_distance(&self) -> u8 { self.view_distance }
	pub fn get_player_count(&self) -> usize {
		let mut players = 0;
		for world in self.get_worlds() {
			players += world.lock().unwrap().get_player_count()
		}
		return players;
	}
	pub fn get_max_players(&self) -> usize { self.max_players }
	pub fn get_icon(&self) -> String { self.icon.clone() }
	pub fn add_player(&mut self, player: Player) {
		let player = Arc::new(Mutex::new(player));
		{
			let p = player.lock().unwrap();
			let mut conn = p.connection.lock().unwrap();
			let world = self.worlds[0].lock().unwrap();
			conn.send(&packets::clientboundloginsuccesspacket::ClientboundLoginSuccessPacket::new(
				p.get_name(),
				p.get_uuid()
			));
			println!("[+] {} присоединился к игре.", p.get_name());
			conn.send(&packets::clientboundjoingamepacket::ClientboundJoinGamePacket::new(
				p.get_id(),
				p.get_gamemode(),
				world.is_hardcore(),
				world.get_dimension(),
				world.get_seed(),
				self.max_players as u8,
				world.get_type(),
				self.view_distance as i32,
				false,
				true
			));
			conn.send(&packets::clientboundpluginmessagepacket::ClientboundPluginMessagePacket::new(Identifier::new(String::from("minecraft"), String::from("brand")), self.get_mod_name().into_bytes()));
			conn.send(&packets::clientboundserverdifficultypacket::ClientboundServerDifficultyPacket::new(world.get_difficulty(), true));
			conn.send(&packets::clientboundplayerabilitiespacket::ClientboundPlayerAbilitiesPacket::new(true, true, true, true, 0.05, 0.1));

			conn.send(&packets::clientboundhelditemchangepacket::ClientboundHeldItemChangePacket::new(0));
			conn.send(&packets::clientbounddeclarerecipespacket::ClientboundDeclareRecipesPacket::new(self.get_recipes()));
			conn.send(&packets::clientboundtagspacket::ClientboundTagsPacket::new());
			let entity_id = p.get_entity().get_id();
			conn.send(&packets::clientboundentitystatuspacket::ClientboundEntityStatusPacket::new(entity_id, entitystatuses::player::OP_PERMISSION_LEVEL_4));
			conn.send(&packets::clientbounddeclarecommandspacket::ClientboundDeclareCommandsPacket::new());
			let recipes = self.get_recipes();
			conn.send(&packets::clientboundunlockrecipespacket::ClientboundUnlockRecipesPacket::new(
				packets::clientboundunlockrecipespacket::Action::Init,
				true,
				true,
				true,
				true,
				recipes.clone(),
				Option::from(recipes)
			));
			conn.send(&packets::clientboundplayerpositionandlookpacket::ClientboundPlayerPositionPacket::new(
				p.get_x(),
				p.get_y(),
				p.get_z(),
				p.get_yaw(),
				p.get_pitch(),
				false,
				false,
				false,
				false,
				false
			));
		}

		let mut player_info_packet = packets::clientboundplayerinfopacket::ClientboundPlayerInfoPacket::new(
			packets::clientboundplayerinfopacket::Action::AddPlayer,
			self.get_players()
		);
		player_info_packet.add_player(&player.lock().unwrap());
		player.lock().unwrap().connection.lock().unwrap().send(&player_info_packet);
		
		self.broadcast_packet(&packets::clientboundplayerinfopacket::ClientboundPlayerInfoPacket::new(
			packets::clientboundplayerinfopacket::Action::AddPlayer,
			vec!(player.clone())
		));
		self.broadcast_packet(&packets::clientboundplayerinfopacket::ClientboundPlayerInfoPacket::new(
			packets::clientboundplayerinfopacket::Action::UpdateLatency,
			vec!(player.clone())
		));
		
		let p = player.lock().unwrap();
		let mut conn = p.connection.lock().unwrap();
		conn.send(&packets::clientboundupdateviewpositionpacket::ClientboundUpdateViewPositionPacket::new(p.get_x(), p.get_z()));
		let chunk_x = p.get_x() as i32 >> 4;
		let chunk_z = p.get_z() as i32 >> 4;
		for x in (chunk_x - p.get_view_distance() as i32)..(chunk_x - p.get_view_distance() as i32 + 1) {

			for z in (chunk_z - p.get_view_distance() as i32)..(chunk_z - p.get_view_distance() as i32 + 1) {
				conn.send(&packets::clientboundupdatelightpacket::ClientboundUpdateLightPacket::new(self.worlds[0].lock().unwrap().get_chunk(x, z), (0..17).collect(), (0..17).collect()))
			}
		}
		self.worlds[0].lock().unwrap().add_player(player.clone());
		network::packet_listener::start(player.clone());
	}
	pub fn free_eid(&self) -> i32 {
		let mut ids = Vec::new();
		for world in self.get_worlds() {
			ids.append(&mut world.lock().unwrap().get_entities().into_iter().map(|e| e.get_id()).collect());
		}
		let mut i = 0;
		loop {
			let arr: Vec<i32> = ((i*256 + 1)..(i*256 + 257)).collect();
			for id in arr {
				if !ids.contains(&id) {
					return id
				}
			}
			i += 1;
		}
	}
	pub fn get_player<S: AsRef<str>>(&self, name: S) -> Option<Arc<Mutex<Player>>> {
		let name = name.as_ref();
		for world in &self.worlds {
			match world.lock().unwrap().get_player(name) {
				Some(p) => return Some(p),
				_ => {}
			}
		}
		return None
	}
	pub fn get_players(&self) -> Vec<Arc<Mutex<Player>>> {
		let mut players = Vec::new();
		for world in &self.worlds {
			players.append(&mut world.lock().unwrap().get_players());
		}
		return players
	}
	pub fn get_recipes(&self) -> Vec<Recipe> { self.recipes.clone() }
	pub fn register_recipe(&mut self, recipe: Recipe) { self.recipes.push(recipe) }
	pub fn broadcast_packet(&self, packet: &dyn ClientboundPacket) {
		for player in self.get_players() {
			player.lock().unwrap().connection.lock().unwrap().send(packet);
		}
	}
}