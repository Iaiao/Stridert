use crate::registry::{identifier::Identifier, recipetypes::RecipeType};
use crate::network::packets::friendlybytebuf::FriendlyByteBuf;
use crate::inventory::itemstack::ItemStack;

#[derive(Clone)]
pub struct Recipe {
	id: Identifier,
	recipe_type: RecipeType,
	result: ItemStack
}

impl Recipe {
	pub fn new(id: Identifier, recipe_type: RecipeType, result: ItemStack) -> Recipe {
		return Recipe {
			id,
			recipe_type,
			result
		}
	}
	pub fn write(&self, buf: &mut FriendlyByteBuf) {

		match &self.recipe_type {
			RecipeType::SHAPELESS(data) => {
				buf.write_string(&String::from("crafting_shapeless"));

				buf.write_identifier(self.id.clone());

				buf.write_identifier(data.get_group().get_identifier());

				buf.write_varint(data.get_ingredients().len() as i32);

				for ingredient in data.get_ingredients() {

					buf.write_varint(ingredient.len() as i32);
					
					for item in ingredient {
						buf.write_itemstack(item);
					}
				}
				buf.write_itemstack(&self.result);
			}
		}
	}
}