use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Property {
	pub name: String,
	pub value: String,
	pub signature: Option<String>
}

impl Property {
	pub fn new(name: String, value: String) -> Property {
		return Property {
			name,
			value,
			signature: Option::None
		}
	}
	pub fn signed(name: String, value: String, signature: String) -> Property {
		return Property {
			name,
			value,
			signature: Option::from(signature)
		}
	}
}