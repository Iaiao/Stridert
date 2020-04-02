use crate::network::packets::{packet::{ClientboundPacket, Packet}, friendlybytebuf};
use crate::util;

pub struct ClientboundLoginSuccessPacket {
	uuid: [i32; 4],
	username: String 
}

impl Packet for ClientboundLoginSuccessPacket {
	const ID: i32 = 0x02;
}

impl ClientboundPacket for ClientboundLoginSuccessPacket {
	fn serialize(&self) -> Vec<u8> {
		let mut buf = friendlybytebuf::FriendlyByteBuf::new();
		buf.write_varint(ClientboundLoginSuccessPacket::ID);
		buf.write_int(self.uuid[0]);
		buf.write_int(self.uuid[1]);
		buf.write_int(self.uuid[2]);
		buf.write_int(self.uuid[3]);
		buf.write_string(&self.username);
		return buf.to_bytes()
	}
}

impl ClientboundLoginSuccessPacket {
	pub fn new(username: String, uuid: &util::uuid::UUID) -> ClientboundLoginSuccessPacket {
		return ClientboundLoginSuccessPacket {
			username: username.clone(),
			uuid: util::uuid::encode(uuid)
		}
	}
}