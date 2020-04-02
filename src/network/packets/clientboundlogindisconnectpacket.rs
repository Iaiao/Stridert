use crate::network::packets::{packet::{ClientboundPacket, Packet}, friendlybytebuf};

pub struct ClientboundLoginDisconnectPacket {
	reason: String
}

impl Packet for ClientboundLoginDisconnectPacket {
	const ID: i32 = 0x00;
}

impl ClientboundPacket for ClientboundLoginDisconnectPacket {
	fn serialize(&self) -> Vec<u8> {
		let mut buf = friendlybytebuf::FriendlyByteBuf::new();
		buf.write_varint(ClientboundLoginDisconnectPacket::ID);
		buf.write_string(&format!("{{\"text\": \"{}\"}}", self.reason));
		return buf.to_bytes();
	}
}

impl ClientboundLoginDisconnectPacket {
	pub fn new(reason: String) -> ClientboundLoginDisconnectPacket {
		return ClientboundLoginDisconnectPacket { reason }
	}
}