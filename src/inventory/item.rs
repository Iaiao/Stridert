use crate::registry::identifier::Identifier;

#[derive(Clone, PartialEq)]
pub struct Item {
	namespace: String,
	name: String,
	id: i32
}

impl Item {
	pub fn new(namespace: &str, name: &str, id: i32) -> Item {
		return Item {
			namespace: String::from(namespace),
			name: String::from(name),
			id
		}
	}
	pub fn get_id(&self) -> i32 { self.id }
	pub fn get_name(&self) -> String { self.name.clone() }
	pub fn get_namespace(&self) -> String { self.namespace.clone() }
	pub fn get_identifier(&self) -> Identifier {
		return Identifier::new(self.namespace.clone(), self.name.clone())
	}
}