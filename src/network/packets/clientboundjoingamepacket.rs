use crate::registry::gamemodes::GameMode;
use crate::registry::leveltypes::LevelType;
use crate::network::packets::{friendlybytebuf, packet::{ClientboundPacket, Packet}};
use crate::registry::dimensions::Dimension;

use sha2::{Sha256, Digest};

pub struct ClientboundJoinGamePacket {
	eid: i32,
	gamemode: GameMode,
	hardcore: bool,
	dimension: Dimension,
	seed: i64,
	max_players: u8,
	level_type: LevelType,
	view_distance: i32,
	reduced_debug_info: bool,
	enable_respawn_screen: bool
}

impl Packet for ClientboundJoinGamePacket {
	const ID: i32 = 0x26;
}

impl ClientboundPacket for ClientboundJoinGamePacket {
	fn serialize(&self) -> Vec<u8> {
		let mut buf = friendlybytebuf::FriendlyByteBuf::new();

		buf.write_varint(ClientboundJoinGamePacket::ID);

		buf.write_int(self.eid);

		let a = match self.gamemode {
			GameMode::SURVIVAL  => 0,
			GameMode::CREATIVE  => 1,
			GameMode::ADVENTURE => 2,
			GameMode::SPECTATOR => 3,
		};
		buf.write_byte(if self.hardcore { a | 0b00000100 } else { a });

		buf.write_int(match self.dimension {
			Dimension::Nether => -1,
			Dimension::Overworld => 0,
			Dimension::TheEnd => 1
		});

		let mut hasher = Sha256::new();
		hasher.input(self.seed.to_string());
		buf.write_bytes(&mut hasher.result()[..8].to_vec());

		buf.write_byte(self.max_players);

		buf.write_string(&self.level_type.to_string());

		buf.write_varint(self.view_distance);

		buf.write_boolean(self.reduced_debug_info);

		buf.write_boolean(self.enable_respawn_screen);

		return buf.to_bytes();
	}
}

impl ClientboundJoinGamePacket {
	pub fn new(
		eid: i32,
		gamemode: GameMode,
		hardcore: bool,
		dimension: Dimension,
		seed: i64,
		max_players: u8,
		level_type: LevelType,
		view_distance: i32,
		reduced_debug_info: bool,
		enable_respawn_screen: bool
	) -> ClientboundJoinGamePacket {
		ClientboundJoinGamePacket {
			eid,
			gamemode,
			hardcore,
			dimension,
			seed,
			max_players,
			level_type,
			view_distance,
			reduced_debug_info,
			enable_respawn_screen
		}
	}
}