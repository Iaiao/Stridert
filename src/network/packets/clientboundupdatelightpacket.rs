use crate::network::packets::{friendlybytebuf::FriendlyByteBuf, packet::{ClientboundPacket, Packet}};
use crate::world::chunk::Chunk;

pub struct ClientboundUpdateLightPacket {
	chunk_x: i32,
	chunk_z: i32,
	sky_mask: i32,
	block_mask: i32,
	empty_sky_mask: i32,
	empty_block_mask: i32,
	sky_light_updates: Vec<[u8; 2048]>,
	block_light_updates: Vec<[u8; 2048]>
}

impl Packet for ClientboundUpdateLightPacket {
	const ID: i32 = 0x25;
}

impl ClientboundUpdateLightPacket {
	pub fn new(chunk: &Chunk, sky_y: Vec<u8>, block_y: Vec<u8>) -> ClientboundUpdateLightPacket {
		let mut sky_mask = 0;
		for i in &sky_y {
			sky_mask |= 1 << i;
		}
		let mut block_mask = 0;
		for i in &block_y {
			block_mask |= 1 << i;
		}

		let mut sky_light_updates = Vec::new();
		for i in &sky_y {
			let mut updates = [0; 2048];
			for y in 0..16 {
				for z in 0..16 {
					for x in 0..8 {
						updates[y*16 + z*8] = (chunk.get_block(x, *i as i32 * 16 - 16 + y as i32, z as i32).get_sky_light() << 4) | chunk.get_block(x + 1, *i as i32 * 16 - 16 + y as i32, z as i32).get_sky_light()
					}
				}
			}
			sky_light_updates.push(updates);
		}
		let mut block_light_updates = Vec::new();
		for i in &block_y {
			let mut updates = [0; 2048];
			for y in 0..16 {
				for z in 0..16 {
					for x in 0..8 {
						updates[y*16 + z*8] = (chunk.get_block(x, *i as i32 * 16 - 16 + y as i32, z as i32).get_block_light() << 4) | chunk.get_block(x + 1, *i as i32 * 16 - 16 + y as i32, z as i32).get_block_light();
					}
				}
			}
			block_light_updates.push(updates);
		}
		return ClientboundUpdateLightPacket {
			chunk_x: chunk.get_x(),
			chunk_z: chunk.get_z(),
			sky_mask,
			block_mask,
			empty_sky_mask: 0,
			empty_block_mask: 0,
			sky_light_updates,
			block_light_updates
		}
	}
}

impl ClientboundPacket for ClientboundUpdateLightPacket {
	fn serialize(&self) -> Vec<u8> {
		let mut buf = FriendlyByteBuf::new();
		buf.write_varint(ClientboundUpdateLightPacket::ID);
		buf.write_varint(self.chunk_x);
		buf.write_varint(self.chunk_z);
		buf.write_varint(self.sky_mask);
		buf.write_varint(self.block_mask);
		buf.write_varint(self.empty_sky_mask);
		buf.write_varint(self.empty_block_mask);
		for updates in &self.sky_light_updates {
			buf.write_varint(2048);
			buf.write_bytes(&mut updates.to_vec());
		}
		for updates in &self.block_light_updates {
			buf.write_varint(2048);
			buf.write_bytes(&mut updates.to_vec());
		}
		return buf.to_bytes();
	}
}