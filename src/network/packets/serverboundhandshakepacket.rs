use crate::network::{connection, states};
use crate::network::packets::{packet::*, friendlybytebuf};
use crate::config;

pub struct ServerboundHandshakePacket {
	next_state: states::State,
	protocol: i32,
	valid: bool
}

impl Packet for ServerboundHandshakePacket {
	const ID: i32 = 0x00;
}

impl ServerboundPacket for ServerboundHandshakePacket {
	fn deserialize(buf: &mut friendlybytebuf::FriendlyByteBuf) -> ServerboundHandshakePacket {
		// Следующее состояние
		let protocol = buf.read_varint();
		let ip = buf.read_string();
		let port = buf.read_ushort();
		let next_state = match buf.read_varint() {
			1 => states::State::STATUS,
			2 => states::State::LOGIN,
			// Либо 1, либо 2, остальное - кик
			_ => states::State::HANDSHAKING
		};
		let valid = next_state != states::State::HANDSHAKING;
		dbg!(protocol, ip, port);
		return ServerboundHandshakePacket {
			next_state,
			protocol,
			valid
		}
	}
}

impl ServerboundHandshakePacket {
	pub fn handle(&self, connection: &mut connection::Connection) {
		if !self.valid {
			return
		}
		if self.next_state == states::State::LOGIN {
			if config::SUPPORTED_VERSIONS.contains(&self.protocol) {
				connection.state = states::State::LOGIN;
			} else {
				connection.disconnect(format!("Я поддерживаю только версии {:?}", &config::SUPPORTED_VERSIONS));
			}
		} else {
			connection.state = states::State::STATUS;
		}
	}
}