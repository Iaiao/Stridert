use crate::inventory::{item::Item, itemstack::ItemStack};

#[derive(Clone)]
pub enum RecipeType {
	SHAPELESS(RecipeShapeless)
}

#[derive(Clone)]
pub struct RecipeShapeless {
	group: Item,
	ingredients: Vec<Vec<ItemStack>>,
	result: ItemStack
}

impl RecipeShapeless {
	pub fn new(group: Item, ingredients: Vec<Vec<ItemStack>>, result: ItemStack) -> RecipeShapeless {
		return RecipeShapeless {
			group,
			ingredients,
			result
		}
	}
	pub fn get_group(&self) -> Item { self.group.clone() }
	pub fn get_ingredients(&self) -> &Vec<Vec<ItemStack>> { &self.ingredients }
}