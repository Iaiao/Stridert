use crate::network::connection;
use crate::network::packets::{packet::*, friendlybytebuf};
use crate::registry::identifier::Identifier;

pub struct ServerboundPluginMessagePacket {
	identifier: Identifier,
	data: Vec<u8>
}

impl Packet for ServerboundPluginMessagePacket {
	const ID: i32 = 0x0B;
}

impl ServerboundPacket for ServerboundPluginMessagePacket {
	fn deserialize(buf: &mut friendlybytebuf::FriendlyByteBuf) -> ServerboundPluginMessagePacket {
		let identifier = buf.read_identifier();
		let data = buf.read_bytes(buf.len() - buf.pointer);
		return ServerboundPluginMessagePacket {
			identifier,
			data
		}
	}
}

impl ServerboundPluginMessagePacket {
	pub fn handle(&self, _connection: &mut connection::Connection) {
		match self.identifier.to_string().as_str() {
			"minecraft:brand" => {
				let _brand = friendlybytebuf::FriendlyByteBuf::from(self.data.clone()).read_string();
			}
			_ => {}
		}
	}
}