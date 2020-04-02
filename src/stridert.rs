use crate::entity::player::Player;
use crate::world::world::World;
use crate::config;
use crate::registry::{dimension::Dimension, difficulty::Difficulty, identifier::Identifier};
use std::sync::{Arc, Mutex};
use crate::network::{self, packets};

pub struct Stridert {
	mod_name: String,
	version: String,
	protocol: i32,
	icon: String,
	worlds: Vec<Arc<Mutex<World>>>,
	max_players: usize,
	view_distance: u8
}

impl Stridert {
	pub fn new() -> Stridert {
		let img: image::DynamicImage = image::open("server-icon.png").unwrap();
		let mut buf: Vec<u8> = Vec::new();
		let _ = img.write_to(&mut buf, image::ImageOutputFormat::Png);
		let icon = base64::encode(&buf);
		return Stridert {
			mod_name: String::from("stridert"),
			version: String::from(config::VERSION),
			protocol: 709,
			icon,
			worlds: vec!(
				Arc::new(Mutex::new(World::new(
					String::from("world"),
					Dimension::OVERWORLD,
					0,
					Difficulty::EASY
				))),
				Arc::new(Mutex::new(World::new(
					String::from("world"),
					Dimension::NETHER,
					0,
					Difficulty::EASY
				))),
				Arc::new(Mutex::new(World::new(
					String::from("world"),
					Dimension::END,
					0,
					Difficulty::EASY
				)))
			),
			max_players: config::MAX_PLAYERS,
			view_distance: config::VIEW_DISTANCE
		}
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
		let mut world = self.worlds[0].lock().unwrap();
		let p = player.lock().unwrap();
		let mut conn = p.connection.lock().unwrap();
		conn.send(&packets::clientboundloginsuccesspacket::ClientboundLoginSuccessPacket::new(
			p.get_name(),
			p.get_entity().get_uuid()
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
		world.add_player(player.clone());
		network::packet_listener::start(player.clone());
	}
	pub fn get_player_world(&self, player: Arc<Mutex<Player>>) -> Option<Arc<Mutex<World>>> {
		let player = player.lock().unwrap();
		for world in &self.worlds {
			for p in world.lock().unwrap().get_players() {
				if *p.lock().unwrap() == *player {
					return Some(world.clone())
				}
			}
		};
		None
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
}