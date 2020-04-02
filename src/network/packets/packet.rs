use crate::network::{packets::friendlybytebuf::FriendlyByteBuf};

pub trait Packet {
	const ID: i32;
}

pub trait ClientboundPacket {
	fn serialize(&self) -> Vec<u8>;
}

pub trait ServerboundPacket {
	fn deserialize(buf: &mut FriendlyByteBuf) -> Self;
}