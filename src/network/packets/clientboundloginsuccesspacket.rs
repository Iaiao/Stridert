use crate::network::packets::{packet::{ClientboundPacket, Packet}, friendlybytebuf};
use crate::util;

pub struct ClientboundLoginSuccessPacket {
	uuid: util::uuid::UUID,
	username: String 
}

impl Packet for ClientboundLoginSuccessPacket {
	const ID: i32 = 0x02;
}

impl ClientboundPacket for ClientboundLoginSuccessPacket {
	fn serialize(&self) -> Vec<u8> {
		let mut buf = friendlybytebuf::FriendlyByteBuf::new();
		buf.write_varint(ClientboundLoginSuccessPacket::ID);
		for i in &util::uuid::encode(&self.uuid) {
			buf.write_uint(*i);
		}
		buf.write_string(&self.username);
		return buf.to_bytes()
	}
}

impl ClientboundLoginSuccessPacket {
	pub fn new(username: String, uuid: &util::uuid::UUID) -> ClientboundLoginSuccessPacket {
		return ClientboundLoginSuccessPacket {
			username: username.clone(),
			uuid: uuid.clone()
		}
	}
}