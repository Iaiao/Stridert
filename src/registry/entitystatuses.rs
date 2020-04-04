#![allow(dead_code)]

pub mod player {
	pub const ITEM_USE_FINISHED: u8 = 9;
	pub const ENABLE_REDUCED_DEBUG_SCREEN: u8 = 22;
	pub const DISABLE_REDUCED_DEBUG_SCREEN: u8 = 22;
	pub const OP_PERMISSION_LEVEL_0: u8 = 24;
	pub const OP_PERMISSION_LEVEL_1: u8 = 25;
	pub const OP_PERMISSION_LEVEL_2: u8 = 26;
	pub const OP_PERMISSION_LEVEL_3: u8 = 27;
	pub const OP_PERMISSION_LEVEL_4: u8 = 28;
	pub const SPAWN_BAD_OMEN_CLOUDS: u8 = 43;
}

pub mod tipped_arrow {
	pub const SPAWN_PARTICLE_EFFECT: u8 = 0;
}

pub mod rabbit {
	pub const JUMP_ANIMATION_AND_PARTICLES: u8 = 1;
}

pub mod minecart_spawner {
	pub const RESET_DELAY: u8 = 1;
}

pub mod living {
	pub const HURT_ANIMATION_AND_SOUND: u8 = 2;
	pub const DEATH_ANIMATION_AND_SOUND: u8 = 3;
	pub const PLAY_SHIELD_BLOCK_SOUND: u8 = 29;
	pub const PLAY_SHIELD_BREAK_SOUND: u8 = 30;
	pub const PLAY_THORNS_SOUND_AND_HURT_SOUND_AND_ANIMATION: u8 = 33;
	pub const PLAY_DROWN_SOUND_AND_HURT_SOUND_AND_ANIMATION: u8 = 36;
	pub const PLAY_BURN_SOUND_AND_HURT_SOUND_AND_ANIMATION: u8 = 37;
	pub const PLAY_SWEET_BERRY_BUSH_SOUND_AND_HURT_SOUND_AND_ANIMATION: u8 = 44;
	pub const SPAWN_TELEPORT_PARTICLES: u8 = 46;
	pub const EQUIPMENT_BREAK_IN_MAIN_HAND: u8 = 47;
	pub const EQUIPMENT_BREAK_OFF_MAIN_HAND: u8 = 48;
	pub const EQUIPMENT_BREAK_IN_HEAD: u8 = 49;
	pub const EQUIPMENT_BREAK_IN_CHEST: u8 = 50;
	pub const EQUIPMENT_BREAK_IN_LEGS: u8 = 51;
	pub const EQUIPMENT_BREAK_IN_FEET: u8 = 52;
}

pub mod snowball {
	pub const SNOWBOOLPOOF_PARTICLES_8: u8 = 3;
}

pub mod egg {
	pub const IRONCRACK_EGG_PARTICLES_8: u8 = 3;
}

pub mod iron_golem {
	pub const ATTACK_ANIMATION_AND_SOUND: u8 = 4;
	pub const HOLD_A_POPPY: u8 = 11;
	pub const PUT_POPPY_AWAY: u8 = 34;
}

pub mod evocation_fangs {
	pub const ATTACK_ANIMATION_AND_SOUND: u8 = 4;
}

pub mod ravager {
	pub const START_ATTACK_ANIMATION: u8 = 4;
	pub const STUNNED: u8 = 39;
}

pub mod abstract_horse {
	pub const TAMING_FAILED_SPAWN_SMOKE_PARTICLES: u8 = 6;
	pub const TAMING_SUCCEED_SPAWN_HEART_PARTICLES: u8 = 7;
}

pub mod tameable_animal {
	pub const TAMING_FAILED_SPAWN_SMOKE_PARTICLES: u8 = 6;
	pub const TAMING_SUCCEED_SPAWN_HEART_PARTICLES: u8 = 7;
}

pub mod wolf {
	pub const PLAY_SHAKING_WATER_ANIMATION: u8 = 8;
}

pub mod sheep {
	pub const PLAY_EATING_GRASS_ANIMATION: u8 = 10;
}

pub mod minecart_tnt {
	pub const IGNITE_TNT: u8 = 10;
}

pub mod villager {
	pub const SPAWN_HEART_PARTICLES: u8 = 12;
	pub const SPAWN_ANGRY_PARTICLES: u8 = 13;
	pub const SPAWN_HAPPY_PARTICLES: u8 = 14;
	pub const SPAWN_SPLASH_RAID_PARTICLES: u8 = 42;
}

pub mod witch {
	pub const SPAWN_WITCH_MAGIC_PARTICLES: u8 = 15;
}

pub mod zombie_villager {
	pub const PLAY_CURE_FINISHED_SOUND: u8 = 16;
}

pub mod fireworks {
	pub const PLAY_EXPLOSION_EFFECT: u8 = 17;
}

pub mod animal {
	pub const MAKE_BEBE: u8 = 18;
}

pub mod squid {
	pub const RESET_ROTATION: u8 = 19;
}

pub mod insertient {
	pub const SPAWN_PARTICLES: u8 = 20;
}

pub mod guardian {
	pub const PLAY_ATTACK_SOUND: u8 = 21;
}

pub mod fishing_hook {
	pub const PULL_PLAYER_TOWARD_THE_CASTER: u8 = 31;
}

pub mod armor_stand {
	pub const PLAY_HIT_SOUND_AND_RESET_COOLDOWN: u8 = 32;
}

pub mod entity {
	pub const PLAY_TOTEM_OF_UNDYING_EFFECT: u8 = 35;
}

pub mod dolphin {
	pub const SPAWN_HAPPY_VILLAGER_EFFECTS: u8 = 38;
}

pub mod fox {
	pub const SPAWN_CHEWING_ITEM_PARTICLES: u8 = 45;
}