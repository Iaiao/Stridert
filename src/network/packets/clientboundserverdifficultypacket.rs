use crate::network::packets::{friendlybytebuf::FriendlyByteBuf, packet::{Packet, ClientboundPacket}};
use crate::registry::difficulties::Difficulty;

pub struct ClientboundServerDifficultyPacket {
	difficulty: Difficulty,
	locked: bool
}

impl Packet for ClientboundServerDifficultyPacket {
	const ID: i32 = 0x0E;
}

impl ClientboundPacket for ClientboundServerDifficultyPacket {
	fn serialize(&self) -> Vec<u8> {
		let mut buf = FriendlyByteBuf::new();
		buf.write_varint(ClientboundServerDifficultyPacket::ID);
		buf.write_byte(match self.difficulty {
			Difficulty::PEACEFUL => 0,
			Difficulty::EASY => 1,
			Difficulty::NORMAL => 2,
			Difficulty::HARD => 3
		});
		buf.write_boolean(self.locked);
		return buf.to_bytes();
	}
}

impl ClientboundServerDifficultyPacket {
	pub fn new(difficulty: Difficulty, locked: bool) -> ClientboundServerDifficultyPacket {
		return ClientboundServerDifficultyPacket {
			difficulty,
			locked
		}
	}
}