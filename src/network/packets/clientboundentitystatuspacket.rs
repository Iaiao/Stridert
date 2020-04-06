use crate::network::packets::{friendlybytebuf::FriendlyByteBuf, packet::{ClientboundPacket, Packet}};

pub struct ClientboundEntityStatusPacket {
	eid: i32,
	status: u8
}

impl Packet for ClientboundEntityStatusPacket {
	const ID: i32 = 0x1C;
}

impl ClientboundPacket for ClientboundEntityStatusPacket {
	fn serialize(&self) -> Vec<u8> {
		let mut buf = FriendlyByteBuf::new();
		buf.write_varint(ClientboundEntityStatusPacket::ID);
		buf.write_int(self.eid);
		buf.write_byte(self.status);
		return buf.to_bytes();
	}
}

impl ClientboundEntityStatusPacket {
	pub fn new(entity_id: i32, status: u8) -> ClientboundEntityStatusPacket {
		return ClientboundEntityStatusPacket {
			eid: entity_id,
			status
		}
	}
}