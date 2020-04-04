use crate::world::block::Block;

pub struct Chunk {
	blocks: [Block; 65536]
}

impl Chunk {
	pub fn get_block_at(&self, x: u8, y: u8, z: u8) -> &Block {
		&self.blocks[((y as usize) << 8) + ((z as usize) << 4) + x as usize]
	}
}