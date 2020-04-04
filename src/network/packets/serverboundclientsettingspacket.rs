use crate::network::packets::{self, packet::{Packet, ServerboundPacket}, friendlybytebuf::FriendlyByteBuf};
use crate::registry::{chatmodes::ChatMode, hands::Hand};
use crate::network::connection::Connection;
use crate::SERVER;

pub struct ServerboundClientSettingsPacket {
	locale: String,
pub view_distance: u8,
	chat_mode: ChatMode,
	chat_colors: bool,
	skin_parts: u8,
	main_hand: Hand
}

impl Packet for ServerboundClientSettingsPacket {
	const ID: i32 = 0x05;
}

impl ServerboundPacket for ServerboundClientSettingsPacket {
	fn deserialize(buf: &mut FriendlyByteBuf) -> ServerboundClientSettingsPacket {
		let locale = buf.read_string();
		let view_distance = buf.read_byte();
		let chat_mode = match buf.read_varint() {
			1 => ChatMode::CommandsOnly,
			2 => ChatMode::Hidden,
			_ => ChatMode::Enabled
		};
		let chat_colors = buf.read_boolean();
		let skin_parts = buf.read_byte();
		let _cape = skin_parts & 0b00000001 != 0;
		let _jacket = skin_parts & 0b00000010 != 0;
		let _left_sleeve = skin_parts & 0b00000100 != 0;
		let _right_sleeve = skin_parts & 0b00001000 != 0;
		let _left_pants_leg = skin_parts & 0b00010000 != 0;
		let _right_pants_leg = skin_parts & 0b00100000 != 0;
		let _hat = skin_parts & 0b01000000 != 0;
		let main_hand = match buf.read_varint() {
			0 => Hand::Left,
			_ => Hand::Right
		};
		return ServerboundClientSettingsPacket {
			locale,
			view_distance,
			chat_mode,
			chat_colors,
			skin_parts,
			main_hand
		}
	}
}

impl ServerboundClientSettingsPacket {
	pub fn handle(&self, connection: &mut Connection) {
		connection.send(&packets::clientboundhelditemchangepacket::ClientboundHeldItemChangePacket::new(0));
		connection.send(&packets::clientbounddeclarerecipespacket::ClientboundDeclareRecipesPacket::new((*SERVER).lock().unwrap().get_recipes()));
		connection.send(&packets::clientboundtagspacket::ClientboundTagsPacket::new());
	}
}