use crate::network::packets::{friendlybytebuf, packet::*};
use crate::stridert::Stridert;
use crate::config;
use std::sync::{Arc, Mutex};
use serde_json::json;

pub struct ClientboundResponsePacket {
	response: String
}

impl Packet for ClientboundResponsePacket {
	const ID: i32 = 0x00;
}

impl ClientboundPacket for ClientboundResponsePacket {
	fn serialize(&self) -> Vec<u8> {
		let mut buf = friendlybytebuf::FriendlyByteBuf::new();
		buf.write_varint(ClientboundResponsePacket::ID as i32);
		buf.write_string(&self.response);
		return buf.to_bytes();
	}
}

impl ClientboundResponsePacket {
	pub fn new(server: Arc<Mutex<Stridert>>) -> ClientboundResponsePacket {
		let server = server.lock().unwrap();
		return ClientboundResponsePacket {
			response: json!({
				"version": {
					"name": server.get_version(),
					"protocol": server.get_protocol()
				},
				"players": {
					"max": server.get_max_players(),
					"online": server.get_player_count()
	//				"sample": []
				},
				"description": {
					"text": config::MOTD
				},
				"favicon": server.get_icon()
			}).to_string()
		}
	}
}