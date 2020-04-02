use crate::network::packets::{friendlybytebuf::FriendlyByteBuf, packet::{Packet, ClientboundPacket}};

pub struct ClientboundPlayerAbilitiesPacket {
	invulnerable: bool,
	flying: bool,
	allow_flying: bool,
	instant_break: bool,
	flying_speed: f32,
	fov: f32
}

impl ClientboundPlayerAbilitiesPacket {
	pub fn new(
		invulnerable: bool,
		flying: bool,
		allow_flying: bool,
		instant_break: bool,
		flying_speed: f32,
		fov: f32
	) -> ClientboundPlayerAbilitiesPacket {
		return ClientboundPlayerAbilitiesPacket {
			invulnerable,
			flying,
			allow_flying,
			instant_break,
			flying_speed,
			fov
		}
	}
}

impl Packet for ClientboundPlayerAbilitiesPacket {
	const ID: i32 = 0x32;
}

impl ClientboundPacket for ClientboundPlayerAbilitiesPacket {
	fn serialize(&self) -> Vec<u8> {
		let mut buf = FriendlyByteBuf::new();
		buf.write_varint(ClientboundPlayerAbilitiesPacket::ID);
		let mut bitfield = 0;
		bitfield = if self.invulnerable { bitfield | 0b00000001 } else { bitfield };
		bitfield = if self.flying { bitfield | 0b00000010 } else { bitfield };
		bitfield = if self.allow_flying { bitfield | 0b00000100 } else { bitfield };
		bitfield = if self.instant_break { bitfield | 0b00001000 } else { bitfield };
		buf.write_byte(bitfield);
		buf.write_float(self.flying_speed);
		buf.write_float(self.fov);
		return buf.to_bytes();
	}
}