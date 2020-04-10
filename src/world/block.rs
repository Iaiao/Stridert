use crate::registry::blocks::BlockType;

#[derive(Clone, PartialEq, Copy)]
pub struct Block {
	block_type: BlockType,
	light: u8 // 0xf_f
	//        sky -^|^- block
}

impl Block {
	pub fn new(block_type: BlockType) -> Block {
		return Block {
			block_type,
			light: 0x00
		}
	}
	pub fn is_solid (&self) -> bool {true }
	pub fn is_opaque(&self) -> bool {false}
	pub fn is_fluid (&self) -> bool {false}
	pub fn is_leaves(&self) -> bool {false}
	pub fn get_block_light(&self) -> u8 { self.light & 15 }
	pub fn get_sky_light(&self) -> u8 { self.light >> 4 }
	pub fn get_light_emission(&self) -> u8 { 0 }
	pub fn get_type(&self) -> BlockType { self.block_type.clone() }
}