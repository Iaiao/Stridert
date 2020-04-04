#![allow(dead_code)]

#[derive(PartialEq, Copy, Clone)]
pub enum LevelType {
	Default,
	Flat,
	LargeBiomes,
	Amplified,
	Customized,
	Buffet,
	Default1_1 
}

impl LevelType {
	pub fn to_string(&self) -> String {
		return String::from(match self {
			LevelType::Amplified => "amplified",
			LevelType::Buffet => "buffet",
			LevelType::Customized => "customized",
			LevelType::Default => "default",
			LevelType::Default1_1 => "default_1_1",
			LevelType::Flat => "flat",
			LevelType::LargeBiomes => "large_biomes"
		})
	}
}