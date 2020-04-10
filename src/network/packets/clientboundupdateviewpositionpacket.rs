use crate::network::packets::{friendlybytebuf::FriendlyByteBuf, packet::{ClientboundPacket, Packet}};

pub struct ClientboundUpdateViewPositionPacket {
	chunk_x: i32,
	chunk_z: i32
}

impl Packet for ClientboundUpdateViewPositionPacket {
	const ID: i32 = 0x41;
}

impl ClientboundPacket for ClientboundUpdateViewPositionPacket {
	fn serialize(&self) -> Vec<u8> {
		let mut buf = FriendlyByteBuf::new();
		buf.write_varint(ClientboundUpdateViewPositionPacket::ID);
		buf.write_varint(self.chunk_x);
		buf.write_varint(self.chunk_z);
		return buf.to_bytes();
	}
}

impl ClientboundUpdateViewPositionPacket {
	pub fn new(x: f64, z: f64) -> ClientboundUpdateViewPositionPacket {
		return ClientboundUpdateViewPositionPacket {
			chunk_x: x as i32 >> 4,
			chunk_z: z as i32 >> 4
		}
	}
}