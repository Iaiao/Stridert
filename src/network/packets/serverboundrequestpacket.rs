use crate::network::{connection};
use crate::network::packets::{packet::*, clientboundresponsepacket, friendlybytebuf};
use std::io::Write;

pub struct ServerboundRequestPacket {}

impl Packet for ServerboundRequestPacket {
	const ID: i32 = 0x00;
}

impl ServerboundPacket for ServerboundRequestPacket {
	fn deserialize(_buf: &mut friendlybytebuf::FriendlyByteBuf) -> ServerboundRequestPacket {
		return ServerboundRequestPacket {}
	}
}

impl ServerboundRequestPacket {
	pub fn handle(&self, connection: &mut connection::Connection) {
		connection.send(&clientboundresponsepacket::ClientboundResponsePacket::new(connection.server.clone()));
		connection.stream.flush().unwrap();
		connection.stream.shutdown(std::net::Shutdown::Both).unwrap();
	}
}