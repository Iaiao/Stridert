#[derive(Clone)]
pub struct Identifier {
	namespace: String,
	name: String,
}

impl Identifier {
	pub fn new(namespace: String, name: String) -> Identifier {
		Identifier { namespace, name }
	}
	pub fn from_string<S: AsRef<str>>(identifier: S) -> Identifier {
		let kv: Vec<&str> = identifier.as_ref().split(":").collect();
		Identifier {
			namespace: kv[0].to_string(),
			name: kv[1].to_string()
		}
	}
	pub fn get_namespace(&self) -> String { self.namespace.clone() }
	pub fn get_name(&self) -> String { self.name.clone() }
	pub fn to_string(&self) -> String { self.namespace.clone() + ":" + &self.name}
}