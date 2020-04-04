use crate::inventory::{recipe::Recipe, itemstack::ItemStack};
use crate::registry::{identifier::Identifier, recipetypes::{RecipeType, RecipeShapeless}, items};

pub fn fill_recipes(recipes: &mut Vec<Recipe>) {
	recipes.append(&mut vec!(
		Recipe::new(
			Identifier::from_string("minecraft:oak_planks"),
			RecipeType::Shapeless(RecipeShapeless::new(
				items::get(items::OAK_PLANKS).unwrap(),
				vec!(
					vec!(
						ItemStack::new(items::get(items::OAK_LOG).unwrap())
					)
				),
				ItemStack::new(items::get(items::OAK_PLANKS).unwrap())
			)),
			ItemStack::new(items::get(items::OAK_PLANKS).unwrap())
		)
	));
}