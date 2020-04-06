use crate::network::packets::{friendlybytebuf::FriendlyByteBuf, packet::{ClientboundPacket, Packet}};
use rand;

pub struct ClientboundPlayerPositionPacket {
	x: f64,
	y: f64,
	z: f64,
	yaw: f32,
	pitch: f32,
	x_rel: bool,
	y_rel: bool,
	z_rel: bool,
	yaw_rel: bool,
	pitch_rel: bool,
	teleport_id: i32
}

impl Packet for ClientboundPlayerPositionPacket {
	const ID: i32 = 0x36;
}

impl ClientboundPacket for ClientboundPlayerPositionPacket {
	fn serialize(&self) -> Vec<u8> {
		let mut buf = FriendlyByteBuf::new();
		buf.write_varint(ClientboundPlayerPositionPacket::ID);
		buf.write_double(self.x);
		buf.write_double(self.y);
		buf.write_double(self.z);
		buf.write_float(self.yaw);
		buf.write_float(self.pitch);
		let mut flags = 0x00;
		if self.x_rel {
			flags |= 0x01;
		}
		if self.y_rel {
			flags |= 0x02;
		}
		if self.z_rel {
			flags |= 0x04;
		}
		if self.pitch_rel {
			flags |= 0x08;
		}
		if self.yaw_rel {
			flags |= 0x10;
		}
		buf.write_byte(flags);
		buf.write_varint(self.teleport_id);
		return buf.to_bytes();
	}
}

impl ClientboundPlayerPositionPacket {
	pub fn new(
		x: f64,
		y: f64,
		z: f64,
		yaw: f32,
		pitch: f32,
		x_rel: bool,
		y_rel: bool,
		z_rel: bool,
		yaw_rel: bool,
		pitch_rel: bool
	) -> ClientboundPlayerPositionPacket {
		let teleport_id: u16 = rand::random();
		// Возможно некоторые клиенты не поймут числа со знаком
		return ClientboundPlayerPositionPacket {
			x,
			y,
			z,
			yaw,
			pitch,
			x_rel,
			y_rel,
			z_rel,
			yaw_rel,
			pitch_rel,
			teleport_id: teleport_id as i32
		}
	}
}