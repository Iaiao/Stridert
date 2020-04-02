use crate::network::{connection, states};
use crate::network::packets::{packet::*, friendlybytebuf};

pub struct ServerboundLoginStartPacket {
pub username: String,
pub logged: bool
}

impl Packet for ServerboundLoginStartPacket {
	const ID: i32 = 0x00;
}

impl ServerboundPacket for ServerboundLoginStartPacket {
	fn deserialize(buf: &mut friendlybytebuf::FriendlyByteBuf) -> ServerboundLoginStartPacket {
		let username = buf.read_string();
		return ServerboundLoginStartPacket {
			username,
			logged: true // пока что нет авторизации
		}
	}
}

impl ServerboundLoginStartPacket {
	pub fn handle(&self, connection: &mut connection::Connection) {
		if self.logged {
			connection.state = states::State::PLAY;
			connection.username = self.username.clone();
		}
	}
}