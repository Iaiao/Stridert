use crate::inventory::item::Item;

#[derive(Clone)]
pub struct ItemStack {
	amount: i8,
	item: Item
	// nbt
}

impl ItemStack {
	pub fn new(item: Item) -> ItemStack {
		return ItemStack {
			item,
			amount: 1
		}
	}
	pub fn get_amount(&self) -> i8 { self.amount }
	pub fn set_amount(&mut self, amount: i8) { self.amount = amount }
	pub fn get_item(&self) -> Item { self.item.clone() }
}