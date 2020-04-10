use crate::world::block::Block;
use crate::registry::blocks::BlockType;

pub struct Chunk {
	blocks: [Block; 65536],
	x: i32,
	z: i32
}

impl Chunk {
	pub fn new(x: i32, z: i32, blocks: [Block; 65536]) -> Chunk {
		return Chunk {
			x,
			z,
			blocks
		}
	}
	pub fn get_block(&self, x: i32, y: i32, z: i32) -> Block {
		if x < 0 || y < 0 || z < 0 || x >= 16 || y >= 256 || z >= 16 {
			return Block::new(BlockType::Air);
		}
		return self.blocks[((y as usize) << 8) + ((z as usize) << 4) + x as usize].clone()
	}
	pub fn get_block_mut(&mut self, x: i32, y: i32, z: i32) -> Option<&mut Block> {
		if x < 0 || y < 0 || z < 0 || x >= 16 || y >= 256 || z >= 16 {
			return None;
		}
		return Some(&mut self.blocks[((y as usize) << 8) + ((z as usize) << 4) + x as usize])
	}
	pub fn get_x(&self) -> i32 { self.x }
	pub fn get_z(&self) -> i32 { self.z }
}