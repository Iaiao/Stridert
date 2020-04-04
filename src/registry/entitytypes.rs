#![allow(dead_code)]

use crate::registry::identifier::Identifier;

#[derive(Copy, Clone)]
pub enum EntityType {
	AreaEffectCloud,
	ArmorStand,
	Arrow,
	Bat,
	Bee,
	Blaze,
	Boat,
	Cat,
	CaveSpider,
	Chicken,
	Cod,
	Cow,
	Creeper,
	Donkey,
	Dolphin,
	DragonFireball,
	Drowned,
	ElderGuardian,
	EndCrystal,
	EnderDragon,
	Enderman,
	Endermite,
	EvokerFangs,
	Evoker,
	ExperienceOrb,
	EyeOfEnder,
	FallingBlock,
	FireworkRocket,
	Fox,
	Ghast,
	Giant,
	Guardian,
	Horse,
	Husk,
	Illusioner,
	Item,
	ItemFrame,
	Fireball,
	LeashKnot,
	Llama,
	LlamaSpit,
	MagmaCube,
	Minecart,
	ChestMinecart,
	CommandBlockMinecart,
	FurnaceMinecart,
	HopperMinecart,
	SpawnerMinecart,
	TntMinecart,
	Mule,
	Mooshroom,
	Ocelot,
	Painting,
	Panda,
	Parrot,
	Pig,
	Pufferfish,
	PolarBear,
	Tnt,
	Rabbit,
	Salmon,
	Sheep,
	Shulker,
	ShulkerBullet,
	Silverfish,
	Skeleton,
	SkeletonHorse,
	Slime,
	SmallFireball,
	SnowGolem,
	Snowball,
	SpectralArrow,
	Spider,
	Squid,
	Stray,
	TraderLlama,
	TropicalFish,
	Turtle,
	Egg,
	EnderPearl,
	ExperienceBottle,
	Potion,
	Trident,
	Vex,
	Villager,
	IronGolem,
	Vindicator,
	Pillager,
	WanderingTrader,
	Witch,
	Wither,
	WitherSkeleton,
	WitherSkull,
	Wolf,
	Zombie,
	ZombifiedPiglin,
	ZombieHorse,
	ZombieVillager,
	Phantom,
	Ravager,
	Hoglin,
	Piglin,
	Strider,
	Zoglin,
	LightningBolt,
	Player,
	FishingBobber,
}

impl EntityType {
	pub fn get_namespaced_id(&self) -> Identifier {
		return Identifier::new("minecraft", match self {
			EntityType::AreaEffectCloud => "area_effect_cloud",
			EntityType::ArmorStand => "armor_stand",
			EntityType::Arrow => "arrow",
			EntityType::Bat => "bat",
			EntityType::Bee => "bee",
			EntityType::Blaze => "blaze",
			EntityType::Boat => "boat",
			EntityType::Cat => "cat",
			EntityType::CaveSpider => "cave_spider",
			EntityType::Chicken => "chicken",
			EntityType::Cod => "cod",
			EntityType::Cow => "cow",
			EntityType::Creeper => "creeper",
			EntityType::Donkey => "donkey",
			EntityType::Dolphin => "dolphin",
			EntityType::DragonFireball => "dragon_fireball",
			EntityType::Drowned => "drowned",
			EntityType::ElderGuardian => "elder_guardian",
			EntityType::EndCrystal => "end_crystal",
			EntityType::EnderDragon => "ender_dragon",
			EntityType::Enderman => "enderman",
			EntityType::Endermite => "endermite",
			EntityType::EvokerFangs => "evoker_fangs",
			EntityType::Evoker => "evoker",
			EntityType::ExperienceOrb => "experience_orb",
			EntityType::EyeOfEnder => "eye_of_ender",
			EntityType::FallingBlock => "falling_block",
			EntityType::FireworkRocket => "firework_rocket",
			EntityType::Fox => "fox",
			EntityType::Ghast => "ghast",
			EntityType::Giant => "giant",
			EntityType::Guardian => "guardian",
			EntityType::Horse => "horse",
			EntityType::Husk => "husk",
			EntityType::Illusioner => "illusioner",
			EntityType::Item => "item",
			EntityType::ItemFrame => "item_frame",
			EntityType::Fireball => "fireball",
			EntityType::LeashKnot => "leash_knot",
			EntityType::Llama => "llama",
			EntityType::LlamaSpit => "llama_spit",
			EntityType::MagmaCube => "magma_cube",
			EntityType::Minecart => "minecart",
			EntityType::ChestMinecart => "chest_minecart",
			EntityType::CommandBlockMinecart => "command_block_minecart",
			EntityType::FurnaceMinecart => "furnace_minecart",
			EntityType::HopperMinecart => "hopper_minecart",
			EntityType::SpawnerMinecart => "spawner_minecart",
			EntityType::TntMinecart => "tnt_minecart",
			EntityType::Mule => "mule",
			EntityType::Mooshroom => "mooshroom",
			EntityType::Ocelot => "ocelot",
			EntityType::Painting => "painting",
			EntityType::Panda => "panda",
			EntityType::Parrot => "parrot",
			EntityType::Pig => "pig",
			EntityType::Pufferfish => "pufferfish",
			EntityType::PolarBear => "polar_bear",
			EntityType::Tnt => "tnt",
			EntityType::Rabbit => "rabbit",
			EntityType::Salmon => "salmon",
			EntityType::Sheep => "sheep",
			EntityType::Shulker => "shulker",
			EntityType::ShulkerBullet => "shulker_bullet",
			EntityType::Silverfish => "silverfish",
			EntityType::Skeleton => "skeleton",
			EntityType::SkeletonHorse => "skeleton_horse",
			EntityType::Slime => "slime",
			EntityType::SmallFireball => "small_fireball",
			EntityType::SnowGolem => "snow_golem",
			EntityType::Snowball => "snowball",
			EntityType::SpectralArrow => "spectral_arrow",
			EntityType::Spider => "spider",
			EntityType::Squid => "squid",
			EntityType::Stray => "stray",
			EntityType::TraderLlama => "trader_llama",
			EntityType::TropicalFish => "tropical_fish",
			EntityType::Turtle => "turtle",
			EntityType::Egg => "egg",
			EntityType::EnderPearl => "ender_pearl",
			EntityType::ExperienceBottle => "experience_bottle",
			EntityType::Potion => "potion",
			EntityType::Trident => "trident",
			EntityType::Vex => "vex",
			EntityType::Villager => "villager",
			EntityType::IronGolem => "iron_golem",
			EntityType::Vindicator => "vindicator",
			EntityType::Pillager => "pillager",
			EntityType::WanderingTrader => "wandering_trader",
			EntityType::Witch => "witch",
			EntityType::Wither => "wither",
			EntityType::WitherSkeleton => "wither_skeleton",
			EntityType::WitherSkull => "wither_skull",
			EntityType::Wolf => "wolf",
			EntityType::Zombie => "zombie",
			EntityType::ZombifiedPiglin => "zombified_piglin",
			EntityType::ZombieHorse => "zombie_horse",
			EntityType::ZombieVillager => "zombie_villager",
			EntityType::Phantom => "phantom",
			EntityType::Ravager => "ravager",
			EntityType::Hoglin => "hoglin",
			EntityType::Piglin => "piglin",
			EntityType::Strider => "strider",
			EntityType::Zoglin => "zoglin",
			EntityType::LightningBolt => "lightning_bolt",
			EntityType::Player => "player",
			EntityType::FishingBobber => "fishing_bobber",
		})
	}
	pub fn get_id(&self) -> i32 {
		return match self {
			EntityType::AreaEffectCloud => AREA_EFFECT_CLOUD,
			EntityType::ArmorStand => ARMOR_STAND,
			EntityType::Arrow => ARROW,
			EntityType::Bat => BAT,
			EntityType::Bee => BEE,
			EntityType::Blaze => BLAZE,
			EntityType::Boat => BOAT,
			EntityType::Cat => CAT,
			EntityType::CaveSpider => CAVE_SPIDER,
			EntityType::Chicken => CHICKEN,
			EntityType::Cod => COD,
			EntityType::Cow => COW,
			EntityType::Creeper => CREEPER,
			EntityType::Donkey => DONKEY,
			EntityType::Dolphin => DOLPHIN,
			EntityType::DragonFireball => DRAGON_FIREBALL,
			EntityType::Drowned => DROWNED,
			EntityType::ElderGuardian => ELDER_GUARDIAN,
			EntityType::EndCrystal => END_CRYSTAL,
			EntityType::EnderDragon => ENDER_DRAGON,
			EntityType::Enderman => ENDERMAN,
			EntityType::Endermite => ENDERMITE,
			EntityType::EvokerFangs => EVOKER_FANGS,
			EntityType::Evoker => EVOKER,
			EntityType::ExperienceOrb => EXPERIENCE_ORB,
			EntityType::EyeOfEnder => EYE_OF_ENDER,
			EntityType::FallingBlock => FALLING_BLOCK,
			EntityType::FireworkRocket => FIREWORK_ROCKET,
			EntityType::Fox => FOX,
			EntityType::Ghast => GHAST,
			EntityType::Giant => GIANT,
			EntityType::Guardian => GUARDIAN,
			EntityType::Horse => HORSE,
			EntityType::Husk => HUSK,
			EntityType::Illusioner => ILLUSIONER,
			EntityType::Item => ITEM,
			EntityType::ItemFrame => ITEM_FRAME,
			EntityType::Fireball => FIREBALL,
			EntityType::LeashKnot => LEASH_KNOT,
			EntityType::Llama => LLAMA,
			EntityType::LlamaSpit => LLAMA_SPIT,
			EntityType::MagmaCube => MAGMA_CUBE,
			EntityType::Minecart => MINECART,
			EntityType::ChestMinecart => CHEST_MINECART,
			EntityType::CommandBlockMinecart => COMMAND_BLOCK_MINECART,
			EntityType::FurnaceMinecart => FURNACE_MINECART,
			EntityType::HopperMinecart => HOPPER_MINECART,
			EntityType::SpawnerMinecart => SPAWNER_MINECART,
			EntityType::TntMinecart => TNT_MINECART,
			EntityType::Mule => MULE,
			EntityType::Mooshroom => MOOSHROOM,
			EntityType::Ocelot => OCELOT,
			EntityType::Painting => PAINTING,
			EntityType::Panda => PANDA,
			EntityType::Parrot => PARROT,
			EntityType::Pig => PIG,
			EntityType::Pufferfish => PUFFERFISH,
			EntityType::PolarBear => POLAR_BEAR,
			EntityType::Tnt => TNT,
			EntityType::Rabbit => RABBIT,
			EntityType::Salmon => SALMON,
			EntityType::Sheep => SHEEP,
			EntityType::Shulker => SHULKER,
			EntityType::ShulkerBullet => SHULKER_BULLET,
			EntityType::Silverfish => SILVERFISH,
			EntityType::Skeleton => SKELETON,
			EntityType::SkeletonHorse => SKELETON_HORSE,
			EntityType::Slime => SLIME,
			EntityType::SmallFireball => SMALL_FIREBALL,
			EntityType::SnowGolem => SNOW_GOLEM,
			EntityType::Snowball => SNOWBALL,
			EntityType::SpectralArrow => SPECTRAL_ARROW,
			EntityType::Spider => SPIDER,
			EntityType::Squid => SQUID,
			EntityType::Stray => STRAY,
			EntityType::TraderLlama => TRADER_LLAMA,
			EntityType::TropicalFish => TROPICAL_FISH,
			EntityType::Turtle => TURTLE,
			EntityType::Egg => EGG,
			EntityType::EnderPearl => ENDER_PEARL,
			EntityType::ExperienceBottle => EXPERIENCE_BOTTLE,
			EntityType::Potion => POTION,
			EntityType::Trident => TRIDENT,
			EntityType::Vex => VEX,
			EntityType::Villager => VILLAGER,
			EntityType::IronGolem => IRON_GOLEM,
			EntityType::Vindicator => VINDICATOR,
			EntityType::Pillager => PILLAGER,
			EntityType::WanderingTrader => WANDERING_TRADER,
			EntityType::Witch => WITCH,
			EntityType::Wither => WITHER,
			EntityType::WitherSkeleton => WITHER_SKELETON,
			EntityType::WitherSkull => WITHER_SKULL,
			EntityType::Wolf => WOLF,
			EntityType::Zombie => ZOMBIE,
			EntityType::ZombifiedPiglin => ZOMBIFIED_PIGLIN,
			EntityType::ZombieHorse => ZOMBIE_HORSE,
			EntityType::ZombieVillager => ZOMBIE_VILLAGER,
			EntityType::Phantom => PHANTOM,
			EntityType::Ravager => RAVAGER,
			EntityType::Hoglin => HOGLIN,
			EntityType::Piglin => PIGLIN,
			EntityType::Strider => STRIDER,
			EntityType::Zoglin => ZOGLIN,
			EntityType::LightningBolt => LIGHTNING_BOLT,
			EntityType::Player => PLAYER,
			EntityType::FishingBobber => FISHING_BOBBER,
		}
	}
}

pub fn get(entity_type: i32) -> EntityType {
	return match entity_type {
		AREA_EFFECT_CLOUD => EntityType::AreaEffectCloud,
		ARMOR_STAND => EntityType::ArmorStand,
		ARROW => EntityType::Arrow,
		BAT => EntityType::Bat,
		BEE => EntityType::Bee,
		BLAZE => EntityType::Blaze,
		BOAT => EntityType::Boat,
		CAT => EntityType::Cat,
		CAVE_SPIDER => EntityType::CaveSpider,
		CHICKEN => EntityType::Chicken,
		COD => EntityType::Cod,
		COW => EntityType::Cow,
		CREEPER => EntityType::Creeper,
		DONKEY => EntityType::Donkey,
		DOLPHIN => EntityType::Dolphin,
		DRAGON_FIREBALL => EntityType::DragonFireball,
		DROWNED => EntityType::Drowned,
		ELDER_GUARDIAN => EntityType::ElderGuardian,
		END_CRYSTAL => EntityType::EndCrystal,
		ENDER_DRAGON => EntityType::EnderDragon,
		ENDERMAN => EntityType::Enderman,
		ENDERMITE => EntityType::Endermite,
		EVOKER_FANGS => EntityType::EvokerFangs,
		EVOKER => EntityType::Evoker,
		EXPERIENCE_ORB => EntityType::ExperienceOrb,
		EYE_OF_ENDER => EntityType::EyeOfEnder,
		FALLING_BLOCK => EntityType::FallingBlock,
		FIREWORK_ROCKET => EntityType::FireworkRocket,
		FOX => EntityType::Fox,
		GHAST => EntityType::Ghast,
		GIANT => EntityType::Giant,
		GUARDIAN => EntityType::Guardian,
		HORSE => EntityType::Horse,
		HUSK => EntityType::Husk,
		ILLUSIONER => EntityType::Illusioner,
		ITEM => EntityType::Item,
		ITEM_FRAME => EntityType::ItemFrame,
		FIREBALL => EntityType::Fireball,
		LEASH_KNOT => EntityType::LeashKnot,
		LLAMA => EntityType::Llama,
		LLAMA_SPIT => EntityType::LlamaSpit,
		MAGMA_CUBE => EntityType::MagmaCube,
		MINECART => EntityType::Minecart,
		CHEST_MINECART => EntityType::ChestMinecart,
		COMMAND_BLOCK_MINECART => EntityType::CommandBlockMinecart,
		FURNACE_MINECART => EntityType::FurnaceMinecart,
		HOPPER_MINECART => EntityType::HopperMinecart,
		SPAWNER_MINECART => EntityType::SpawnerMinecart,
		TNT_MINECART => EntityType::TntMinecart,
		MULE => EntityType::Mule,
		MOOSHROOM => EntityType::Mooshroom,
		OCELOT => EntityType::Ocelot,
		PAINTING => EntityType::Painting,
		PANDA => EntityType::Panda,
		PARROT => EntityType::Parrot,
		PIG => EntityType::Pig,
		PUFFERFISH => EntityType::Pufferfish,
		POLAR_BEAR => EntityType::PolarBear,
		TNT => EntityType::Tnt,
		RABBIT => EntityType::Rabbit,
		SALMON => EntityType::Salmon,
		SHEEP => EntityType::Sheep,
		SHULKER => EntityType::Shulker,
		SHULKER_BULLET => EntityType::ShulkerBullet,
		SILVERFISH => EntityType::Silverfish,
		SKELETON => EntityType::Skeleton,
		SKELETON_HORSE => EntityType::SkeletonHorse,
		SLIME => EntityType::Slime,
		SMALL_FIREBALL => EntityType::SmallFireball,
		SNOW_GOLEM => EntityType::SnowGolem,
		SNOWBALL => EntityType::Snowball,
		SPECTRAL_ARROW => EntityType::SpectralArrow,
		SPIDER => EntityType::Spider,
		SQUID => EntityType::Squid,
		STRAY => EntityType::Stray,
		TRADER_LLAMA => EntityType::TraderLlama,
		TROPICAL_FISH => EntityType::TropicalFish,
		TURTLE => EntityType::Turtle,
		EGG => EntityType::Egg,
		ENDER_PEARL => EntityType::EnderPearl,
		EXPERIENCE_BOTTLE => EntityType::ExperienceBottle,
		POTION => EntityType::Potion,
		TRIDENT => EntityType::Trident,
		VEX => EntityType::Vex,
		VILLAGER => EntityType::Villager,
		IRON_GOLEM => EntityType::IronGolem,
		VINDICATOR => EntityType::Vindicator,
		PILLAGER => EntityType::Pillager,
		WANDERING_TRADER => EntityType::WanderingTrader,
		WITCH => EntityType::Witch,
		WITHER => EntityType::Wither,
		WITHER_SKELETON => EntityType::WitherSkeleton,
		WITHER_SKULL => EntityType::WitherSkull,
		WOLF => EntityType::Wolf,
		ZOMBIE => EntityType::Zombie,
		ZOMBIFIED_PIGLIN => EntityType::ZombifiedPiglin,
		ZOMBIE_HORSE => EntityType::ZombieHorse,
		ZOMBIE_VILLAGER => EntityType::ZombieVillager,
		PHANTOM => EntityType::Phantom,
		RAVAGER => EntityType::Ravager,
		HOGLIN => EntityType::Hoglin,
		PIGLIN => EntityType::Piglin,
		STRIDER => EntityType::Strider,
		ZOGLIN => EntityType::Zoglin,
		LIGHTNING_BOLT => EntityType::LightningBolt,
		PLAYER => EntityType::Player,
		FISHING_BOBBER => EntityType::FishingBobber,
		_ => EntityType::ArmorStand
	}
}

pub fn get_entity_types() -> [EntityType; 107] {
	return [
		EntityType::AreaEffectCloud,
		EntityType::ArmorStand,
		EntityType::Arrow,
		EntityType::Bat,
		EntityType::Bee,
		EntityType::Blaze,
		EntityType::Boat,
		EntityType::Cat,
		EntityType::CaveSpider,
		EntityType::Chicken,
		EntityType::Cod,
		EntityType::Cow,
		EntityType::Creeper,
		EntityType::Donkey,
		EntityType::Dolphin,
		EntityType::DragonFireball,
		EntityType::Drowned,
		EntityType::ElderGuardian,
		EntityType::EndCrystal,
		EntityType::EnderDragon,
		EntityType::Enderman,
		EntityType::Endermite,
		EntityType::EvokerFangs,
		EntityType::Evoker,
		EntityType::ExperienceOrb,
		EntityType::EyeOfEnder,
		EntityType::FallingBlock,
		EntityType::FireworkRocket,
		EntityType::Fox,
		EntityType::Ghast,
		EntityType::Giant,
		EntityType::Guardian,
		EntityType::Horse,
		EntityType::Husk,
		EntityType::Illusioner,
		EntityType::Item,
		EntityType::ItemFrame,
		EntityType::Fireball,
		EntityType::LeashKnot,
		EntityType::Llama,
		EntityType::LlamaSpit,
		EntityType::MagmaCube,
		EntityType::Minecart,
		EntityType::ChestMinecart,
		EntityType::CommandBlockMinecart,
		EntityType::FurnaceMinecart,
		EntityType::HopperMinecart,
		EntityType::SpawnerMinecart,
		EntityType::TntMinecart,
		EntityType::Mule,
		EntityType::Mooshroom,
		EntityType::Ocelot,
		EntityType::Painting,
		EntityType::Panda,
		EntityType::Parrot,
		EntityType::Pig,
		EntityType::Pufferfish,
		EntityType::PolarBear,
		EntityType::Tnt,
		EntityType::Rabbit,
		EntityType::Salmon,
		EntityType::Sheep,
		EntityType::Shulker,
		EntityType::ShulkerBullet,
		EntityType::Silverfish,
		EntityType::Skeleton,
		EntityType::SkeletonHorse,
		EntityType::Slime,
		EntityType::SmallFireball,
		EntityType::SnowGolem,
		EntityType::Snowball,
		EntityType::SpectralArrow,
		EntityType::Spider,
		EntityType::Squid,
		EntityType::Stray,
		EntityType::TraderLlama,
		EntityType::TropicalFish,
		EntityType::Turtle,
		EntityType::Egg,
		EntityType::EnderPearl,
		EntityType::ExperienceBottle,
		EntityType::Potion,
		EntityType::Trident,
		EntityType::Vex,
		EntityType::Villager,
		EntityType::IronGolem,
		EntityType::Vindicator,
		EntityType::Pillager,
		EntityType::WanderingTrader,
		EntityType::Witch,
		EntityType::Wither,
		EntityType::WitherSkeleton,
		EntityType::WitherSkull,
		EntityType::Wolf,
		EntityType::Zombie,
		EntityType::ZombifiedPiglin,
		EntityType::ZombieHorse,
		EntityType::ZombieVillager,
		EntityType::Phantom,
		EntityType::Ravager,
		EntityType::Hoglin,
		EntityType::Piglin,
		EntityType::Strider,
		EntityType::Zoglin,
		EntityType::LightningBolt,
		EntityType::Player,
		EntityType::FishingBobber,
	]
}

pub const AREA_EFFECT_CLOUD: i32 = 0;
pub const ARMOR_STAND: i32 = 1;
pub const ARROW: i32 = 2;
pub const BAT: i32 = 3;
pub const BEE: i32 = 4;
pub const BLAZE: i32 = 5;
pub const BOAT: i32 = 6;
pub const CAT: i32 = 7;
pub const CAVE_SPIDER: i32 = 8;
pub const CHICKEN: i32 = 9;
pub const COD: i32 = 10;
pub const COW: i32 = 11;
pub const CREEPER: i32 = 12;
pub const DONKEY: i32 = 13;
pub const DOLPHIN: i32 = 14;
pub const DRAGON_FIREBALL: i32 = 15;
pub const DROWNED: i32 = 16;
pub const ELDER_GUARDIAN: i32 = 17;
pub const END_CRYSTAL: i32 = 18;
pub const ENDER_DRAGON: i32 = 19;
pub const ENDERMAN: i32 = 20;
pub const ENDERMITE: i32 = 21;
pub const EVOKER_FANGS: i32 = 22;
pub const EVOKER: i32 = 23;
pub const EXPERIENCE_ORB: i32 = 24;
pub const EYE_OF_ENDER: i32 = 25;
pub const FALLING_BLOCK: i32 = 26;
pub const FIREWORK_ROCKET: i32 = 27;
pub const FOX: i32 = 28;
pub const GHAST: i32 = 29;
pub const GIANT: i32 = 30;
pub const GUARDIAN: i32 = 31;
pub const HORSE: i32 = 32;
pub const HUSK: i32 = 33;
pub const ILLUSIONER: i32 = 34;
pub const ITEM: i32 = 35;
pub const ITEM_FRAME: i32 = 36;
pub const FIREBALL: i32 = 37;
pub const LEASH_KNOT: i32 = 38;
pub const LLAMA: i32 = 39;
pub const LLAMA_SPIT: i32 = 40;
pub const MAGMA_CUBE: i32 = 41;
pub const MINECART: i32 = 42;
pub const CHEST_MINECART: i32 = 43;
pub const COMMAND_BLOCK_MINECART: i32 = 44;
pub const FURNACE_MINECART: i32 = 45;
pub const HOPPER_MINECART: i32 = 46;
pub const SPAWNER_MINECART: i32 = 47;
pub const TNT_MINECART: i32 = 48;
pub const MULE: i32 = 49;
pub const MOOSHROOM: i32 = 50;
pub const OCELOT: i32 = 51;
pub const PAINTING: i32 = 52;
pub const PANDA: i32 = 53;
pub const PARROT: i32 = 54;
pub const PIG: i32 = 55;
pub const PUFFERFISH: i32 = 56;
pub const POLAR_BEAR: i32 = 57;
pub const TNT: i32 = 58;
pub const RABBIT: i32 = 59;
pub const SALMON: i32 = 60;
pub const SHEEP: i32 = 61;
pub const SHULKER: i32 = 62;
pub const SHULKER_BULLET: i32 = 63;
pub const SILVERFISH: i32 = 64;
pub const SKELETON: i32 = 65;
pub const SKELETON_HORSE: i32 = 66;
pub const SLIME: i32 = 67;
pub const SMALL_FIREBALL: i32 = 68;
pub const SNOW_GOLEM: i32 = 69;
pub const SNOWBALL: i32 = 70;
pub const SPECTRAL_ARROW: i32 = 71;
pub const SPIDER: i32 = 72;
pub const SQUID: i32 = 73;
pub const STRAY: i32 = 74;
pub const TRADER_LLAMA: i32 = 75;
pub const TROPICAL_FISH: i32 = 76;
pub const TURTLE: i32 = 77;
pub const EGG: i32 = 78;
pub const ENDER_PEARL: i32 = 79;
pub const EXPERIENCE_BOTTLE: i32 = 80;
pub const POTION: i32 = 81;
pub const TRIDENT: i32 = 82;
pub const VEX: i32 = 83;
pub const VILLAGER: i32 = 84;
pub const IRON_GOLEM: i32 = 85;
pub const VINDICATOR: i32 = 86;
pub const PILLAGER: i32 = 87;
pub const WANDERING_TRADER: i32 = 88;
pub const WITCH: i32 = 89;
pub const WITHER: i32 = 90;
pub const WITHER_SKELETON: i32 = 91;
pub const WITHER_SKULL: i32 = 92;
pub const WOLF: i32 = 93;
pub const ZOMBIE: i32 = 94;
pub const ZOMBIFIED_PIGLIN: i32 = 95;
pub const ZOMBIE_HORSE: i32 = 96;
pub const ZOMBIE_VILLAGER: i32 = 97;
pub const PHANTOM: i32 = 98;
pub const RAVAGER: i32 = 99;
pub const HOGLIN: i32 = 100;
pub const PIGLIN: i32 = 101;
pub const STRIDER: i32 = 102;
pub const ZOGLIN: i32 = 103;
pub const LIGHTNING_BOLT: i32 = 104;
pub const PLAYER: i32 = 105;
pub const FISHING_BOBBER: i32 = 106;