use crate::network::packets::{friendlybytebuf::FriendlyByteBuf, packet::{Packet, ClientboundPacket}};
use crate::registry::identifier::Identifier;

pub struct ClientboundPluginMessagePacket {
	identifier: Identifier,
	data: Vec<u8>
}

impl ClientboundPluginMessagePacket {
	pub fn new(identifier: Identifier, data: Vec<u8>) -> ClientboundPluginMessagePacket {
		return ClientboundPluginMessagePacket {
			identifier,
			data
		}
	}
}

impl Packet for ClientboundPluginMessagePacket {
	const ID: i32 = 0x19;
}

impl ClientboundPacket for ClientboundPluginMessagePacket {
	fn serialize(&self) -> Vec<u8> {
		let mut buf = FriendlyByteBuf::new();
		buf.write_varint(ClientboundPluginMessagePacket::ID);
		buf.write_identifier(self.identifier.clone());
		buf.write_bytes(&mut self.data.clone());
		return buf.to_bytes();
	}
}