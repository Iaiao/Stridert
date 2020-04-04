use crate::world::block::Block;
use crate::inventory::item::Item;
use crate::registry::items;
use crate::registry::blocks;
use crate::registry::fluids;
use crate::registry::entitytypes;
use crate::network::packets::{friendlybytebuf::FriendlyByteBuf, packet::{Packet, ClientboundPacket}};

pub struct ClientboundTagsPacket {
	blocks: Vec<Block>,
	items: Vec<Item>,
	fluids: Vec<fluids::Fluid>,
	entity_types: Vec<entitytypes::EntityType>,
}

impl Packet for ClientboundTagsPacket {
	const ID: i32 = 0x5C;
}

impl ClientboundPacket for ClientboundTagsPacket {
	fn serialize(&self) -> Vec<u8> {
		let mut buf = FriendlyByteBuf::new();
		buf.write_varint(ClientboundTagsPacket::ID);

		buf.write_varint(self.blocks.len() as i32);
		for block in self.blocks.clone() {
			buf.write_identifier(block.get_namespaced_id());
			buf.write_varint(1);
			buf.write_varint(block.get_id());
		}

		buf.write_varint(self.items.len() as i32);
		for item in self.items.clone() {
			buf.write_identifier(item.get_namespaced_id());
			buf.write_varint(1);
			buf.write_varint(item.get_id());
		}

		buf.write_varint(self.fluids.len() as i32);
		for fluid in self.fluids.clone() {
			buf.write_identifier(fluid.get_namespaced_id());
			buf.write_varint(1);
			buf.write_varint(fluid.get_id());
		}

		buf.write_varint(self.blocks.len() as i32);
		for block in self.blocks.clone() {
			buf.write_identifier(block.get_namespaced_id());
			buf.write_varint(1);
			buf.write_varint(block.get_id());
		}

		return buf.to_bytes();
	}
}

impl ClientboundTagsPacket {
	pub fn new() -> ClientboundTagsPacket {
		return ClientboundTagsPacket {
			blocks: blocks::get_blocks().to_vec(),
			items: items::get_items().to_vec(),
			fluids: fluids::get_fluids().to_vec(),
			entity_types: entitytypes::get_entity_types().to_vec()
		}
	}
}