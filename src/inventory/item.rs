#[derive(Clone, PartialEq)]
pub struct Item {
	id: i32
	// nbt
}

impl Item {
	pub fn new(id: i32) -> Item {
		return Item { id }
	}
	pub fn get_id(&self) -> i32 { self.id }
}