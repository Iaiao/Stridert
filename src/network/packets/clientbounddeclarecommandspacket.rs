use crate::network::packets::{friendlybytebuf::FriendlyByteBuf, packet::{ClientboundPacket, Packet}};

pub struct ClientboundDeclareCommandsPacket {

}

impl Packet for ClientboundDeclareCommandsPacket {
	const ID: i32 = 0x12;
}

impl ClientboundPacket for ClientboundDeclareCommandsPacket {
	fn serialize(&self) -> Vec<u8> {
		let mut buf = FriendlyByteBuf::new();
		buf.write_varint(ClientboundDeclareCommandsPacket::ID);

		buf.write_varint(1);

		buf.write_byte(0x00);
		buf.write_varint(0);

		buf.write_varint(0);
	
		return buf.to_bytes();
	}
}

impl ClientboundDeclareCommandsPacket {
	pub fn new() -> ClientboundDeclareCommandsPacket {
		return ClientboundDeclareCommandsPacket {}
	}
}