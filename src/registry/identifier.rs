#[derive(Clone)]
pub struct Identifier {
	key: String,
	value: String,
}

impl Identifier {
	pub fn new(key: String, value: String) -> Identifier {
		Identifier { key, value }
	}
	pub fn from_string(identifier: String) -> Identifier {
		let kv: Vec<&str> = identifier.split(":").collect();
		Identifier {
			key: kv[0].to_string(),
			value: kv[1].to_string()
		}
	}
	pub fn get_key(&self) -> String { self.key.clone() }
	pub fn get_value(&self) -> String { self.value.clone() }
	pub fn to_string(&self) -> String { self.key.clone() + ":" + &self.value}
}