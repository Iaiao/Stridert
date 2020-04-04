#![allow(dead_code)]

use crate::registry::identifier::Identifier;

#[derive(Clone)]
pub enum Fluid {
	Water,
	FlowingWater,
	Lava,
	FlowingLava,
	Empty
}

impl Fluid {
	pub fn get_namespaced_id(&self) -> Identifier {
		return Identifier::new("minecraft", match self {
			Fluid::Water => "water",
			Fluid::FlowingWater => "flowing_water",
			Fluid::Lava => "lava",
			Fluid::FlowingLava => "flowing_lava",
			Fluid::Empty => "empty"
		})
	}
	pub fn get_id(&self) -> i32 {
		return match self {
			Fluid::FlowingWater => FLOWING_WATER,
			Fluid::Water => WATER,
			Fluid::FlowingLava => FLOWING_LAVA,
			Fluid::Lava => LAVA,
			Fluid::Empty => EMPTY,
		}
	}
}

pub const FLOWING_WATER: i32 = 1;
pub const WATER: i32 = 2;
pub const FLOWING_LAVA: i32 = 3;
pub const LAVA: i32 = 4;
pub const EMPTY: i32 = 0;

pub fn get_fluids() -> [Fluid; 5] {
	return [
		Fluid::Water,
		Fluid::FlowingWater,
		Fluid::Lava,
		Fluid::FlowingLava,
		Fluid::Empty
	]
}

pub fn get(fluid: i32) -> Fluid {
	return match fluid {
		FLOWING_WATER => Fluid::FlowingWater,
		WATER => Fluid::Water,
		FLOWING_LAVA => Fluid::FlowingLava,
		LAVA => Fluid::Lava,
		_ => Fluid::Empty
	}
}