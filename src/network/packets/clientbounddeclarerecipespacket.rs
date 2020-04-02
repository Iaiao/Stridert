use crate::network::packets::{friendlybytebuf, packet::{ClientboundPacket, Packet}};
use crate::inventory::recipe::Recipe;

pub struct ClientboundDeclareRecipesPacket {
	recipes: Vec<Recipe>
}

impl Packet for ClientboundDeclareRecipesPacket {
	const ID: i32 = 0x5B;
}

impl ClientboundPacket for ClientboundDeclareRecipesPacket {
	fn serialize(&self) -> Vec<u8> {
		let mut buf = friendlybytebuf::FriendlyByteBuf::new();
		buf.write_varint(ClientboundDeclareRecipesPacket::ID);
		buf.write_varint(self.recipes.len() as i32);
		for recipe in self.recipes.clone() {
			recipe.write(&mut buf);
		}
		return buf.to_bytes();
	}
}

impl ClientboundDeclareRecipesPacket {
	pub fn new(recipes: Vec<Recipe>) -> ClientboundDeclareRecipesPacket {
		return ClientboundDeclareRecipesPacket { recipes }
	}
}