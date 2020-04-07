use crate::network::packets::{friendlybytebuf::FriendlyByteBuf, packet::{ClientboundPacket, Packet}};
use crate::inventory::recipe::Recipe;

pub struct ClientboundUnlockRecipesPacket {
	action: Action,
	open_crafting_recipe_book: bool,
	filter_crafting_recipe_book: bool,
	open_smelting_recipe_book: bool,
	filter_smelting_recipe_book: bool,
	recipes: Vec<Recipe>,
	init_recipes: Option<Vec<Recipe>>
}

impl Packet for ClientboundUnlockRecipesPacket {
	const ID: i32 = 0x37;
}

impl ClientboundPacket for ClientboundUnlockRecipesPacket {
	fn serialize(&self) -> Vec<u8> {
		let mut buf = FriendlyByteBuf::new();
		buf.write_varint(ClientboundUnlockRecipesPacket::ID);
		buf.write_varint(match self.action {
			Action::Init   => 0,
			Action::Add    => 1,
			Action::Remove => 2
		});
		buf.write_boolean(self.open_crafting_recipe_book);
		buf.write_boolean(self.filter_crafting_recipe_book);
		buf.write_boolean(self.open_smelting_recipe_book);
		buf.write_boolean(self.filter_smelting_recipe_book);
		buf.write_varint(self.recipes.len() as i32);
		for recipe in &self.recipes {
			buf.write_identifier(recipe.get_id());
		}
		if self.action == Action::Init {
			buf.write_varint((&self.init_recipes).as_ref().unwrap().len() as i32);
			for recipe in (&self.init_recipes).as_ref().unwrap() {
				buf.write_identifier(recipe.get_id());
			}
		}
		return buf.to_bytes();
	}
}

impl ClientboundUnlockRecipesPacket {
	pub fn new(
		action: Action,
		open_crafting_recipe_book: bool,
		filter_crafting_recipe_book: bool,
		open_smelting_recipe_book: bool,
		filter_smelting_recipe_book: bool,
		recipes: Vec<Recipe>,
		init_recipes: Option<Vec<Recipe>>
	) -> ClientboundUnlockRecipesPacket {
		return ClientboundUnlockRecipesPacket {
			action,
			open_crafting_recipe_book,
			filter_crafting_recipe_book,
			open_smelting_recipe_book,
			filter_smelting_recipe_book,
			recipes,
			init_recipes
		}
	}
}

#[derive(PartialEq)]
pub enum Action {
	Init,
	Add,
	Remove
}