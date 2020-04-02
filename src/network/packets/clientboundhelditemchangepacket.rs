use crate::network::packets::{friendlybytebuf, packet::{ClientboundPacket, Packet}};

pub struct ClientboundHeldItemChangePacket {
	slot: u8
}

impl Packet for ClientboundHeldItemChangePacket {
	const ID: i32 = 0x40;
}

impl ClientboundPacket for ClientboundHeldItemChangePacket {
	fn serialize(&self) -> Vec<u8> {
		let mut buf = friendlybytebuf::FriendlyByteBuf::new();
		buf.write_varint(ClientboundHeldItemChangePacket::ID);
		buf.write_byte(self.slot);
		return buf.to_bytes();
	}
}

impl ClientboundHeldItemChangePacket {
	pub fn new(slot: u8) -> ClientboundHeldItemChangePacket {
		return ClientboundHeldItemChangePacket {
			slot
		}
	}
}