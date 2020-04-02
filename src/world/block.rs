pub enum Block {
	STONE
}

impl Block {
	pub fn is_solid (&self) -> bool {true }
	pub fn is_opaque(&self) -> bool {false}
	pub fn is_fluid (&self) -> bool {false}
	pub fn is_leaves(&self) -> bool {false}
	pub fn get_light_emission(&self) -> u8 { 0 }
}