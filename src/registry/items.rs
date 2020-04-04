#![allow(dead_code)]

use crate::inventory::item::Item;
use crate::registry::identifier::Identifier;

pub fn get_items() -> [Item; 947] {
	[
		Item::new(AIR),
		Item::new(STONE),
		Item::new(GRANITE),
		Item::new(POLISHED_GRANITE),
		Item::new(DIORITE),
		Item::new(POLISHED_DIORITE),
		Item::new(ANDESITE),
		Item::new(POLISHED_ANDESITE),
		Item::new(GRASS_BLOCK),
		Item::new(DIRT),
		Item::new(COARSE_DIRT),
		Item::new(PODZOL),
		Item::new(CRIMSON_NYLIUM),
		Item::new(WARPED_NYLIUM),
		Item::new(COBBLESTONE),
		Item::new(OAK_PLANKS),
		Item::new(SPRUCE_PLANKS),
		Item::new(BIRCH_PLANKS),
		Item::new(JUNGLE_PLANKS),
		Item::new(ACACIA_PLANKS),
		Item::new(DARK_OAK_PLANKS),
		Item::new(CRIMSON_PLANKS),
		Item::new(WARPED_PLANKS),
		Item::new(OAK_SAPLING),
		Item::new(SPRUCE_SAPLING),
		Item::new(BIRCH_SAPLING),
		Item::new(JUNGLE_SAPLING),
		Item::new(ACACIA_SAPLING),
		Item::new(DARK_OAK_SAPLING),
		Item::new(BEDROCK),
		Item::new(SAND),
		Item::new(RED_SAND),
		Item::new(GRAVEL),
		Item::new(GOLD_ORE),
		Item::new(IRON_ORE),
		Item::new(COAL_ORE),
		Item::new(NETHER_GOLD_ORE),
		Item::new(OAK_LOG),
		Item::new(SPRUCE_LOG),
		Item::new(BIRCH_LOG),
		Item::new(JUNGLE_LOG),
		Item::new(ACACIA_LOG),
		Item::new(DARK_OAK_LOG),
		Item::new(CRIMSON_STEM),
		Item::new(WARPED_STEM),
		Item::new(STRIPPED_OAK_LOG),
		Item::new(STRIPPED_SPRUCE_LOG),
		Item::new(STRIPPED_BIRCH_LOG),
		Item::new(STRIPPED_JUNGLE_LOG),
		Item::new(STRIPPED_ACACIA_LOG),
		Item::new(STRIPPED_DARK_OAK_LOG),
		Item::new(STRIPPED_CRIMSON_STEM),
		Item::new(STRIPPED_WARPED_STEM),
		Item::new(STRIPPED_OAK_WOOD),
		Item::new(STRIPPED_SPRUCE_WOOD),
		Item::new(STRIPPED_BIRCH_WOOD),
		Item::new(STRIPPED_JUNGLE_WOOD),
		Item::new(STRIPPED_ACACIA_WOOD),
		Item::new(STRIPPED_DARK_OAK_WOOD),
		Item::new(STRIPPED_CRIMSON_HYPHAE),
		Item::new(STRIPPED_WARPED_HYPHAE),
		Item::new(OAK_WOOD),
		Item::new(SPRUCE_WOOD),
		Item::new(BIRCH_WOOD),
		Item::new(JUNGLE_WOOD),
		Item::new(ACACIA_WOOD),
		Item::new(DARK_OAK_WOOD),
		Item::new(CRIMSON_HYPHAE),
		Item::new(WARPED_HYPHAE),
		Item::new(OAK_LEAVES),
		Item::new(SPRUCE_LEAVES),
		Item::new(BIRCH_LEAVES),
		Item::new(JUNGLE_LEAVES),
		Item::new(ACACIA_LEAVES),
		Item::new(DARK_OAK_LEAVES),
		Item::new(SPONGE),
		Item::new(WET_SPONGE),
		Item::new(GLASS),
		Item::new(LAPIS_ORE),
		Item::new(LAPIS_BLOCK),
		Item::new(DISPENSER),
		Item::new(SANDSTONE),
		Item::new(CHISELED_SANDSTONE),
		Item::new(CUT_SANDSTONE),
		Item::new(NOTE_BLOCK),
		Item::new(POWERED_RAIL),
		Item::new(DETECTOR_RAIL),
		Item::new(STICKY_PISTON),
		Item::new(COBWEB),
		Item::new(GRASS),
		Item::new(FERN),
		Item::new(DEAD_BUSH),
		Item::new(SEAGRASS),
		Item::new(SEA_PICKLE),
		Item::new(PISTON),
		Item::new(WHITE_WOOL),
		Item::new(ORANGE_WOOL),
		Item::new(MAGENTA_WOOL),
		Item::new(LIGHT_BLUE_WOOL),
		Item::new(YELLOW_WOOL),
		Item::new(LIME_WOOL),
		Item::new(PINK_WOOL),
		Item::new(GRAY_WOOL),
		Item::new(LIGHT_GRAY_WOOL),
		Item::new(CYAN_WOOL),
		Item::new(PURPLE_WOOL),
		Item::new(BLUE_WOOL),
		Item::new(BROWN_WOOL),
		Item::new(GREEN_WOOL),
		Item::new(RED_WOOL),
		Item::new(BLACK_WOOL),
		Item::new(DANDELION),
		Item::new(POPPY),
		Item::new(BLUE_ORCHID),
		Item::new(ALLIUM),
		Item::new(AZURE_BLUET),
		Item::new(RED_TULIP),
		Item::new(ORANGE_TULIP),
		Item::new(WHITE_TULIP),
		Item::new(PINK_TULIP),
		Item::new(OXEYE_DAISY),
		Item::new(CORNFLOWER),
		Item::new(LILY_OF_THE_VALLEY),
		Item::new(WITHER_ROSE),
		Item::new(BROWN_MUSHROOM),
		Item::new(RED_MUSHROOM),
		Item::new(CRIMSON_FUNGUS),
		Item::new(WARPED_FUNGUS),
		Item::new(CRIMSON_ROOTS),
		Item::new(WARPED_ROOTS),
		Item::new(NETHER_SPROUTS),
		Item::new(WEEPING_VINES),
		Item::new(TWISTING_VINES),
		Item::new(GOLD_BLOCK),
		Item::new(IRON_BLOCK),
		Item::new(OAK_SLAB),
		Item::new(SPRUCE_SLAB),
		Item::new(BIRCH_SLAB),
		Item::new(JUNGLE_SLAB),
		Item::new(ACACIA_SLAB),
		Item::new(DARK_OAK_SLAB),
		Item::new(CRIMSON_SLAB),
		Item::new(WARPED_SLAB),
		Item::new(STONE_SLAB),
		Item::new(SMOOTH_STONE_SLAB),
		Item::new(SANDSTONE_SLAB),
		Item::new(CUT_SANDSTONE_SLAB),
		Item::new(PETRIFIED_OAK_SLAB),
		Item::new(COBBLESTONE_SLAB),
		Item::new(BRICK_SLAB),
		Item::new(STONE_BRICK_SLAB),
		Item::new(NETHER_BRICK_SLAB),
		Item::new(QUARTZ_SLAB),
		Item::new(RED_SANDSTONE_SLAB),
		Item::new(CUT_RED_SANDSTONE_SLAB),
		Item::new(PURPUR_SLAB),
		Item::new(PRISMARINE_SLAB),
		Item::new(PRISMARINE_BRICK_SLAB),
		Item::new(DARK_PRISMARINE_SLAB),
		Item::new(SMOOTH_QUARTZ),
		Item::new(SMOOTH_RED_SANDSTONE),
		Item::new(SMOOTH_SANDSTONE),
		Item::new(SMOOTH_STONE),
		Item::new(BRICKS),
		Item::new(TNT),
		Item::new(BOOKSHELF),
		Item::new(MOSSY_COBBLESTONE),
		Item::new(OBSIDIAN),
		Item::new(TORCH),
		Item::new(END_ROD),
		Item::new(CHORUS_PLANT),
		Item::new(CHORUS_FLOWER),
		Item::new(PURPUR_BLOCK),
		Item::new(PURPUR_PILLAR),
		Item::new(PURPUR_STAIRS),
		Item::new(SPAWNER),
		Item::new(OAK_STAIRS),
		Item::new(CHEST),
		Item::new(DIAMOND_ORE),
		Item::new(DIAMOND_BLOCK),
		Item::new(CRAFTING_TABLE),
		Item::new(FARMLAND),
		Item::new(FURNACE),
		Item::new(LADDER),
		Item::new(RAIL),
		Item::new(COBBLESTONE_STAIRS),
		Item::new(LEVER),
		Item::new(STONE_PRESSURE_PLATE),
		Item::new(OAK_PRESSURE_PLATE),
		Item::new(SPRUCE_PRESSURE_PLATE),
		Item::new(BIRCH_PRESSURE_PLATE),
		Item::new(JUNGLE_PRESSURE_PLATE),
		Item::new(ACACIA_PRESSURE_PLATE),
		Item::new(DARK_OAK_PRESSURE_PLATE),
		Item::new(CRIMSON_PRESSURE_PLATE),
		Item::new(WARPED_PRESSURE_PLATE),
		Item::new(REDSTONE_ORE),
		Item::new(REDSTONE_TORCH),
		Item::new(STONE_BUTTON),
		Item::new(SNOW),
		Item::new(ICE),
		Item::new(SNOW_BLOCK),
		Item::new(CACTUS),
		Item::new(CLAY),
		Item::new(JUKEBOX),
		Item::new(OAK_FENCE),
		Item::new(SPRUCE_FENCE),
		Item::new(BIRCH_FENCE),
		Item::new(JUNGLE_FENCE),
		Item::new(ACACIA_FENCE),
		Item::new(DARK_OAK_FENCE),
		Item::new(CRIMSON_FENCE),
		Item::new(WARPED_FENCE),
		Item::new(PUMPKIN),
		Item::new(CARVED_PUMPKIN),
		Item::new(NETHERRACK),
		Item::new(SOUL_SAND),
		Item::new(SOUL_SOIL),
		Item::new(BASALT),
		Item::new(POLISHED_BASALT),
		Item::new(SOUL_FIRE_TORCH),
		Item::new(GLOWSTONE),
		Item::new(JACK_O_LANTERN),
		Item::new(OAK_TRAPDOOR),
		Item::new(SPRUCE_TRAPDOOR),
		Item::new(BIRCH_TRAPDOOR),
		Item::new(JUNGLE_TRAPDOOR),
		Item::new(ACACIA_TRAPDOOR),
		Item::new(DARK_OAK_TRAPDOOR),
		Item::new(CRIMSON_TRAPDOOR),
		Item::new(WARPED_TRAPDOOR),
		Item::new(INFESTED_STONE),
		Item::new(INFESTED_COBBLESTONE),
		Item::new(INFESTED_STONE_BRICKS),
		Item::new(INFESTED_MOSSY_STONE_BRICKS),
		Item::new(INFESTED_CRACKED_STONE_BRICKS),
		Item::new(INFESTED_CHISELED_STONE_BRICKS),
		Item::new(STONE_BRICKS),
		Item::new(MOSSY_STONE_BRICKS),
		Item::new(CRACKED_STONE_BRICKS),
		Item::new(CHISELED_STONE_BRICKS),
		Item::new(BROWN_MUSHROOM_BLOCK),
		Item::new(RED_MUSHROOM_BLOCK),
		Item::new(MUSHROOM_STEM),
		Item::new(IRON_BARS),
		Item::new(GLASS_PANE),
		Item::new(MELON),
		Item::new(VINE),
		Item::new(OAK_FENCE_GATE),
		Item::new(SPRUCE_FENCE_GATE),
		Item::new(BIRCH_FENCE_GATE),
		Item::new(JUNGLE_FENCE_GATE),
		Item::new(ACACIA_FENCE_GATE),
		Item::new(DARK_OAK_FENCE_GATE),
		Item::new(CRIMSON_FENCE_GATE),
		Item::new(WARPED_FENCE_GATE),
		Item::new(BRICK_STAIRS),
		Item::new(STONE_BRICK_STAIRS),
		Item::new(MYCELIUM),
		Item::new(LILY_PAD),
		Item::new(NETHER_BRICKS),
		Item::new(NETHER_BRICK_FENCE),
		Item::new(NETHER_BRICK_STAIRS),
		Item::new(ENCHANTING_TABLE),
		Item::new(END_PORTAL_FRAME),
		Item::new(END_STONE),
		Item::new(END_STONE_BRICKS),
		Item::new(DRAGON_EGG),
		Item::new(REDSTONE_LAMP),
		Item::new(SANDSTONE_STAIRS),
		Item::new(EMERALD_ORE),
		Item::new(ENDER_CHEST),
		Item::new(TRIPWIRE_HOOK),
		Item::new(EMERALD_BLOCK),
		Item::new(SPRUCE_STAIRS),
		Item::new(BIRCH_STAIRS),
		Item::new(JUNGLE_STAIRS),
		Item::new(CRIMSON_STAIRS),
		Item::new(WARPED_STAIRS),
		Item::new(COMMAND_BLOCK),
		Item::new(BEACON),
		Item::new(COBBLESTONE_WALL),
		Item::new(MOSSY_COBBLESTONE_WALL),
		Item::new(BRICK_WALL),
		Item::new(PRISMARINE_WALL),
		Item::new(RED_SANDSTONE_WALL),
		Item::new(MOSSY_STONE_BRICK_WALL),
		Item::new(GRANITE_WALL),
		Item::new(STONE_BRICK_WALL),
		Item::new(NETHER_BRICK_WALL),
		Item::new(ANDESITE_WALL),
		Item::new(RED_NETHER_BRICK_WALL),
		Item::new(SANDSTONE_WALL),
		Item::new(END_STONE_BRICK_WALL),
		Item::new(DIORITE_WALL),
		Item::new(OAK_BUTTON),
		Item::new(SPRUCE_BUTTON),
		Item::new(BIRCH_BUTTON),
		Item::new(JUNGLE_BUTTON),
		Item::new(ACACIA_BUTTON),
		Item::new(DARK_OAK_BUTTON),
		Item::new(CRIMSON_BUTTON),
		Item::new(WARPED_BUTTON),
		Item::new(ANVIL),
		Item::new(CHIPPED_ANVIL),
		Item::new(DAMAGED_ANVIL),
		Item::new(TRAPPED_CHEST),
		Item::new(LIGHT_WEIGHTED_PRESSURE_PLATE),
		Item::new(HEAVY_WEIGHTED_PRESSURE_PLATE),
		Item::new(DAYLIGHT_DETECTOR),
		Item::new(REDSTONE_BLOCK),
		Item::new(NETHER_QUARTZ_ORE),
		Item::new(HOPPER),
		Item::new(CHISELED_QUARTZ_BLOCK),
		Item::new(QUARTZ_BLOCK),
		Item::new(QUARTZ_PILLAR),
		Item::new(QUARTZ_STAIRS),
		Item::new(ACTIVATOR_RAIL),
		Item::new(DROPPER),
		Item::new(WHITE_TERRACOTTA),
		Item::new(ORANGE_TERRACOTTA),
		Item::new(MAGENTA_TERRACOTTA),
		Item::new(LIGHT_BLUE_TERRACOTTA),
		Item::new(YELLOW_TERRACOTTA),
		Item::new(LIME_TERRACOTTA),
		Item::new(PINK_TERRACOTTA),
		Item::new(GRAY_TERRACOTTA),
		Item::new(LIGHT_GRAY_TERRACOTTA),
		Item::new(CYAN_TERRACOTTA),
		Item::new(PURPLE_TERRACOTTA),
		Item::new(BLUE_TERRACOTTA),
		Item::new(BROWN_TERRACOTTA),
		Item::new(GREEN_TERRACOTTA),
		Item::new(RED_TERRACOTTA),
		Item::new(BLACK_TERRACOTTA),
		Item::new(BARRIER),
		Item::new(IRON_TRAPDOOR),
		Item::new(HAY_BLOCK),
		Item::new(WHITE_CARPET),
		Item::new(ORANGE_CARPET),
		Item::new(MAGENTA_CARPET),
		Item::new(LIGHT_BLUE_CARPET),
		Item::new(YELLOW_CARPET),
		Item::new(LIME_CARPET),
		Item::new(PINK_CARPET),
		Item::new(GRAY_CARPET),
		Item::new(LIGHT_GRAY_CARPET),
		Item::new(CYAN_CARPET),
		Item::new(PURPLE_CARPET),
		Item::new(BLUE_CARPET),
		Item::new(BROWN_CARPET),
		Item::new(GREEN_CARPET),
		Item::new(RED_CARPET),
		Item::new(BLACK_CARPET),
		Item::new(TERRACOTTA),
		Item::new(COAL_BLOCK),
		Item::new(PACKED_ICE),
		Item::new(ACACIA_STAIRS),
		Item::new(DARK_OAK_STAIRS),
		Item::new(SLIME_BLOCK),
		Item::new(GRASS_PATH),
		Item::new(SUNFLOWER),
		Item::new(LILAC),
		Item::new(ROSE_BUSH),
		Item::new(PEONY),
		Item::new(TALL_GRASS),
		Item::new(LARGE_FERN),
		Item::new(WHITE_STAINED_GLASS),
		Item::new(ORANGE_STAINED_GLASS),
		Item::new(MAGENTA_STAINED_GLASS),
		Item::new(LIGHT_BLUE_STAINED_GLASS),
		Item::new(YELLOW_STAINED_GLASS),
		Item::new(LIME_STAINED_GLASS),
		Item::new(PINK_STAINED_GLASS),
		Item::new(GRAY_STAINED_GLASS),
		Item::new(LIGHT_GRAY_STAINED_GLASS),
		Item::new(CYAN_STAINED_GLASS),
		Item::new(PURPLE_STAINED_GLASS),
		Item::new(BLUE_STAINED_GLASS),
		Item::new(BROWN_STAINED_GLASS),
		Item::new(GREEN_STAINED_GLASS),
		Item::new(RED_STAINED_GLASS),
		Item::new(BLACK_STAINED_GLASS),
		Item::new(WHITE_STAINED_GLASS_PANE),
		Item::new(ORANGE_STAINED_GLASS_PANE),
		Item::new(MAGENTA_STAINED_GLASS_PANE),
		Item::new(LIGHT_BLUE_STAINED_GLASS_PANE),
		Item::new(YELLOW_STAINED_GLASS_PANE),
		Item::new(LIME_STAINED_GLASS_PANE),
		Item::new(PINK_STAINED_GLASS_PANE),
		Item::new(GRAY_STAINED_GLASS_PANE),
		Item::new(LIGHT_GRAY_STAINED_GLASS_PANE),
		Item::new(CYAN_STAINED_GLASS_PANE),
		Item::new(PURPLE_STAINED_GLASS_PANE),
		Item::new(BLUE_STAINED_GLASS_PANE),
		Item::new(BROWN_STAINED_GLASS_PANE),
		Item::new(GREEN_STAINED_GLASS_PANE),
		Item::new(RED_STAINED_GLASS_PANE),
		Item::new(BLACK_STAINED_GLASS_PANE),
		Item::new(PRISMARINE),
		Item::new(PRISMARINE_BRICKS),
		Item::new(DARK_PRISMARINE),
		Item::new(PRISMARINE_STAIRS),
		Item::new(PRISMARINE_BRICK_STAIRS),
		Item::new(DARK_PRISMARINE_STAIRS),
		Item::new(SEA_LANTERN),
		Item::new(RED_SANDSTONE),
		Item::new(CHISELED_RED_SANDSTONE),
		Item::new(CUT_RED_SANDSTONE),
		Item::new(RED_SANDSTONE_STAIRS),
		Item::new(REPEATING_COMMAND_BLOCK),
		Item::new(CHAIN_COMMAND_BLOCK),
		Item::new(MAGMA_BLOCK),
		Item::new(NETHER_WART_BLOCK),
		Item::new(WARPED_WART_BLOCK),
		Item::new(RED_NETHER_BRICKS),
		Item::new(BONE_BLOCK),
		Item::new(STRUCTURE_VOID),
		Item::new(OBSERVER),
		Item::new(SHULKER_BOX),
		Item::new(WHITE_SHULKER_BOX),
		Item::new(ORANGE_SHULKER_BOX),
		Item::new(MAGENTA_SHULKER_BOX),
		Item::new(LIGHT_BLUE_SHULKER_BOX),
		Item::new(YELLOW_SHULKER_BOX),
		Item::new(LIME_SHULKER_BOX),
		Item::new(PINK_SHULKER_BOX),
		Item::new(GRAY_SHULKER_BOX),
		Item::new(LIGHT_GRAY_SHULKER_BOX),
		Item::new(CYAN_SHULKER_BOX),
		Item::new(PURPLE_SHULKER_BOX),
		Item::new(BLUE_SHULKER_BOX),
		Item::new(BROWN_SHULKER_BOX),
		Item::new(GREEN_SHULKER_BOX),
		Item::new(RED_SHULKER_BOX),
		Item::new(BLACK_SHULKER_BOX),
		Item::new(WHITE_GLAZED_TERRACOTTA),
		Item::new(ORANGE_GLAZED_TERRACOTTA),
		Item::new(MAGENTA_GLAZED_TERRACOTTA),
		Item::new(LIGHT_BLUE_GLAZED_TERRACOTTA),
		Item::new(YELLOW_GLAZED_TERRACOTTA),
		Item::new(LIME_GLAZED_TERRACOTTA),
		Item::new(PINK_GLAZED_TERRACOTTA),
		Item::new(GRAY_GLAZED_TERRACOTTA),
		Item::new(LIGHT_GRAY_GLAZED_TERRACOTTA),
		Item::new(CYAN_GLAZED_TERRACOTTA),
		Item::new(PURPLE_GLAZED_TERRACOTTA),
		Item::new(BLUE_GLAZED_TERRACOTTA),
		Item::new(BROWN_GLAZED_TERRACOTTA),
		Item::new(GREEN_GLAZED_TERRACOTTA),
		Item::new(RED_GLAZED_TERRACOTTA),
		Item::new(BLACK_GLAZED_TERRACOTTA),
		Item::new(WHITE_CONCRETE),
		Item::new(ORANGE_CONCRETE),
		Item::new(MAGENTA_CONCRETE),
		Item::new(LIGHT_BLUE_CONCRETE),
		Item::new(YELLOW_CONCRETE),
		Item::new(LIME_CONCRETE),
		Item::new(PINK_CONCRETE),
		Item::new(GRAY_CONCRETE),
		Item::new(LIGHT_GRAY_CONCRETE),
		Item::new(CYAN_CONCRETE),
		Item::new(PURPLE_CONCRETE),
		Item::new(BLUE_CONCRETE),
		Item::new(BROWN_CONCRETE),
		Item::new(GREEN_CONCRETE),
		Item::new(RED_CONCRETE),
		Item::new(BLACK_CONCRETE),
		Item::new(WHITE_CONCRETE_POWDER),
		Item::new(ORANGE_CONCRETE_POWDER),
		Item::new(MAGENTA_CONCRETE_POWDER),
		Item::new(LIGHT_BLUE_CONCRETE_POWDER),
		Item::new(YELLOW_CONCRETE_POWDER),
		Item::new(LIME_CONCRETE_POWDER),
		Item::new(PINK_CONCRETE_POWDER),
		Item::new(GRAY_CONCRETE_POWDER),
		Item::new(LIGHT_GRAY_CONCRETE_POWDER),
		Item::new(CYAN_CONCRETE_POWDER),
		Item::new(PURPLE_CONCRETE_POWDER),
		Item::new(BLUE_CONCRETE_POWDER),
		Item::new(BROWN_CONCRETE_POWDER),
		Item::new(GREEN_CONCRETE_POWDER),
		Item::new(RED_CONCRETE_POWDER),
		Item::new(BLACK_CONCRETE_POWDER),
		Item::new(TURTLE_EGG),
		Item::new(DEAD_TUBE_CORAL_BLOCK),
		Item::new(DEAD_BRAIN_CORAL_BLOCK),
		Item::new(DEAD_BUBBLE_CORAL_BLOCK),
		Item::new(DEAD_FIRE_CORAL_BLOCK),
		Item::new(DEAD_HORN_CORAL_BLOCK),
		Item::new(TUBE_CORAL_BLOCK),
		Item::new(BRAIN_CORAL_BLOCK),
		Item::new(BUBBLE_CORAL_BLOCK),
		Item::new(FIRE_CORAL_BLOCK),
		Item::new(HORN_CORAL_BLOCK),
		Item::new(TUBE_CORAL),
		Item::new(BRAIN_CORAL),
		Item::new(BUBBLE_CORAL),
		Item::new(FIRE_CORAL),
		Item::new(HORN_CORAL),
		Item::new(DEAD_BRAIN_CORAL),
		Item::new(DEAD_BUBBLE_CORAL),
		Item::new(DEAD_FIRE_CORAL),
		Item::new(DEAD_HORN_CORAL),
		Item::new(DEAD_TUBE_CORAL),
		Item::new(TUBE_CORAL_FAN),
		Item::new(BRAIN_CORAL_FAN),
		Item::new(BUBBLE_CORAL_FAN),
		Item::new(FIRE_CORAL_FAN),
		Item::new(HORN_CORAL_FAN),
		Item::new(DEAD_TUBE_CORAL_FAN),
		Item::new(DEAD_BRAIN_CORAL_FAN),
		Item::new(DEAD_BUBBLE_CORAL_FAN),
		Item::new(DEAD_FIRE_CORAL_FAN),
		Item::new(DEAD_HORN_CORAL_FAN),
		Item::new(BLUE_ICE),
		Item::new(CONDUIT),
		Item::new(POLISHED_GRANITE_STAIRS),
		Item::new(SMOOTH_RED_SANDSTONE_STAIRS),
		Item::new(MOSSY_STONE_BRICK_STAIRS),
		Item::new(POLISHED_DIORITE_STAIRS),
		Item::new(MOSSY_COBBLESTONE_STAIRS),
		Item::new(END_STONE_BRICK_STAIRS),
		Item::new(STONE_STAIRS),
		Item::new(SMOOTH_SANDSTONE_STAIRS),
		Item::new(SMOOTH_QUARTZ_STAIRS),
		Item::new(GRANITE_STAIRS),
		Item::new(ANDESITE_STAIRS),
		Item::new(RED_NETHER_BRICK_STAIRS),
		Item::new(POLISHED_ANDESITE_STAIRS),
		Item::new(DIORITE_STAIRS),
		Item::new(POLISHED_GRANITE_SLAB),
		Item::new(SMOOTH_RED_SANDSTONE_SLAB),
		Item::new(MOSSY_STONE_BRICK_SLAB),
		Item::new(POLISHED_DIORITE_SLAB),
		Item::new(MOSSY_COBBLESTONE_SLAB),
		Item::new(END_STONE_BRICK_SLAB),
		Item::new(SMOOTH_SANDSTONE_SLAB),
		Item::new(SMOOTH_QUARTZ_SLAB),
		Item::new(GRANITE_SLAB),
		Item::new(ANDESITE_SLAB),
		Item::new(RED_NETHER_BRICK_SLAB),
		Item::new(POLISHED_ANDESITE_SLAB),
		Item::new(DIORITE_SLAB),
		Item::new(SCAFFOLDING),
		Item::new(IRON_DOOR),
		Item::new(OAK_DOOR),
		Item::new(SPRUCE_DOOR),
		Item::new(BIRCH_DOOR),
		Item::new(JUNGLE_DOOR),
		Item::new(ACACIA_DOOR),
		Item::new(DARK_OAK_DOOR),
		Item::new(CRIMSON_DOOR),
		Item::new(WARPED_DOOR),
		Item::new(REPEATER),
		Item::new(COMPARATOR),
		Item::new(STRUCTURE_BLOCK),
		Item::new(JIGSAW),
		Item::new(TURTLE_HELMET),
		Item::new(SCUTE),
		Item::new(IRON_SHOVEL),
		Item::new(IRON_PICKAXE),
		Item::new(IRON_AXE),
		Item::new(FLINT_AND_STEEL),
		Item::new(APPLE),
		Item::new(BOW),
		Item::new(ARROW),
		Item::new(COAL),
		Item::new(CHARCOAL),
		Item::new(DIAMOND),
		Item::new(IRON_INGOT),
		Item::new(GOLD_INGOT),
		Item::new(IRON_SWORD),
		Item::new(WOODEN_SWORD),
		Item::new(WOODEN_SHOVEL),
		Item::new(WOODEN_PICKAXE),
		Item::new(WOODEN_AXE),
		Item::new(STONE_SWORD),
		Item::new(STONE_SHOVEL),
		Item::new(STONE_PICKAXE),
		Item::new(STONE_AXE),
		Item::new(DIAMOND_SWORD),
		Item::new(DIAMOND_SHOVEL),
		Item::new(DIAMOND_PICKAXE),
		Item::new(DIAMOND_AXE),
		Item::new(STICK),
		Item::new(BOWL),
		Item::new(MUSHROOM_STEW),
		Item::new(GOLDEN_SWORD),
		Item::new(GOLDEN_SHOVEL),
		Item::new(GOLDEN_PICKAXE),
		Item::new(GOLDEN_AXE),
		Item::new(NETHERITE_SWORD),
		Item::new(NETHERITE_SHOVEL),
		Item::new(NETHERITE_PICKAXE),
		Item::new(NETHERITE_AXE),
		Item::new(STRING),
		Item::new(FEATHER),
		Item::new(GUNPOWDER),
		Item::new(WOODEN_HOE),
		Item::new(STONE_HOE),
		Item::new(IRON_HOE),
		Item::new(DIAMOND_HOE),
		Item::new(GOLDEN_HOE),
		Item::new(NETHERITE_HOE),
		Item::new(WHEAT_SEEDS),
		Item::new(WHEAT),
		Item::new(BREAD),
		Item::new(LEATHER_HELMET),
		Item::new(LEATHER_CHESTPLATE),
		Item::new(LEATHER_LEGGINGS),
		Item::new(LEATHER_BOOTS),
		Item::new(CHAINMAIL_HELMET),
		Item::new(CHAINMAIL_CHESTPLATE),
		Item::new(CHAINMAIL_LEGGINGS),
		Item::new(CHAINMAIL_BOOTS),
		Item::new(IRON_HELMET),
		Item::new(IRON_CHESTPLATE),
		Item::new(IRON_LEGGINGS),
		Item::new(IRON_BOOTS),
		Item::new(DIAMOND_HELMET),
		Item::new(DIAMOND_CHESTPLATE),
		Item::new(DIAMOND_LEGGINGS),
		Item::new(DIAMOND_BOOTS),
		Item::new(GOLDEN_HELMET),
		Item::new(GOLDEN_CHESTPLATE),
		Item::new(GOLDEN_LEGGINGS),
		Item::new(GOLDEN_BOOTS),
		Item::new(NETHERITE_HELMET),
		Item::new(NETHERITE_CHESTPLATE),
		Item::new(NETHERITE_LEGGINGS),
		Item::new(NETHERITE_BOOTS),
		Item::new(FLINT),
		Item::new(PORKCHOP),
		Item::new(COOKED_PORKCHOP),
		Item::new(PAINTING),
		Item::new(GOLDEN_APPLE),
		Item::new(ENCHANTED_GOLDEN_APPLE),
		Item::new(OAK_SIGN),
		Item::new(SPRUCE_SIGN),
		Item::new(BIRCH_SIGN),
		Item::new(JUNGLE_SIGN),
		Item::new(ACACIA_SIGN),
		Item::new(DARK_OAK_SIGN),
		Item::new(CRIMSON_SIGN),
		Item::new(WARPED_SIGN),
		Item::new(BUCKET),
		Item::new(WATER_BUCKET),
		Item::new(LAVA_BUCKET),
		Item::new(MINECART),
		Item::new(SADDLE),
		Item::new(REDSTONE),
		Item::new(SNOWBALL),
		Item::new(OAK_BOAT),
		Item::new(LEATHER),
		Item::new(MILK_BUCKET),
		Item::new(PUFFERFISH_BUCKET),
		Item::new(SALMON_BUCKET),
		Item::new(COD_BUCKET),
		Item::new(TROPICAL_FISH_BUCKET),
		Item::new(BRICK),
		Item::new(CLAY_BALL),
		Item::new(SUGAR_CANE),
		Item::new(KELP),
		Item::new(DRIED_KELP_BLOCK),
		Item::new(BAMBOO),
		Item::new(PAPER),
		Item::new(BOOK),
		Item::new(SLIME_BALL),
		Item::new(CHEST_MINECART),
		Item::new(FURNACE_MINECART),
		Item::new(EGG),
		Item::new(COMPASS),
		Item::new(FISHING_ROD),
		Item::new(CLOCK),
		Item::new(GLOWSTONE_DUST),
		Item::new(COD),
		Item::new(SALMON),
		Item::new(TROPICAL_FISH),
		Item::new(PUFFERFISH),
		Item::new(COOKED_COD),
		Item::new(COOKED_SALMON),
		Item::new(INK_SAC),
		Item::new(RED_DYE),
		Item::new(GREEN_DYE),
		Item::new(COCOA_BEANS),
		Item::new(LAPIS_LAZULI),
		Item::new(PURPLE_DYE),
		Item::new(CYAN_DYE),
		Item::new(LIGHT_GRAY_DYE),
		Item::new(GRAY_DYE),
		Item::new(PINK_DYE),
		Item::new(LIME_DYE),
		Item::new(YELLOW_DYE),
		Item::new(LIGHT_BLUE_DYE),
		Item::new(MAGENTA_DYE),
		Item::new(ORANGE_DYE),
		Item::new(BONE_MEAL),
		Item::new(BLUE_DYE),
		Item::new(BROWN_DYE),
		Item::new(BLACK_DYE),
		Item::new(WHITE_DYE),
		Item::new(BONE),
		Item::new(SUGAR),
		Item::new(CAKE),
		Item::new(WHITE_BED),
		Item::new(ORANGE_BED),
		Item::new(MAGENTA_BED),
		Item::new(LIGHT_BLUE_BED),
		Item::new(YELLOW_BED),
		Item::new(LIME_BED),
		Item::new(PINK_BED),
		Item::new(GRAY_BED),
		Item::new(LIGHT_GRAY_BED),
		Item::new(CYAN_BED),
		Item::new(PURPLE_BED),
		Item::new(BLUE_BED),
		Item::new(BROWN_BED),
		Item::new(GREEN_BED),
		Item::new(RED_BED),
		Item::new(BLACK_BED),
		Item::new(COOKIE),
		Item::new(FILLED_MAP),
		Item::new(SHEARS),
		Item::new(MELON_SLICE),
		Item::new(DRIED_KELP),
		Item::new(PUMPKIN_SEEDS),
		Item::new(MELON_SEEDS),
		Item::new(BEEF),
		Item::new(COOKED_BEEF),
		Item::new(CHICKEN),
		Item::new(COOKED_CHICKEN),
		Item::new(ROTTEN_FLESH),
		Item::new(ENDER_PEARL),
		Item::new(BLAZE_ROD),
		Item::new(GHAST_TEAR),
		Item::new(GOLD_NUGGET),
		Item::new(NETHER_WART),
		Item::new(POTION),
		Item::new(GLASS_BOTTLE),
		Item::new(SPIDER_EYE),
		Item::new(FERMENTED_SPIDER_EYE),
		Item::new(BLAZE_POWDER),
		Item::new(MAGMA_CREAM),
		Item::new(BREWING_STAND),
		Item::new(CAULDRON),
		Item::new(ENDER_EYE),
		Item::new(GLISTERING_MELON_SLICE),
		Item::new(BAT_SPAWN_EGG),
		Item::new(BEE_SPAWN_EGG),
		Item::new(BLAZE_SPAWN_EGG),
		Item::new(CAT_SPAWN_EGG),
		Item::new(CAVE_SPIDER_SPAWN_EGG),
		Item::new(CHICKEN_SPAWN_EGG),
		Item::new(COD_SPAWN_EGG),
		Item::new(COW_SPAWN_EGG),
		Item::new(CREEPER_SPAWN_EGG),
		Item::new(DOLPHIN_SPAWN_EGG),
		Item::new(DONKEY_SPAWN_EGG),
		Item::new(DROWNED_SPAWN_EGG),
		Item::new(ELDER_GUARDIAN_SPAWN_EGG),
		Item::new(ENDERMAN_SPAWN_EGG),
		Item::new(ENDERMITE_SPAWN_EGG),
		Item::new(EVOKER_SPAWN_EGG),
		Item::new(FOX_SPAWN_EGG),
		Item::new(GHAST_SPAWN_EGG),
		Item::new(GUARDIAN_SPAWN_EGG),
		Item::new(HOGLIN_SPAWN_EGG),
		Item::new(HORSE_SPAWN_EGG),
		Item::new(HUSK_SPAWN_EGG),
		Item::new(LLAMA_SPAWN_EGG),
		Item::new(MAGMA_CUBE_SPAWN_EGG),
		Item::new(MOOSHROOM_SPAWN_EGG),
		Item::new(MULE_SPAWN_EGG),
		Item::new(OCELOT_SPAWN_EGG),
		Item::new(PANDA_SPAWN_EGG),
		Item::new(PARROT_SPAWN_EGG),
		Item::new(PHANTOM_SPAWN_EGG),
		Item::new(PIG_SPAWN_EGG),
		Item::new(PIGLIN_SPAWN_EGG),
		Item::new(PILLAGER_SPAWN_EGG),
		Item::new(POLAR_BEAR_SPAWN_EGG),
		Item::new(PUFFERFISH_SPAWN_EGG),
		Item::new(RABBIT_SPAWN_EGG),
		Item::new(RAVAGER_SPAWN_EGG),
		Item::new(SALMON_SPAWN_EGG),
		Item::new(SHEEP_SPAWN_EGG),
		Item::new(SHULKER_SPAWN_EGG),
		Item::new(SILVERFISH_SPAWN_EGG),
		Item::new(SKELETON_SPAWN_EGG),
		Item::new(SKELETON_HORSE_SPAWN_EGG),
		Item::new(SLIME_SPAWN_EGG),
		Item::new(SPIDER_SPAWN_EGG),
		Item::new(SQUID_SPAWN_EGG),
		Item::new(STRAY_SPAWN_EGG),
		Item::new(TRADER_LLAMA_SPAWN_EGG),
		Item::new(TROPICAL_FISH_SPAWN_EGG),
		Item::new(TURTLE_SPAWN_EGG),
		Item::new(VEX_SPAWN_EGG),
		Item::new(VILLAGER_SPAWN_EGG),
		Item::new(VINDICATOR_SPAWN_EGG),
		Item::new(WANDERING_TRADER_SPAWN_EGG),
		Item::new(WITCH_SPAWN_EGG),
		Item::new(WITHER_SKELETON_SPAWN_EGG),
		Item::new(WOLF_SPAWN_EGG),
		Item::new(ZOMBIE_SPAWN_EGG),
		Item::new(ZOMBIE_HORSE_SPAWN_EGG),
		Item::new(ZOMBIFIED_PIGLIN_SPAWN_EGG),
		Item::new(ZOMBIE_VILLAGER_SPAWN_EGG),
		Item::new(EXPERIENCE_BOTTLE),
		Item::new(FIRE_CHARGE),
		Item::new(WRITABLE_BOOK),
		Item::new(WRITTEN_BOOK),
		Item::new(EMERALD),
		Item::new(ITEM_FRAME),
		Item::new(FLOWER_POT),
		Item::new(CARROT),
		Item::new(POTATO),
		Item::new(BAKED_POTATO),
		Item::new(POISONOUS_POTATO),
		Item::new(MAP),
		Item::new(GOLDEN_CARROT),
		Item::new(SKELETON_SKULL),
		Item::new(WITHER_SKELETON_SKULL),
		Item::new(PLAYER_HEAD),
		Item::new(ZOMBIE_HEAD),
		Item::new(CREEPER_HEAD),
		Item::new(DRAGON_HEAD),
		Item::new(CARROT_ON_A_STICK),
		Item::new(NETHER_STAR),
		Item::new(PUMPKIN_PIE),
		Item::new(FIREWORK_ROCKET),
		Item::new(FIREWORK_STAR),
		Item::new(ENCHANTED_BOOK),
		Item::new(NETHER_BRICK),
		Item::new(QUARTZ),
		Item::new(TNT_MINECART),
		Item::new(HOPPER_MINECART),
		Item::new(PRISMARINE_SHARD),
		Item::new(PRISMARINE_CRYSTALS),
		Item::new(RABBIT),
		Item::new(COOKED_RABBIT),
		Item::new(RABBIT_STEW),
		Item::new(RABBIT_FOOT),
		Item::new(RABBIT_HIDE),
		Item::new(ARMOR_STAND),
		Item::new(IRON_HORSE_ARMOR),
		Item::new(GOLDEN_HORSE_ARMOR),
		Item::new(DIAMOND_HORSE_ARMOR),
		Item::new(LEATHER_HORSE_ARMOR),
		Item::new(LEAD),
		Item::new(NAME_TAG),
		Item::new(COMMAND_BLOCK_MINECART),
		Item::new(MUTTON),
		Item::new(COOKED_MUTTON),
		Item::new(WHITE_BANNER),
		Item::new(ORANGE_BANNER),
		Item::new(MAGENTA_BANNER),
		Item::new(LIGHT_BLUE_BANNER),
		Item::new(YELLOW_BANNER),
		Item::new(LIME_BANNER),
		Item::new(PINK_BANNER),
		Item::new(GRAY_BANNER),
		Item::new(LIGHT_GRAY_BANNER),
		Item::new(CYAN_BANNER),
		Item::new(PURPLE_BANNER),
		Item::new(BLUE_BANNER),
		Item::new(BROWN_BANNER),
		Item::new(GREEN_BANNER),
		Item::new(RED_BANNER),
		Item::new(BLACK_BANNER),
		Item::new(END_CRYSTAL),
		Item::new(CHORUS_FRUIT),
		Item::new(POPPED_CHORUS_FRUIT),
		Item::new(BEETROOT),
		Item::new(BEETROOT_SEEDS),
		Item::new(BEETROOT_SOUP),
		Item::new(DRAGON_BREATH),
		Item::new(SPLASH_POTION),
		Item::new(SPECTRAL_ARROW),
		Item::new(TIPPED_ARROW),
		Item::new(LINGERING_POTION),
		Item::new(SHIELD),
		Item::new(ELYTRA),
		Item::new(SPRUCE_BOAT),
		Item::new(BIRCH_BOAT),
		Item::new(JUNGLE_BOAT),
		Item::new(ACACIA_BOAT),
		Item::new(DARK_OAK_BOAT),
		Item::new(TOTEM_OF_UNDYING),
		Item::new(SHULKER_SHELL),
		Item::new(IRON_NUGGET),
		Item::new(KNOWLEDGE_BOOK),
		Item::new(DEBUG_STICK),
		Item::new(MUSIC_DISC_13),
		Item::new(MUSIC_DISC_CAT),
		Item::new(MUSIC_DISC_BLOCKS),
		Item::new(MUSIC_DISC_CHIRP),
		Item::new(MUSIC_DISC_FAR),
		Item::new(MUSIC_DISC_MALL),
		Item::new(MUSIC_DISC_MELLOHI),
		Item::new(MUSIC_DISC_STAL),
		Item::new(MUSIC_DISC_STRAD),
		Item::new(MUSIC_DISC_WARD),
		Item::new(MUSIC_DISC_11),
		Item::new(MUSIC_DISC_WAIT),
		Item::new(TRIDENT),
		Item::new(PHANTOM_MEMBRANE),
		Item::new(NAUTILUS_SHELL),
		Item::new(HEART_OF_THE_SEA),
		Item::new(CROSSBOW),
		Item::new(SUSPICIOUS_STEW),
		Item::new(LOOM),
		Item::new(FLOWER_BANNER_PATTERN),
		Item::new(CREEPER_BANNER_PATTERN),
		Item::new(SKULL_BANNER_PATTERN),
		Item::new(MOJANG_BANNER_PATTERN),
		Item::new(GLOBE_BANNER_PATTERN),
		Item::new(COMPOSTER),
		Item::new(BARREL),
		Item::new(SMOKER),
		Item::new(BLAST_FURNACE),
		Item::new(CARTOGRAPHY_TABLE),
		Item::new(FLETCHING_TABLE),
		Item::new(GRINDSTONE),
		Item::new(LECTERN),
		Item::new(SMITHING_TABLE),
		Item::new(STONECUTTER),
		Item::new(BELL),
		Item::new(LANTERN),
		Item::new(SOUL_FIRE_LANTERN),
		Item::new(SWEET_BERRIES),
		Item::new(CAMPFIRE),
		Item::new(SHROOMLIGHT),
		Item::new(HONEYCOMB),
		Item::new(BEE_NEST),
		Item::new(BEEHIVE),
		Item::new(HONEY_BOTTLE),
		Item::new(HONEY_BLOCK),
		Item::new(HONEYCOMB_BLOCK),
		Item::new(NETHERITE_BLOCK),
		Item::new(ANCIENT_DEBRIS),
		Item::new(NETHERITE_INGOT),
		Item::new(NETHERITE_SCRAP),
		Item::new(TARGET),
		Item::new(CRYING_OBSIDIAN),
		Item::new(RESPAWN_ANCHOR),
	]
}

pub fn get(item: i32) -> Option<Item> {
	match item {
		AIR => Some(Item::new(AIR)), 
		STONE => Some(Item::new(STONE)), 
		GRANITE => Some(Item::new(GRANITE)), 
		POLISHED_GRANITE => Some(Item::new(POLISHED_GRANITE)), 
		DIORITE => Some(Item::new(DIORITE)), 
		POLISHED_DIORITE => Some(Item::new(POLISHED_DIORITE)), 
		ANDESITE => Some(Item::new(ANDESITE)), 
		POLISHED_ANDESITE => Some(Item::new(POLISHED_ANDESITE)), 
		GRASS_BLOCK => Some(Item::new(GRASS_BLOCK)), 
		DIRT => Some(Item::new(DIRT)), 
		COARSE_DIRT => Some(Item::new(COARSE_DIRT)), 
		PODZOL => Some(Item::new(PODZOL)), 
		CRIMSON_NYLIUM => Some(Item::new(CRIMSON_NYLIUM)), 
		WARPED_NYLIUM => Some(Item::new(WARPED_NYLIUM)), 
		COBBLESTONE => Some(Item::new(COBBLESTONE)), 
		OAK_PLANKS => Some(Item::new(OAK_PLANKS)), 
		SPRUCE_PLANKS => Some(Item::new(SPRUCE_PLANKS)), 
		BIRCH_PLANKS => Some(Item::new(BIRCH_PLANKS)), 
		JUNGLE_PLANKS => Some(Item::new(JUNGLE_PLANKS)), 
		ACACIA_PLANKS => Some(Item::new(ACACIA_PLANKS)), 
		DARK_OAK_PLANKS => Some(Item::new(DARK_OAK_PLANKS)), 
		CRIMSON_PLANKS => Some(Item::new(CRIMSON_PLANKS)), 
		WARPED_PLANKS => Some(Item::new(WARPED_PLANKS)), 
		OAK_SAPLING => Some(Item::new(OAK_SAPLING)), 
		SPRUCE_SAPLING => Some(Item::new(SPRUCE_SAPLING)), 
		BIRCH_SAPLING => Some(Item::new(BIRCH_SAPLING)), 
		JUNGLE_SAPLING => Some(Item::new(JUNGLE_SAPLING)), 
		ACACIA_SAPLING => Some(Item::new(ACACIA_SAPLING)), 
		DARK_OAK_SAPLING => Some(Item::new(DARK_OAK_SAPLING)), 
		BEDROCK => Some(Item::new(BEDROCK)), 
		SAND => Some(Item::new(SAND)), 
		RED_SAND => Some(Item::new(RED_SAND)), 
		GRAVEL => Some(Item::new(GRAVEL)), 
		GOLD_ORE => Some(Item::new(GOLD_ORE)), 
		IRON_ORE => Some(Item::new(IRON_ORE)), 
		COAL_ORE => Some(Item::new(COAL_ORE)), 
		NETHER_GOLD_ORE => Some(Item::new(NETHER_GOLD_ORE)), 
		OAK_LOG => Some(Item::new(OAK_LOG)), 
		SPRUCE_LOG => Some(Item::new(SPRUCE_LOG)), 
		BIRCH_LOG => Some(Item::new(BIRCH_LOG)), 
		JUNGLE_LOG => Some(Item::new(JUNGLE_LOG)), 
		ACACIA_LOG => Some(Item::new(ACACIA_LOG)), 
		DARK_OAK_LOG => Some(Item::new(DARK_OAK_LOG)), 
		CRIMSON_STEM => Some(Item::new(CRIMSON_STEM)), 
		WARPED_STEM => Some(Item::new(WARPED_STEM)), 
		STRIPPED_OAK_LOG => Some(Item::new(STRIPPED_OAK_LOG)), 
		STRIPPED_SPRUCE_LOG => Some(Item::new(STRIPPED_SPRUCE_LOG)), 
		STRIPPED_BIRCH_LOG => Some(Item::new(STRIPPED_BIRCH_LOG)), 
		STRIPPED_JUNGLE_LOG => Some(Item::new(STRIPPED_JUNGLE_LOG)), 
		STRIPPED_ACACIA_LOG => Some(Item::new(STRIPPED_ACACIA_LOG)), 
		STRIPPED_DARK_OAK_LOG => Some(Item::new(STRIPPED_DARK_OAK_LOG)), 
		STRIPPED_CRIMSON_STEM => Some(Item::new(STRIPPED_CRIMSON_STEM)), 
		STRIPPED_WARPED_STEM => Some(Item::new(STRIPPED_WARPED_STEM)), 
		STRIPPED_OAK_WOOD => Some(Item::new(STRIPPED_OAK_WOOD)), 
		STRIPPED_SPRUCE_WOOD => Some(Item::new(STRIPPED_SPRUCE_WOOD)), 
		STRIPPED_BIRCH_WOOD => Some(Item::new(STRIPPED_BIRCH_WOOD)), 
		STRIPPED_JUNGLE_WOOD => Some(Item::new(STRIPPED_JUNGLE_WOOD)), 
		STRIPPED_ACACIA_WOOD => Some(Item::new(STRIPPED_ACACIA_WOOD)), 
		STRIPPED_DARK_OAK_WOOD => Some(Item::new(STRIPPED_DARK_OAK_WOOD)), 
		STRIPPED_CRIMSON_HYPHAE => Some(Item::new(STRIPPED_CRIMSON_HYPHAE)), 
		STRIPPED_WARPED_HYPHAE => Some(Item::new(STRIPPED_WARPED_HYPHAE)), 
		OAK_WOOD => Some(Item::new(OAK_WOOD)), 
		SPRUCE_WOOD => Some(Item::new(SPRUCE_WOOD)), 
		BIRCH_WOOD => Some(Item::new(BIRCH_WOOD)), 
		JUNGLE_WOOD => Some(Item::new(JUNGLE_WOOD)), 
		ACACIA_WOOD => Some(Item::new(ACACIA_WOOD)), 
		DARK_OAK_WOOD => Some(Item::new(DARK_OAK_WOOD)), 
		CRIMSON_HYPHAE => Some(Item::new(CRIMSON_HYPHAE)), 
		WARPED_HYPHAE => Some(Item::new(WARPED_HYPHAE)), 
		OAK_LEAVES => Some(Item::new(OAK_LEAVES)), 
		SPRUCE_LEAVES => Some(Item::new(SPRUCE_LEAVES)), 
		BIRCH_LEAVES => Some(Item::new(BIRCH_LEAVES)), 
		JUNGLE_LEAVES => Some(Item::new(JUNGLE_LEAVES)), 
		ACACIA_LEAVES => Some(Item::new(ACACIA_LEAVES)), 
		DARK_OAK_LEAVES => Some(Item::new(DARK_OAK_LEAVES)), 
		SPONGE => Some(Item::new(SPONGE)), 
		WET_SPONGE => Some(Item::new(WET_SPONGE)), 
		GLASS => Some(Item::new(GLASS)), 
		LAPIS_ORE => Some(Item::new(LAPIS_ORE)), 
		LAPIS_BLOCK => Some(Item::new(LAPIS_BLOCK)), 
		DISPENSER => Some(Item::new(DISPENSER)), 
		SANDSTONE => Some(Item::new(SANDSTONE)), 
		CHISELED_SANDSTONE => Some(Item::new(CHISELED_SANDSTONE)), 
		CUT_SANDSTONE => Some(Item::new(CUT_SANDSTONE)), 
		NOTE_BLOCK => Some(Item::new(NOTE_BLOCK)), 
		POWERED_RAIL => Some(Item::new(POWERED_RAIL)), 
		DETECTOR_RAIL => Some(Item::new(DETECTOR_RAIL)), 
		STICKY_PISTON => Some(Item::new(STICKY_PISTON)), 
		COBWEB => Some(Item::new(COBWEB)), 
		GRASS => Some(Item::new(GRASS)), 
		FERN => Some(Item::new(FERN)), 
		DEAD_BUSH => Some(Item::new(DEAD_BUSH)), 
		SEAGRASS => Some(Item::new(SEAGRASS)), 
		SEA_PICKLE => Some(Item::new(SEA_PICKLE)), 
		PISTON => Some(Item::new(PISTON)), 
		WHITE_WOOL => Some(Item::new(WHITE_WOOL)), 
		ORANGE_WOOL => Some(Item::new(ORANGE_WOOL)), 
		MAGENTA_WOOL => Some(Item::new(MAGENTA_WOOL)), 
		LIGHT_BLUE_WOOL => Some(Item::new(LIGHT_BLUE_WOOL)), 
		YELLOW_WOOL => Some(Item::new(YELLOW_WOOL)), 
		LIME_WOOL => Some(Item::new(LIME_WOOL)), 
		PINK_WOOL => Some(Item::new(PINK_WOOL)), 
		GRAY_WOOL => Some(Item::new(GRAY_WOOL)), 
		LIGHT_GRAY_WOOL => Some(Item::new(LIGHT_GRAY_WOOL)), 
		CYAN_WOOL => Some(Item::new(CYAN_WOOL)), 
		PURPLE_WOOL => Some(Item::new(PURPLE_WOOL)), 
		BLUE_WOOL => Some(Item::new(BLUE_WOOL)), 
		BROWN_WOOL => Some(Item::new(BROWN_WOOL)), 
		GREEN_WOOL => Some(Item::new(GREEN_WOOL)), 
		RED_WOOL => Some(Item::new(RED_WOOL)), 
		BLACK_WOOL => Some(Item::new(BLACK_WOOL)), 
		DANDELION => Some(Item::new(DANDELION)), 
		POPPY => Some(Item::new(POPPY)), 
		BLUE_ORCHID => Some(Item::new(BLUE_ORCHID)), 
		ALLIUM => Some(Item::new(ALLIUM)), 
		AZURE_BLUET => Some(Item::new(AZURE_BLUET)), 
		RED_TULIP => Some(Item::new(RED_TULIP)), 
		ORANGE_TULIP => Some(Item::new(ORANGE_TULIP)), 
		WHITE_TULIP => Some(Item::new(WHITE_TULIP)), 
		PINK_TULIP => Some(Item::new(PINK_TULIP)), 
		OXEYE_DAISY => Some(Item::new(OXEYE_DAISY)), 
		CORNFLOWER => Some(Item::new(CORNFLOWER)), 
		LILY_OF_THE_VALLEY => Some(Item::new(LILY_OF_THE_VALLEY)), 
		WITHER_ROSE => Some(Item::new(WITHER_ROSE)), 
		BROWN_MUSHROOM => Some(Item::new(BROWN_MUSHROOM)), 
		RED_MUSHROOM => Some(Item::new(RED_MUSHROOM)), 
		CRIMSON_FUNGUS => Some(Item::new(CRIMSON_FUNGUS)), 
		WARPED_FUNGUS => Some(Item::new(WARPED_FUNGUS)), 
		CRIMSON_ROOTS => Some(Item::new(CRIMSON_ROOTS)), 
		WARPED_ROOTS => Some(Item::new(WARPED_ROOTS)), 
		NETHER_SPROUTS => Some(Item::new(NETHER_SPROUTS)), 
		WEEPING_VINES => Some(Item::new(WEEPING_VINES)), 
		TWISTING_VINES => Some(Item::new(TWISTING_VINES)), 
		GOLD_BLOCK => Some(Item::new(GOLD_BLOCK)), 
		IRON_BLOCK => Some(Item::new(IRON_BLOCK)), 
		OAK_SLAB => Some(Item::new(OAK_SLAB)), 
		SPRUCE_SLAB => Some(Item::new(SPRUCE_SLAB)), 
		BIRCH_SLAB => Some(Item::new(BIRCH_SLAB)), 
		JUNGLE_SLAB => Some(Item::new(JUNGLE_SLAB)), 
		ACACIA_SLAB => Some(Item::new(ACACIA_SLAB)), 
		DARK_OAK_SLAB => Some(Item::new(DARK_OAK_SLAB)), 
		CRIMSON_SLAB => Some(Item::new(CRIMSON_SLAB)), 
		WARPED_SLAB => Some(Item::new(WARPED_SLAB)), 
		STONE_SLAB => Some(Item::new(STONE_SLAB)), 
		SMOOTH_STONE_SLAB => Some(Item::new(SMOOTH_STONE_SLAB)), 
		SANDSTONE_SLAB => Some(Item::new(SANDSTONE_SLAB)), 
		CUT_SANDSTONE_SLAB => Some(Item::new(CUT_SANDSTONE_SLAB)), 
		PETRIFIED_OAK_SLAB => Some(Item::new(PETRIFIED_OAK_SLAB)), 
		COBBLESTONE_SLAB => Some(Item::new(COBBLESTONE_SLAB)), 
		BRICK_SLAB => Some(Item::new(BRICK_SLAB)), 
		STONE_BRICK_SLAB => Some(Item::new(STONE_BRICK_SLAB)), 
		NETHER_BRICK_SLAB => Some(Item::new(NETHER_BRICK_SLAB)), 
		QUARTZ_SLAB => Some(Item::new(QUARTZ_SLAB)), 
		RED_SANDSTONE_SLAB => Some(Item::new(RED_SANDSTONE_SLAB)), 
		CUT_RED_SANDSTONE_SLAB => Some(Item::new(CUT_RED_SANDSTONE_SLAB)), 
		PURPUR_SLAB => Some(Item::new(PURPUR_SLAB)), 
		PRISMARINE_SLAB => Some(Item::new(PRISMARINE_SLAB)), 
		PRISMARINE_BRICK_SLAB => Some(Item::new(PRISMARINE_BRICK_SLAB)), 
		DARK_PRISMARINE_SLAB => Some(Item::new(DARK_PRISMARINE_SLAB)), 
		SMOOTH_QUARTZ => Some(Item::new(SMOOTH_QUARTZ)), 
		SMOOTH_RED_SANDSTONE => Some(Item::new(SMOOTH_RED_SANDSTONE)), 
		SMOOTH_SANDSTONE => Some(Item::new(SMOOTH_SANDSTONE)), 
		SMOOTH_STONE => Some(Item::new(SMOOTH_STONE)), 
		BRICKS => Some(Item::new(BRICKS)), 
		TNT => Some(Item::new(TNT)), 
		BOOKSHELF => Some(Item::new(BOOKSHELF)), 
		MOSSY_COBBLESTONE => Some(Item::new(MOSSY_COBBLESTONE)), 
		OBSIDIAN => Some(Item::new(OBSIDIAN)), 
		TORCH => Some(Item::new(TORCH)), 
		END_ROD => Some(Item::new(END_ROD)), 
		CHORUS_PLANT => Some(Item::new(CHORUS_PLANT)), 
		CHORUS_FLOWER => Some(Item::new(CHORUS_FLOWER)), 
		PURPUR_BLOCK => Some(Item::new(PURPUR_BLOCK)), 
		PURPUR_PILLAR => Some(Item::new(PURPUR_PILLAR)), 
		PURPUR_STAIRS => Some(Item::new(PURPUR_STAIRS)), 
		SPAWNER => Some(Item::new(SPAWNER)), 
		OAK_STAIRS => Some(Item::new(OAK_STAIRS)), 
		CHEST => Some(Item::new(CHEST)), 
		DIAMOND_ORE => Some(Item::new(DIAMOND_ORE)), 
		DIAMOND_BLOCK => Some(Item::new(DIAMOND_BLOCK)), 
		CRAFTING_TABLE => Some(Item::new(CRAFTING_TABLE)), 
		FARMLAND => Some(Item::new(FARMLAND)), 
		FURNACE => Some(Item::new(FURNACE)), 
		LADDER => Some(Item::new(LADDER)), 
		RAIL => Some(Item::new(RAIL)), 
		COBBLESTONE_STAIRS => Some(Item::new(COBBLESTONE_STAIRS)), 
		LEVER => Some(Item::new(LEVER)), 
		STONE_PRESSURE_PLATE => Some(Item::new(STONE_PRESSURE_PLATE)), 
		OAK_PRESSURE_PLATE => Some(Item::new(OAK_PRESSURE_PLATE)), 
		SPRUCE_PRESSURE_PLATE => Some(Item::new(SPRUCE_PRESSURE_PLATE)), 
		BIRCH_PRESSURE_PLATE => Some(Item::new(BIRCH_PRESSURE_PLATE)), 
		JUNGLE_PRESSURE_PLATE => Some(Item::new(JUNGLE_PRESSURE_PLATE)), 
		ACACIA_PRESSURE_PLATE => Some(Item::new(ACACIA_PRESSURE_PLATE)), 
		DARK_OAK_PRESSURE_PLATE => Some(Item::new(DARK_OAK_PRESSURE_PLATE)), 
		CRIMSON_PRESSURE_PLATE => Some(Item::new(CRIMSON_PRESSURE_PLATE)), 
		WARPED_PRESSURE_PLATE => Some(Item::new(WARPED_PRESSURE_PLATE)), 
		REDSTONE_ORE => Some(Item::new(REDSTONE_ORE)), 
		REDSTONE_TORCH => Some(Item::new(REDSTONE_TORCH)), 
		STONE_BUTTON => Some(Item::new(STONE_BUTTON)), 
		SNOW => Some(Item::new(SNOW)), 
		ICE => Some(Item::new(ICE)), 
		SNOW_BLOCK => Some(Item::new(SNOW_BLOCK)), 
		CACTUS => Some(Item::new(CACTUS)), 
		CLAY => Some(Item::new(CLAY)), 
		JUKEBOX => Some(Item::new(JUKEBOX)), 
		OAK_FENCE => Some(Item::new(OAK_FENCE)), 
		SPRUCE_FENCE => Some(Item::new(SPRUCE_FENCE)), 
		BIRCH_FENCE => Some(Item::new(BIRCH_FENCE)), 
		JUNGLE_FENCE => Some(Item::new(JUNGLE_FENCE)), 
		ACACIA_FENCE => Some(Item::new(ACACIA_FENCE)), 
		DARK_OAK_FENCE => Some(Item::new(DARK_OAK_FENCE)), 
		CRIMSON_FENCE => Some(Item::new(CRIMSON_FENCE)), 
		WARPED_FENCE => Some(Item::new(WARPED_FENCE)), 
		PUMPKIN => Some(Item::new(PUMPKIN)), 
		CARVED_PUMPKIN => Some(Item::new(CARVED_PUMPKIN)), 
		NETHERRACK => Some(Item::new(NETHERRACK)), 
		SOUL_SAND => Some(Item::new(SOUL_SAND)), 
		SOUL_SOIL => Some(Item::new(SOUL_SOIL)), 
		BASALT => Some(Item::new(BASALT)), 
		POLISHED_BASALT => Some(Item::new(POLISHED_BASALT)), 
		SOUL_FIRE_TORCH => Some(Item::new(SOUL_FIRE_TORCH)), 
		GLOWSTONE => Some(Item::new(GLOWSTONE)), 
		JACK_O_LANTERN => Some(Item::new(JACK_O_LANTERN)), 
		OAK_TRAPDOOR => Some(Item::new(OAK_TRAPDOOR)), 
		SPRUCE_TRAPDOOR => Some(Item::new(SPRUCE_TRAPDOOR)), 
		BIRCH_TRAPDOOR => Some(Item::new(BIRCH_TRAPDOOR)), 
		JUNGLE_TRAPDOOR => Some(Item::new(JUNGLE_TRAPDOOR)), 
		ACACIA_TRAPDOOR => Some(Item::new(ACACIA_TRAPDOOR)), 
		DARK_OAK_TRAPDOOR => Some(Item::new(DARK_OAK_TRAPDOOR)), 
		CRIMSON_TRAPDOOR => Some(Item::new(CRIMSON_TRAPDOOR)), 
		WARPED_TRAPDOOR => Some(Item::new(WARPED_TRAPDOOR)), 
		INFESTED_STONE => Some(Item::new(INFESTED_STONE)), 
		INFESTED_COBBLESTONE => Some(Item::new(INFESTED_COBBLESTONE)), 
		INFESTED_STONE_BRICKS => Some(Item::new(INFESTED_STONE_BRICKS)), 
		INFESTED_MOSSY_STONE_BRICKS => Some(Item::new(INFESTED_MOSSY_STONE_BRICKS)), 
		INFESTED_CRACKED_STONE_BRICKS => Some(Item::new(INFESTED_CRACKED_STONE_BRICKS)), 
		INFESTED_CHISELED_STONE_BRICKS => Some(Item::new(INFESTED_CHISELED_STONE_BRICKS)), 
		STONE_BRICKS => Some(Item::new(STONE_BRICKS)), 
		MOSSY_STONE_BRICKS => Some(Item::new(MOSSY_STONE_BRICKS)), 
		CRACKED_STONE_BRICKS => Some(Item::new(CRACKED_STONE_BRICKS)), 
		CHISELED_STONE_BRICKS => Some(Item::new(CHISELED_STONE_BRICKS)), 
		BROWN_MUSHROOM_BLOCK => Some(Item::new(BROWN_MUSHROOM_BLOCK)), 
		RED_MUSHROOM_BLOCK => Some(Item::new(RED_MUSHROOM_BLOCK)), 
		MUSHROOM_STEM => Some(Item::new(MUSHROOM_STEM)), 
		IRON_BARS => Some(Item::new(IRON_BARS)), 
		GLASS_PANE => Some(Item::new(GLASS_PANE)), 
		MELON => Some(Item::new(MELON)), 
		VINE => Some(Item::new(VINE)), 
		OAK_FENCE_GATE => Some(Item::new(OAK_FENCE_GATE)), 
		SPRUCE_FENCE_GATE => Some(Item::new(SPRUCE_FENCE_GATE)), 
		BIRCH_FENCE_GATE => Some(Item::new(BIRCH_FENCE_GATE)), 
		JUNGLE_FENCE_GATE => Some(Item::new(JUNGLE_FENCE_GATE)), 
		ACACIA_FENCE_GATE => Some(Item::new(ACACIA_FENCE_GATE)), 
		DARK_OAK_FENCE_GATE => Some(Item::new(DARK_OAK_FENCE_GATE)), 
		CRIMSON_FENCE_GATE => Some(Item::new(CRIMSON_FENCE_GATE)), 
		WARPED_FENCE_GATE => Some(Item::new(WARPED_FENCE_GATE)), 
		BRICK_STAIRS => Some(Item::new(BRICK_STAIRS)), 
		STONE_BRICK_STAIRS => Some(Item::new(STONE_BRICK_STAIRS)), 
		MYCELIUM => Some(Item::new(MYCELIUM)), 
		LILY_PAD => Some(Item::new(LILY_PAD)), 
		NETHER_BRICKS => Some(Item::new(NETHER_BRICKS)), 
		NETHER_BRICK_FENCE => Some(Item::new(NETHER_BRICK_FENCE)), 
		NETHER_BRICK_STAIRS => Some(Item::new(NETHER_BRICK_STAIRS)), 
		ENCHANTING_TABLE => Some(Item::new(ENCHANTING_TABLE)), 
		END_PORTAL_FRAME => Some(Item::new(END_PORTAL_FRAME)), 
		END_STONE => Some(Item::new(END_STONE)), 
		END_STONE_BRICKS => Some(Item::new(END_STONE_BRICKS)), 
		DRAGON_EGG => Some(Item::new(DRAGON_EGG)), 
		REDSTONE_LAMP => Some(Item::new(REDSTONE_LAMP)), 
		SANDSTONE_STAIRS => Some(Item::new(SANDSTONE_STAIRS)), 
		EMERALD_ORE => Some(Item::new(EMERALD_ORE)), 
		ENDER_CHEST => Some(Item::new(ENDER_CHEST)), 
		TRIPWIRE_HOOK => Some(Item::new(TRIPWIRE_HOOK)), 
		EMERALD_BLOCK => Some(Item::new(EMERALD_BLOCK)), 
		SPRUCE_STAIRS => Some(Item::new(SPRUCE_STAIRS)), 
		BIRCH_STAIRS => Some(Item::new(BIRCH_STAIRS)), 
		JUNGLE_STAIRS => Some(Item::new(JUNGLE_STAIRS)), 
		CRIMSON_STAIRS => Some(Item::new(CRIMSON_STAIRS)), 
		WARPED_STAIRS => Some(Item::new(WARPED_STAIRS)), 
		COMMAND_BLOCK => Some(Item::new(COMMAND_BLOCK)), 
		BEACON => Some(Item::new(BEACON)), 
		COBBLESTONE_WALL => Some(Item::new(COBBLESTONE_WALL)), 
		MOSSY_COBBLESTONE_WALL => Some(Item::new(MOSSY_COBBLESTONE_WALL)), 
		BRICK_WALL => Some(Item::new(BRICK_WALL)), 
		PRISMARINE_WALL => Some(Item::new(PRISMARINE_WALL)), 
		RED_SANDSTONE_WALL => Some(Item::new(RED_SANDSTONE_WALL)), 
		MOSSY_STONE_BRICK_WALL => Some(Item::new(MOSSY_STONE_BRICK_WALL)), 
		GRANITE_WALL => Some(Item::new(GRANITE_WALL)), 
		STONE_BRICK_WALL => Some(Item::new(STONE_BRICK_WALL)), 
		NETHER_BRICK_WALL => Some(Item::new(NETHER_BRICK_WALL)), 
		ANDESITE_WALL => Some(Item::new(ANDESITE_WALL)), 
		RED_NETHER_BRICK_WALL => Some(Item::new(RED_NETHER_BRICK_WALL)), 
		SANDSTONE_WALL => Some(Item::new(SANDSTONE_WALL)), 
		END_STONE_BRICK_WALL => Some(Item::new(END_STONE_BRICK_WALL)), 
		DIORITE_WALL => Some(Item::new(DIORITE_WALL)), 
		OAK_BUTTON => Some(Item::new(OAK_BUTTON)), 
		SPRUCE_BUTTON => Some(Item::new(SPRUCE_BUTTON)), 
		BIRCH_BUTTON => Some(Item::new(BIRCH_BUTTON)), 
		JUNGLE_BUTTON => Some(Item::new(JUNGLE_BUTTON)), 
		ACACIA_BUTTON => Some(Item::new(ACACIA_BUTTON)), 
		DARK_OAK_BUTTON => Some(Item::new(DARK_OAK_BUTTON)), 
		CRIMSON_BUTTON => Some(Item::new(CRIMSON_BUTTON)), 
		WARPED_BUTTON => Some(Item::new(WARPED_BUTTON)), 
		ANVIL => Some(Item::new(ANVIL)), 
		CHIPPED_ANVIL => Some(Item::new(CHIPPED_ANVIL)), 
		DAMAGED_ANVIL => Some(Item::new(DAMAGED_ANVIL)), 
		TRAPPED_CHEST => Some(Item::new(TRAPPED_CHEST)), 
		LIGHT_WEIGHTED_PRESSURE_PLATE => Some(Item::new(LIGHT_WEIGHTED_PRESSURE_PLATE)), 
		HEAVY_WEIGHTED_PRESSURE_PLATE => Some(Item::new(HEAVY_WEIGHTED_PRESSURE_PLATE)), 
		DAYLIGHT_DETECTOR => Some(Item::new(DAYLIGHT_DETECTOR)), 
		REDSTONE_BLOCK => Some(Item::new(REDSTONE_BLOCK)), 
		NETHER_QUARTZ_ORE => Some(Item::new(NETHER_QUARTZ_ORE)), 
		HOPPER => Some(Item::new(HOPPER)), 
		CHISELED_QUARTZ_BLOCK => Some(Item::new(CHISELED_QUARTZ_BLOCK)), 
		QUARTZ_BLOCK => Some(Item::new(QUARTZ_BLOCK)), 
		QUARTZ_PILLAR => Some(Item::new(QUARTZ_PILLAR)), 
		QUARTZ_STAIRS => Some(Item::new(QUARTZ_STAIRS)), 
		ACTIVATOR_RAIL => Some(Item::new(ACTIVATOR_RAIL)), 
		DROPPER => Some(Item::new(DROPPER)), 
		WHITE_TERRACOTTA => Some(Item::new(WHITE_TERRACOTTA)), 
		ORANGE_TERRACOTTA => Some(Item::new(ORANGE_TERRACOTTA)), 
		MAGENTA_TERRACOTTA => Some(Item::new(MAGENTA_TERRACOTTA)), 
		LIGHT_BLUE_TERRACOTTA => Some(Item::new(LIGHT_BLUE_TERRACOTTA)), 
		YELLOW_TERRACOTTA => Some(Item::new(YELLOW_TERRACOTTA)), 
		LIME_TERRACOTTA => Some(Item::new(LIME_TERRACOTTA)), 
		PINK_TERRACOTTA => Some(Item::new(PINK_TERRACOTTA)), 
		GRAY_TERRACOTTA => Some(Item::new(GRAY_TERRACOTTA)), 
		LIGHT_GRAY_TERRACOTTA => Some(Item::new(LIGHT_GRAY_TERRACOTTA)), 
		CYAN_TERRACOTTA => Some(Item::new(CYAN_TERRACOTTA)), 
		PURPLE_TERRACOTTA => Some(Item::new(PURPLE_TERRACOTTA)), 
		BLUE_TERRACOTTA => Some(Item::new(BLUE_TERRACOTTA)), 
		BROWN_TERRACOTTA => Some(Item::new(BROWN_TERRACOTTA)), 
		GREEN_TERRACOTTA => Some(Item::new(GREEN_TERRACOTTA)), 
		RED_TERRACOTTA => Some(Item::new(RED_TERRACOTTA)), 
		BLACK_TERRACOTTA => Some(Item::new(BLACK_TERRACOTTA)), 
		BARRIER => Some(Item::new(BARRIER)), 
		IRON_TRAPDOOR => Some(Item::new(IRON_TRAPDOOR)), 
		HAY_BLOCK => Some(Item::new(HAY_BLOCK)), 
		WHITE_CARPET => Some(Item::new(WHITE_CARPET)), 
		ORANGE_CARPET => Some(Item::new(ORANGE_CARPET)), 
		MAGENTA_CARPET => Some(Item::new(MAGENTA_CARPET)), 
		LIGHT_BLUE_CARPET => Some(Item::new(LIGHT_BLUE_CARPET)), 
		YELLOW_CARPET => Some(Item::new(YELLOW_CARPET)), 
		LIME_CARPET => Some(Item::new(LIME_CARPET)), 
		PINK_CARPET => Some(Item::new(PINK_CARPET)), 
		GRAY_CARPET => Some(Item::new(GRAY_CARPET)), 
		LIGHT_GRAY_CARPET => Some(Item::new(LIGHT_GRAY_CARPET)), 
		CYAN_CARPET => Some(Item::new(CYAN_CARPET)), 
		PURPLE_CARPET => Some(Item::new(PURPLE_CARPET)), 
		BLUE_CARPET => Some(Item::new(BLUE_CARPET)), 
		BROWN_CARPET => Some(Item::new(BROWN_CARPET)), 
		GREEN_CARPET => Some(Item::new(GREEN_CARPET)), 
		RED_CARPET => Some(Item::new(RED_CARPET)), 
		BLACK_CARPET => Some(Item::new(BLACK_CARPET)), 
		TERRACOTTA => Some(Item::new(TERRACOTTA)), 
		COAL_BLOCK => Some(Item::new(COAL_BLOCK)), 
		PACKED_ICE => Some(Item::new(PACKED_ICE)), 
		ACACIA_STAIRS => Some(Item::new(ACACIA_STAIRS)), 
		DARK_OAK_STAIRS => Some(Item::new(DARK_OAK_STAIRS)), 
		SLIME_BLOCK => Some(Item::new(SLIME_BLOCK)), 
		GRASS_PATH => Some(Item::new(GRASS_PATH)), 
		SUNFLOWER => Some(Item::new(SUNFLOWER)), 
		LILAC => Some(Item::new(LILAC)), 
		ROSE_BUSH => Some(Item::new(ROSE_BUSH)), 
		PEONY => Some(Item::new(PEONY)), 
		TALL_GRASS => Some(Item::new(TALL_GRASS)), 
		LARGE_FERN => Some(Item::new(LARGE_FERN)), 
		WHITE_STAINED_GLASS => Some(Item::new(WHITE_STAINED_GLASS)), 
		ORANGE_STAINED_GLASS => Some(Item::new(ORANGE_STAINED_GLASS)), 
		MAGENTA_STAINED_GLASS => Some(Item::new(MAGENTA_STAINED_GLASS)), 
		LIGHT_BLUE_STAINED_GLASS => Some(Item::new(LIGHT_BLUE_STAINED_GLASS)), 
		YELLOW_STAINED_GLASS => Some(Item::new(YELLOW_STAINED_GLASS)), 
		LIME_STAINED_GLASS => Some(Item::new(LIME_STAINED_GLASS)), 
		PINK_STAINED_GLASS => Some(Item::new(PINK_STAINED_GLASS)), 
		GRAY_STAINED_GLASS => Some(Item::new(GRAY_STAINED_GLASS)), 
		LIGHT_GRAY_STAINED_GLASS => Some(Item::new(LIGHT_GRAY_STAINED_GLASS)), 
		CYAN_STAINED_GLASS => Some(Item::new(CYAN_STAINED_GLASS)), 
		PURPLE_STAINED_GLASS => Some(Item::new(PURPLE_STAINED_GLASS)), 
		BLUE_STAINED_GLASS => Some(Item::new(BLUE_STAINED_GLASS)), 
		BROWN_STAINED_GLASS => Some(Item::new(BROWN_STAINED_GLASS)), 
		GREEN_STAINED_GLASS => Some(Item::new(GREEN_STAINED_GLASS)), 
		RED_STAINED_GLASS => Some(Item::new(RED_STAINED_GLASS)), 
		BLACK_STAINED_GLASS => Some(Item::new(BLACK_STAINED_GLASS)), 
		WHITE_STAINED_GLASS_PANE => Some(Item::new(WHITE_STAINED_GLASS_PANE)), 
		ORANGE_STAINED_GLASS_PANE => Some(Item::new(ORANGE_STAINED_GLASS_PANE)), 
		MAGENTA_STAINED_GLASS_PANE => Some(Item::new(MAGENTA_STAINED_GLASS_PANE)), 
		LIGHT_BLUE_STAINED_GLASS_PANE => Some(Item::new(LIGHT_BLUE_STAINED_GLASS_PANE)), 
		YELLOW_STAINED_GLASS_PANE => Some(Item::new(YELLOW_STAINED_GLASS_PANE)), 
		LIME_STAINED_GLASS_PANE => Some(Item::new(LIME_STAINED_GLASS_PANE)), 
		PINK_STAINED_GLASS_PANE => Some(Item::new(PINK_STAINED_GLASS_PANE)), 
		GRAY_STAINED_GLASS_PANE => Some(Item::new(GRAY_STAINED_GLASS_PANE)), 
		LIGHT_GRAY_STAINED_GLASS_PANE => Some(Item::new(LIGHT_GRAY_STAINED_GLASS_PANE)), 
		CYAN_STAINED_GLASS_PANE => Some(Item::new(CYAN_STAINED_GLASS_PANE)), 
		PURPLE_STAINED_GLASS_PANE => Some(Item::new(PURPLE_STAINED_GLASS_PANE)), 
		BLUE_STAINED_GLASS_PANE => Some(Item::new(BLUE_STAINED_GLASS_PANE)), 
		BROWN_STAINED_GLASS_PANE => Some(Item::new(BROWN_STAINED_GLASS_PANE)), 
		GREEN_STAINED_GLASS_PANE => Some(Item::new(GREEN_STAINED_GLASS_PANE)), 
		RED_STAINED_GLASS_PANE => Some(Item::new(RED_STAINED_GLASS_PANE)), 
		BLACK_STAINED_GLASS_PANE => Some(Item::new(BLACK_STAINED_GLASS_PANE)), 
		PRISMARINE => Some(Item::new(PRISMARINE)), 
		PRISMARINE_BRICKS => Some(Item::new(PRISMARINE_BRICKS)), 
		DARK_PRISMARINE => Some(Item::new(DARK_PRISMARINE)), 
		PRISMARINE_STAIRS => Some(Item::new(PRISMARINE_STAIRS)), 
		PRISMARINE_BRICK_STAIRS => Some(Item::new(PRISMARINE_BRICK_STAIRS)), 
		DARK_PRISMARINE_STAIRS => Some(Item::new(DARK_PRISMARINE_STAIRS)), 
		SEA_LANTERN => Some(Item::new(SEA_LANTERN)), 
		RED_SANDSTONE => Some(Item::new(RED_SANDSTONE)), 
		CHISELED_RED_SANDSTONE => Some(Item::new(CHISELED_RED_SANDSTONE)), 
		CUT_RED_SANDSTONE => Some(Item::new(CUT_RED_SANDSTONE)), 
		RED_SANDSTONE_STAIRS => Some(Item::new(RED_SANDSTONE_STAIRS)), 
		REPEATING_COMMAND_BLOCK => Some(Item::new(REPEATING_COMMAND_BLOCK)), 
		CHAIN_COMMAND_BLOCK => Some(Item::new(CHAIN_COMMAND_BLOCK)), 
		MAGMA_BLOCK => Some(Item::new(MAGMA_BLOCK)), 
		NETHER_WART_BLOCK => Some(Item::new(NETHER_WART_BLOCK)), 
		WARPED_WART_BLOCK => Some(Item::new(WARPED_WART_BLOCK)), 
		RED_NETHER_BRICKS => Some(Item::new(RED_NETHER_BRICKS)), 
		BONE_BLOCK => Some(Item::new(BONE_BLOCK)), 
		STRUCTURE_VOID => Some(Item::new(STRUCTURE_VOID)), 
		OBSERVER => Some(Item::new(OBSERVER)), 
		SHULKER_BOX => Some(Item::new(SHULKER_BOX)), 
		WHITE_SHULKER_BOX => Some(Item::new(WHITE_SHULKER_BOX)), 
		ORANGE_SHULKER_BOX => Some(Item::new(ORANGE_SHULKER_BOX)), 
		MAGENTA_SHULKER_BOX => Some(Item::new(MAGENTA_SHULKER_BOX)), 
		LIGHT_BLUE_SHULKER_BOX => Some(Item::new(LIGHT_BLUE_SHULKER_BOX)), 
		YELLOW_SHULKER_BOX => Some(Item::new(YELLOW_SHULKER_BOX)), 
		LIME_SHULKER_BOX => Some(Item::new(LIME_SHULKER_BOX)), 
		PINK_SHULKER_BOX => Some(Item::new(PINK_SHULKER_BOX)), 
		GRAY_SHULKER_BOX => Some(Item::new(GRAY_SHULKER_BOX)), 
		LIGHT_GRAY_SHULKER_BOX => Some(Item::new(LIGHT_GRAY_SHULKER_BOX)), 
		CYAN_SHULKER_BOX => Some(Item::new(CYAN_SHULKER_BOX)), 
		PURPLE_SHULKER_BOX => Some(Item::new(PURPLE_SHULKER_BOX)), 
		BLUE_SHULKER_BOX => Some(Item::new(BLUE_SHULKER_BOX)), 
		BROWN_SHULKER_BOX => Some(Item::new(BROWN_SHULKER_BOX)), 
		GREEN_SHULKER_BOX => Some(Item::new(GREEN_SHULKER_BOX)), 
		RED_SHULKER_BOX => Some(Item::new(RED_SHULKER_BOX)), 
		BLACK_SHULKER_BOX => Some(Item::new(BLACK_SHULKER_BOX)), 
		WHITE_GLAZED_TERRACOTTA => Some(Item::new(WHITE_GLAZED_TERRACOTTA)), 
		ORANGE_GLAZED_TERRACOTTA => Some(Item::new(ORANGE_GLAZED_TERRACOTTA)), 
		MAGENTA_GLAZED_TERRACOTTA => Some(Item::new(MAGENTA_GLAZED_TERRACOTTA)), 
		LIGHT_BLUE_GLAZED_TERRACOTTA => Some(Item::new(LIGHT_BLUE_GLAZED_TERRACOTTA)), 
		YELLOW_GLAZED_TERRACOTTA => Some(Item::new(YELLOW_GLAZED_TERRACOTTA)), 
		LIME_GLAZED_TERRACOTTA => Some(Item::new(LIME_GLAZED_TERRACOTTA)), 
		PINK_GLAZED_TERRACOTTA => Some(Item::new(PINK_GLAZED_TERRACOTTA)), 
		GRAY_GLAZED_TERRACOTTA => Some(Item::new(GRAY_GLAZED_TERRACOTTA)), 
		LIGHT_GRAY_GLAZED_TERRACOTTA => Some(Item::new(LIGHT_GRAY_GLAZED_TERRACOTTA)), 
		CYAN_GLAZED_TERRACOTTA => Some(Item::new(CYAN_GLAZED_TERRACOTTA)), 
		PURPLE_GLAZED_TERRACOTTA => Some(Item::new(PURPLE_GLAZED_TERRACOTTA)), 
		BLUE_GLAZED_TERRACOTTA => Some(Item::new(BLUE_GLAZED_TERRACOTTA)), 
		BROWN_GLAZED_TERRACOTTA => Some(Item::new(BROWN_GLAZED_TERRACOTTA)), 
		GREEN_GLAZED_TERRACOTTA => Some(Item::new(GREEN_GLAZED_TERRACOTTA)), 
		RED_GLAZED_TERRACOTTA => Some(Item::new(RED_GLAZED_TERRACOTTA)), 
		BLACK_GLAZED_TERRACOTTA => Some(Item::new(BLACK_GLAZED_TERRACOTTA)), 
		WHITE_CONCRETE => Some(Item::new(WHITE_CONCRETE)), 
		ORANGE_CONCRETE => Some(Item::new(ORANGE_CONCRETE)), 
		MAGENTA_CONCRETE => Some(Item::new(MAGENTA_CONCRETE)), 
		LIGHT_BLUE_CONCRETE => Some(Item::new(LIGHT_BLUE_CONCRETE)), 
		YELLOW_CONCRETE => Some(Item::new(YELLOW_CONCRETE)), 
		LIME_CONCRETE => Some(Item::new(LIME_CONCRETE)), 
		PINK_CONCRETE => Some(Item::new(PINK_CONCRETE)), 
		GRAY_CONCRETE => Some(Item::new(GRAY_CONCRETE)), 
		LIGHT_GRAY_CONCRETE => Some(Item::new(LIGHT_GRAY_CONCRETE)), 
		CYAN_CONCRETE => Some(Item::new(CYAN_CONCRETE)), 
		PURPLE_CONCRETE => Some(Item::new(PURPLE_CONCRETE)), 
		BLUE_CONCRETE => Some(Item::new(BLUE_CONCRETE)), 
		BROWN_CONCRETE => Some(Item::new(BROWN_CONCRETE)), 
		GREEN_CONCRETE => Some(Item::new(GREEN_CONCRETE)), 
		RED_CONCRETE => Some(Item::new(RED_CONCRETE)), 
		BLACK_CONCRETE => Some(Item::new(BLACK_CONCRETE)), 
		WHITE_CONCRETE_POWDER => Some(Item::new(WHITE_CONCRETE_POWDER)), 
		ORANGE_CONCRETE_POWDER => Some(Item::new(ORANGE_CONCRETE_POWDER)), 
		MAGENTA_CONCRETE_POWDER => Some(Item::new(MAGENTA_CONCRETE_POWDER)), 
		LIGHT_BLUE_CONCRETE_POWDER => Some(Item::new(LIGHT_BLUE_CONCRETE_POWDER)), 
		YELLOW_CONCRETE_POWDER => Some(Item::new(YELLOW_CONCRETE_POWDER)), 
		LIME_CONCRETE_POWDER => Some(Item::new(LIME_CONCRETE_POWDER)), 
		PINK_CONCRETE_POWDER => Some(Item::new(PINK_CONCRETE_POWDER)), 
		GRAY_CONCRETE_POWDER => Some(Item::new(GRAY_CONCRETE_POWDER)), 
		LIGHT_GRAY_CONCRETE_POWDER => Some(Item::new(LIGHT_GRAY_CONCRETE_POWDER)), 
		CYAN_CONCRETE_POWDER => Some(Item::new(CYAN_CONCRETE_POWDER)), 
		PURPLE_CONCRETE_POWDER => Some(Item::new(PURPLE_CONCRETE_POWDER)), 
		BLUE_CONCRETE_POWDER => Some(Item::new(BLUE_CONCRETE_POWDER)), 
		BROWN_CONCRETE_POWDER => Some(Item::new(BROWN_CONCRETE_POWDER)), 
		GREEN_CONCRETE_POWDER => Some(Item::new(GREEN_CONCRETE_POWDER)), 
		RED_CONCRETE_POWDER => Some(Item::new(RED_CONCRETE_POWDER)), 
		BLACK_CONCRETE_POWDER => Some(Item::new(BLACK_CONCRETE_POWDER)), 
		TURTLE_EGG => Some(Item::new(TURTLE_EGG)), 
		DEAD_TUBE_CORAL_BLOCK => Some(Item::new(DEAD_TUBE_CORAL_BLOCK)), 
		DEAD_BRAIN_CORAL_BLOCK => Some(Item::new(DEAD_BRAIN_CORAL_BLOCK)), 
		DEAD_BUBBLE_CORAL_BLOCK => Some(Item::new(DEAD_BUBBLE_CORAL_BLOCK)), 
		DEAD_FIRE_CORAL_BLOCK => Some(Item::new(DEAD_FIRE_CORAL_BLOCK)), 
		DEAD_HORN_CORAL_BLOCK => Some(Item::new(DEAD_HORN_CORAL_BLOCK)), 
		TUBE_CORAL_BLOCK => Some(Item::new(TUBE_CORAL_BLOCK)), 
		BRAIN_CORAL_BLOCK => Some(Item::new(BRAIN_CORAL_BLOCK)), 
		BUBBLE_CORAL_BLOCK => Some(Item::new(BUBBLE_CORAL_BLOCK)), 
		FIRE_CORAL_BLOCK => Some(Item::new(FIRE_CORAL_BLOCK)), 
		HORN_CORAL_BLOCK => Some(Item::new(HORN_CORAL_BLOCK)), 
		TUBE_CORAL => Some(Item::new(TUBE_CORAL)), 
		BRAIN_CORAL => Some(Item::new(BRAIN_CORAL)), 
		BUBBLE_CORAL => Some(Item::new(BUBBLE_CORAL)), 
		FIRE_CORAL => Some(Item::new(FIRE_CORAL)), 
		HORN_CORAL => Some(Item::new(HORN_CORAL)), 
		DEAD_BRAIN_CORAL => Some(Item::new(DEAD_BRAIN_CORAL)), 
		DEAD_BUBBLE_CORAL => Some(Item::new(DEAD_BUBBLE_CORAL)), 
		DEAD_FIRE_CORAL => Some(Item::new(DEAD_FIRE_CORAL)), 
		DEAD_HORN_CORAL => Some(Item::new(DEAD_HORN_CORAL)), 
		DEAD_TUBE_CORAL => Some(Item::new(DEAD_TUBE_CORAL)), 
		TUBE_CORAL_FAN => Some(Item::new(TUBE_CORAL_FAN)), 
		BRAIN_CORAL_FAN => Some(Item::new(BRAIN_CORAL_FAN)), 
		BUBBLE_CORAL_FAN => Some(Item::new(BUBBLE_CORAL_FAN)), 
		FIRE_CORAL_FAN => Some(Item::new(FIRE_CORAL_FAN)), 
		HORN_CORAL_FAN => Some(Item::new(HORN_CORAL_FAN)), 
		DEAD_TUBE_CORAL_FAN => Some(Item::new(DEAD_TUBE_CORAL_FAN)), 
		DEAD_BRAIN_CORAL_FAN => Some(Item::new(DEAD_BRAIN_CORAL_FAN)), 
		DEAD_BUBBLE_CORAL_FAN => Some(Item::new(DEAD_BUBBLE_CORAL_FAN)), 
		DEAD_FIRE_CORAL_FAN => Some(Item::new(DEAD_FIRE_CORAL_FAN)), 
		DEAD_HORN_CORAL_FAN => Some(Item::new(DEAD_HORN_CORAL_FAN)), 
		BLUE_ICE => Some(Item::new(BLUE_ICE)), 
		CONDUIT => Some(Item::new(CONDUIT)), 
		POLISHED_GRANITE_STAIRS => Some(Item::new(POLISHED_GRANITE_STAIRS)), 
		SMOOTH_RED_SANDSTONE_STAIRS => Some(Item::new(SMOOTH_RED_SANDSTONE_STAIRS)), 
		MOSSY_STONE_BRICK_STAIRS => Some(Item::new(MOSSY_STONE_BRICK_STAIRS)), 
		POLISHED_DIORITE_STAIRS => Some(Item::new(POLISHED_DIORITE_STAIRS)), 
		MOSSY_COBBLESTONE_STAIRS => Some(Item::new(MOSSY_COBBLESTONE_STAIRS)), 
		END_STONE_BRICK_STAIRS => Some(Item::new(END_STONE_BRICK_STAIRS)), 
		STONE_STAIRS => Some(Item::new(STONE_STAIRS)), 
		SMOOTH_SANDSTONE_STAIRS => Some(Item::new(SMOOTH_SANDSTONE_STAIRS)), 
		SMOOTH_QUARTZ_STAIRS => Some(Item::new(SMOOTH_QUARTZ_STAIRS)), 
		GRANITE_STAIRS => Some(Item::new(GRANITE_STAIRS)), 
		ANDESITE_STAIRS => Some(Item::new(ANDESITE_STAIRS)), 
		RED_NETHER_BRICK_STAIRS => Some(Item::new(RED_NETHER_BRICK_STAIRS)), 
		POLISHED_ANDESITE_STAIRS => Some(Item::new(POLISHED_ANDESITE_STAIRS)), 
		DIORITE_STAIRS => Some(Item::new(DIORITE_STAIRS)), 
		POLISHED_GRANITE_SLAB => Some(Item::new(POLISHED_GRANITE_SLAB)), 
		SMOOTH_RED_SANDSTONE_SLAB => Some(Item::new(SMOOTH_RED_SANDSTONE_SLAB)), 
		MOSSY_STONE_BRICK_SLAB => Some(Item::new(MOSSY_STONE_BRICK_SLAB)), 
		POLISHED_DIORITE_SLAB => Some(Item::new(POLISHED_DIORITE_SLAB)), 
		MOSSY_COBBLESTONE_SLAB => Some(Item::new(MOSSY_COBBLESTONE_SLAB)), 
		END_STONE_BRICK_SLAB => Some(Item::new(END_STONE_BRICK_SLAB)), 
		SMOOTH_SANDSTONE_SLAB => Some(Item::new(SMOOTH_SANDSTONE_SLAB)), 
		SMOOTH_QUARTZ_SLAB => Some(Item::new(SMOOTH_QUARTZ_SLAB)), 
		GRANITE_SLAB => Some(Item::new(GRANITE_SLAB)), 
		ANDESITE_SLAB => Some(Item::new(ANDESITE_SLAB)), 
		RED_NETHER_BRICK_SLAB => Some(Item::new(RED_NETHER_BRICK_SLAB)), 
		POLISHED_ANDESITE_SLAB => Some(Item::new(POLISHED_ANDESITE_SLAB)), 
		DIORITE_SLAB => Some(Item::new(DIORITE_SLAB)), 
		SCAFFOLDING => Some(Item::new(SCAFFOLDING)), 
		IRON_DOOR => Some(Item::new(IRON_DOOR)), 
		OAK_DOOR => Some(Item::new(OAK_DOOR)), 
		SPRUCE_DOOR => Some(Item::new(SPRUCE_DOOR)), 
		BIRCH_DOOR => Some(Item::new(BIRCH_DOOR)), 
		JUNGLE_DOOR => Some(Item::new(JUNGLE_DOOR)), 
		ACACIA_DOOR => Some(Item::new(ACACIA_DOOR)), 
		DARK_OAK_DOOR => Some(Item::new(DARK_OAK_DOOR)), 
		CRIMSON_DOOR => Some(Item::new(CRIMSON_DOOR)), 
		WARPED_DOOR => Some(Item::new(WARPED_DOOR)), 
		REPEATER => Some(Item::new(REPEATER)), 
		COMPARATOR => Some(Item::new(COMPARATOR)), 
		STRUCTURE_BLOCK => Some(Item::new(STRUCTURE_BLOCK)), 
		JIGSAW => Some(Item::new(JIGSAW)), 
		TURTLE_HELMET => Some(Item::new(TURTLE_HELMET)), 
		SCUTE => Some(Item::new(SCUTE)), 
		IRON_SHOVEL => Some(Item::new(IRON_SHOVEL)), 
		IRON_PICKAXE => Some(Item::new(IRON_PICKAXE)), 
		IRON_AXE => Some(Item::new(IRON_AXE)), 
		FLINT_AND_STEEL => Some(Item::new(FLINT_AND_STEEL)), 
		APPLE => Some(Item::new(APPLE)), 
		BOW => Some(Item::new(BOW)), 
		ARROW => Some(Item::new(ARROW)), 
		COAL => Some(Item::new(COAL)), 
		CHARCOAL => Some(Item::new(CHARCOAL)), 
		DIAMOND => Some(Item::new(DIAMOND)), 
		IRON_INGOT => Some(Item::new(IRON_INGOT)), 
		GOLD_INGOT => Some(Item::new(GOLD_INGOT)), 
		IRON_SWORD => Some(Item::new(IRON_SWORD)), 
		WOODEN_SWORD => Some(Item::new(WOODEN_SWORD)), 
		WOODEN_SHOVEL => Some(Item::new(WOODEN_SHOVEL)), 
		WOODEN_PICKAXE => Some(Item::new(WOODEN_PICKAXE)), 
		WOODEN_AXE => Some(Item::new(WOODEN_AXE)), 
		STONE_SWORD => Some(Item::new(STONE_SWORD)), 
		STONE_SHOVEL => Some(Item::new(STONE_SHOVEL)), 
		STONE_PICKAXE => Some(Item::new(STONE_PICKAXE)), 
		STONE_AXE => Some(Item::new(STONE_AXE)), 
		DIAMOND_SWORD => Some(Item::new(DIAMOND_SWORD)), 
		DIAMOND_SHOVEL => Some(Item::new(DIAMOND_SHOVEL)), 
		DIAMOND_PICKAXE => Some(Item::new(DIAMOND_PICKAXE)), 
		DIAMOND_AXE => Some(Item::new(DIAMOND_AXE)), 
		STICK => Some(Item::new(STICK)), 
		BOWL => Some(Item::new(BOWL)), 
		MUSHROOM_STEW => Some(Item::new(MUSHROOM_STEW)), 
		GOLDEN_SWORD => Some(Item::new(GOLDEN_SWORD)), 
		GOLDEN_SHOVEL => Some(Item::new(GOLDEN_SHOVEL)), 
		GOLDEN_PICKAXE => Some(Item::new(GOLDEN_PICKAXE)), 
		GOLDEN_AXE => Some(Item::new(GOLDEN_AXE)), 
		NETHERITE_SWORD => Some(Item::new(NETHERITE_SWORD)), 
		NETHERITE_SHOVEL => Some(Item::new(NETHERITE_SHOVEL)), 
		NETHERITE_PICKAXE => Some(Item::new(NETHERITE_PICKAXE)), 
		NETHERITE_AXE => Some(Item::new(NETHERITE_AXE)), 
		STRING => Some(Item::new(STRING)), 
		FEATHER => Some(Item::new(FEATHER)), 
		GUNPOWDER => Some(Item::new(GUNPOWDER)), 
		WOODEN_HOE => Some(Item::new(WOODEN_HOE)), 
		STONE_HOE => Some(Item::new(STONE_HOE)), 
		IRON_HOE => Some(Item::new(IRON_HOE)), 
		DIAMOND_HOE => Some(Item::new(DIAMOND_HOE)), 
		GOLDEN_HOE => Some(Item::new(GOLDEN_HOE)), 
		NETHERITE_HOE => Some(Item::new(NETHERITE_HOE)), 
		WHEAT_SEEDS => Some(Item::new(WHEAT_SEEDS)), 
		WHEAT => Some(Item::new(WHEAT)), 
		BREAD => Some(Item::new(BREAD)), 
		LEATHER_HELMET => Some(Item::new(LEATHER_HELMET)), 
		LEATHER_CHESTPLATE => Some(Item::new(LEATHER_CHESTPLATE)), 
		LEATHER_LEGGINGS => Some(Item::new(LEATHER_LEGGINGS)), 
		LEATHER_BOOTS => Some(Item::new(LEATHER_BOOTS)), 
		CHAINMAIL_HELMET => Some(Item::new(CHAINMAIL_HELMET)), 
		CHAINMAIL_CHESTPLATE => Some(Item::new(CHAINMAIL_CHESTPLATE)), 
		CHAINMAIL_LEGGINGS => Some(Item::new(CHAINMAIL_LEGGINGS)), 
		CHAINMAIL_BOOTS => Some(Item::new(CHAINMAIL_BOOTS)), 
		IRON_HELMET => Some(Item::new(IRON_HELMET)), 
		IRON_CHESTPLATE => Some(Item::new(IRON_CHESTPLATE)), 
		IRON_LEGGINGS => Some(Item::new(IRON_LEGGINGS)), 
		IRON_BOOTS => Some(Item::new(IRON_BOOTS)), 
		DIAMOND_HELMET => Some(Item::new(DIAMOND_HELMET)), 
		DIAMOND_CHESTPLATE => Some(Item::new(DIAMOND_CHESTPLATE)), 
		DIAMOND_LEGGINGS => Some(Item::new(DIAMOND_LEGGINGS)), 
		DIAMOND_BOOTS => Some(Item::new(DIAMOND_BOOTS)), 
		GOLDEN_HELMET => Some(Item::new(GOLDEN_HELMET)), 
		GOLDEN_CHESTPLATE => Some(Item::new(GOLDEN_CHESTPLATE)), 
		GOLDEN_LEGGINGS => Some(Item::new(GOLDEN_LEGGINGS)), 
		GOLDEN_BOOTS => Some(Item::new(GOLDEN_BOOTS)), 
		NETHERITE_HELMET => Some(Item::new(NETHERITE_HELMET)), 
		NETHERITE_CHESTPLATE => Some(Item::new(NETHERITE_CHESTPLATE)), 
		NETHERITE_LEGGINGS => Some(Item::new(NETHERITE_LEGGINGS)), 
		NETHERITE_BOOTS => Some(Item::new(NETHERITE_BOOTS)), 
		FLINT => Some(Item::new(FLINT)), 
		PORKCHOP => Some(Item::new(PORKCHOP)), 
		COOKED_PORKCHOP => Some(Item::new(COOKED_PORKCHOP)), 
		PAINTING => Some(Item::new(PAINTING)), 
		GOLDEN_APPLE => Some(Item::new(GOLDEN_APPLE)), 
		ENCHANTED_GOLDEN_APPLE => Some(Item::new(ENCHANTED_GOLDEN_APPLE)), 
		OAK_SIGN => Some(Item::new(OAK_SIGN)), 
		SPRUCE_SIGN => Some(Item::new(SPRUCE_SIGN)), 
		BIRCH_SIGN => Some(Item::new(BIRCH_SIGN)), 
		JUNGLE_SIGN => Some(Item::new(JUNGLE_SIGN)), 
		ACACIA_SIGN => Some(Item::new(ACACIA_SIGN)), 
		DARK_OAK_SIGN => Some(Item::new(DARK_OAK_SIGN)), 
		CRIMSON_SIGN => Some(Item::new(CRIMSON_SIGN)), 
		WARPED_SIGN => Some(Item::new(WARPED_SIGN)), 
		BUCKET => Some(Item::new(BUCKET)), 
		WATER_BUCKET => Some(Item::new(WATER_BUCKET)), 
		LAVA_BUCKET => Some(Item::new(LAVA_BUCKET)), 
		MINECART => Some(Item::new(MINECART)), 
		SADDLE => Some(Item::new(SADDLE)), 
		REDSTONE => Some(Item::new(REDSTONE)), 
		SNOWBALL => Some(Item::new(SNOWBALL)), 
		OAK_BOAT => Some(Item::new(OAK_BOAT)), 
		LEATHER => Some(Item::new(LEATHER)), 
		MILK_BUCKET => Some(Item::new(MILK_BUCKET)), 
		PUFFERFISH_BUCKET => Some(Item::new(PUFFERFISH_BUCKET)), 
		SALMON_BUCKET => Some(Item::new(SALMON_BUCKET)), 
		COD_BUCKET => Some(Item::new(COD_BUCKET)), 
		TROPICAL_FISH_BUCKET => Some(Item::new(TROPICAL_FISH_BUCKET)), 
		BRICK => Some(Item::new(BRICK)), 
		CLAY_BALL => Some(Item::new(CLAY_BALL)), 
		SUGAR_CANE => Some(Item::new(SUGAR_CANE)), 
		KELP => Some(Item::new(KELP)), 
		DRIED_KELP_BLOCK => Some(Item::new(DRIED_KELP_BLOCK)), 
		BAMBOO => Some(Item::new(BAMBOO)), 
		PAPER => Some(Item::new(PAPER)), 
		BOOK => Some(Item::new(BOOK)), 
		SLIME_BALL => Some(Item::new(SLIME_BALL)), 
		CHEST_MINECART => Some(Item::new(CHEST_MINECART)), 
		FURNACE_MINECART => Some(Item::new(FURNACE_MINECART)), 
		EGG => Some(Item::new(EGG)), 
		COMPASS => Some(Item::new(COMPASS)), 
		FISHING_ROD => Some(Item::new(FISHING_ROD)), 
		CLOCK => Some(Item::new(CLOCK)), 
		GLOWSTONE_DUST => Some(Item::new(GLOWSTONE_DUST)), 
		COD => Some(Item::new(COD)), 
		SALMON => Some(Item::new(SALMON)), 
		TROPICAL_FISH => Some(Item::new(TROPICAL_FISH)), 
		PUFFERFISH => Some(Item::new(PUFFERFISH)), 
		COOKED_COD => Some(Item::new(COOKED_COD)), 
		COOKED_SALMON => Some(Item::new(COOKED_SALMON)), 
		INK_SAC => Some(Item::new(INK_SAC)), 
		RED_DYE => Some(Item::new(RED_DYE)), 
		GREEN_DYE => Some(Item::new(GREEN_DYE)), 
		COCOA_BEANS => Some(Item::new(COCOA_BEANS)), 
		LAPIS_LAZULI => Some(Item::new(LAPIS_LAZULI)), 
		PURPLE_DYE => Some(Item::new(PURPLE_DYE)), 
		CYAN_DYE => Some(Item::new(CYAN_DYE)), 
		LIGHT_GRAY_DYE => Some(Item::new(LIGHT_GRAY_DYE)), 
		GRAY_DYE => Some(Item::new(GRAY_DYE)), 
		PINK_DYE => Some(Item::new(PINK_DYE)), 
		LIME_DYE => Some(Item::new(LIME_DYE)), 
		YELLOW_DYE => Some(Item::new(YELLOW_DYE)), 
		LIGHT_BLUE_DYE => Some(Item::new(LIGHT_BLUE_DYE)), 
		MAGENTA_DYE => Some(Item::new(MAGENTA_DYE)), 
		ORANGE_DYE => Some(Item::new(ORANGE_DYE)), 
		BONE_MEAL => Some(Item::new(BONE_MEAL)), 
		BLUE_DYE => Some(Item::new(BLUE_DYE)), 
		BROWN_DYE => Some(Item::new(BROWN_DYE)), 
		BLACK_DYE => Some(Item::new(BLACK_DYE)), 
		WHITE_DYE => Some(Item::new(WHITE_DYE)), 
		BONE => Some(Item::new(BONE)), 
		SUGAR => Some(Item::new(SUGAR)), 
		CAKE => Some(Item::new(CAKE)), 
		WHITE_BED => Some(Item::new(WHITE_BED)), 
		ORANGE_BED => Some(Item::new(ORANGE_BED)), 
		MAGENTA_BED => Some(Item::new(MAGENTA_BED)), 
		LIGHT_BLUE_BED => Some(Item::new(LIGHT_BLUE_BED)), 
		YELLOW_BED => Some(Item::new(YELLOW_BED)), 
		LIME_BED => Some(Item::new(LIME_BED)), 
		PINK_BED => Some(Item::new(PINK_BED)), 
		GRAY_BED => Some(Item::new(GRAY_BED)), 
		LIGHT_GRAY_BED => Some(Item::new(LIGHT_GRAY_BED)), 
		CYAN_BED => Some(Item::new(CYAN_BED)), 
		PURPLE_BED => Some(Item::new(PURPLE_BED)), 
		BLUE_BED => Some(Item::new(BLUE_BED)), 
		BROWN_BED => Some(Item::new(BROWN_BED)), 
		GREEN_BED => Some(Item::new(GREEN_BED)), 
		RED_BED => Some(Item::new(RED_BED)), 
		BLACK_BED => Some(Item::new(BLACK_BED)), 
		COOKIE => Some(Item::new(COOKIE)), 
		FILLED_MAP => Some(Item::new(FILLED_MAP)), 
		SHEARS => Some(Item::new(SHEARS)), 
		MELON_SLICE => Some(Item::new(MELON_SLICE)), 
		DRIED_KELP => Some(Item::new(DRIED_KELP)), 
		PUMPKIN_SEEDS => Some(Item::new(PUMPKIN_SEEDS)), 
		MELON_SEEDS => Some(Item::new(MELON_SEEDS)), 
		BEEF => Some(Item::new(BEEF)), 
		COOKED_BEEF => Some(Item::new(COOKED_BEEF)), 
		CHICKEN => Some(Item::new(CHICKEN)), 
		COOKED_CHICKEN => Some(Item::new(COOKED_CHICKEN)), 
		ROTTEN_FLESH => Some(Item::new(ROTTEN_FLESH)), 
		ENDER_PEARL => Some(Item::new(ENDER_PEARL)), 
		BLAZE_ROD => Some(Item::new(BLAZE_ROD)), 
		GHAST_TEAR => Some(Item::new(GHAST_TEAR)), 
		GOLD_NUGGET => Some(Item::new(GOLD_NUGGET)), 
		NETHER_WART => Some(Item::new(NETHER_WART)), 
		POTION => Some(Item::new(POTION)), 
		GLASS_BOTTLE => Some(Item::new(GLASS_BOTTLE)), 
		SPIDER_EYE => Some(Item::new(SPIDER_EYE)), 
		FERMENTED_SPIDER_EYE => Some(Item::new(FERMENTED_SPIDER_EYE)), 
		BLAZE_POWDER => Some(Item::new(BLAZE_POWDER)), 
		MAGMA_CREAM => Some(Item::new(MAGMA_CREAM)), 
		BREWING_STAND => Some(Item::new(BREWING_STAND)), 
		CAULDRON => Some(Item::new(CAULDRON)), 
		ENDER_EYE => Some(Item::new(ENDER_EYE)), 
		GLISTERING_MELON_SLICE => Some(Item::new(GLISTERING_MELON_SLICE)), 
		BAT_SPAWN_EGG => Some(Item::new(BAT_SPAWN_EGG)), 
		BEE_SPAWN_EGG => Some(Item::new(BEE_SPAWN_EGG)), 
		BLAZE_SPAWN_EGG => Some(Item::new(BLAZE_SPAWN_EGG)), 
		CAT_SPAWN_EGG => Some(Item::new(CAT_SPAWN_EGG)), 
		CAVE_SPIDER_SPAWN_EGG => Some(Item::new(CAVE_SPIDER_SPAWN_EGG)), 
		CHICKEN_SPAWN_EGG => Some(Item::new(CHICKEN_SPAWN_EGG)), 
		COD_SPAWN_EGG => Some(Item::new(COD_SPAWN_EGG)), 
		COW_SPAWN_EGG => Some(Item::new(COW_SPAWN_EGG)), 
		CREEPER_SPAWN_EGG => Some(Item::new(CREEPER_SPAWN_EGG)), 
		DOLPHIN_SPAWN_EGG => Some(Item::new(DOLPHIN_SPAWN_EGG)), 
		DONKEY_SPAWN_EGG => Some(Item::new(DONKEY_SPAWN_EGG)), 
		DROWNED_SPAWN_EGG => Some(Item::new(DROWNED_SPAWN_EGG)), 
		ELDER_GUARDIAN_SPAWN_EGG => Some(Item::new(ELDER_GUARDIAN_SPAWN_EGG)), 
		ENDERMAN_SPAWN_EGG => Some(Item::new(ENDERMAN_SPAWN_EGG)), 
		ENDERMITE_SPAWN_EGG => Some(Item::new(ENDERMITE_SPAWN_EGG)), 
		EVOKER_SPAWN_EGG => Some(Item::new(EVOKER_SPAWN_EGG)), 
		FOX_SPAWN_EGG => Some(Item::new(FOX_SPAWN_EGG)), 
		GHAST_SPAWN_EGG => Some(Item::new(GHAST_SPAWN_EGG)), 
		GUARDIAN_SPAWN_EGG => Some(Item::new(GUARDIAN_SPAWN_EGG)), 
		HOGLIN_SPAWN_EGG => Some(Item::new(HOGLIN_SPAWN_EGG)), 
		HORSE_SPAWN_EGG => Some(Item::new(HORSE_SPAWN_EGG)), 
		HUSK_SPAWN_EGG => Some(Item::new(HUSK_SPAWN_EGG)), 
		LLAMA_SPAWN_EGG => Some(Item::new(LLAMA_SPAWN_EGG)), 
		MAGMA_CUBE_SPAWN_EGG => Some(Item::new(MAGMA_CUBE_SPAWN_EGG)), 
		MOOSHROOM_SPAWN_EGG => Some(Item::new(MOOSHROOM_SPAWN_EGG)), 
		MULE_SPAWN_EGG => Some(Item::new(MULE_SPAWN_EGG)), 
		OCELOT_SPAWN_EGG => Some(Item::new(OCELOT_SPAWN_EGG)), 
		PANDA_SPAWN_EGG => Some(Item::new(PANDA_SPAWN_EGG)), 
		PARROT_SPAWN_EGG => Some(Item::new(PARROT_SPAWN_EGG)), 
		PHANTOM_SPAWN_EGG => Some(Item::new(PHANTOM_SPAWN_EGG)), 
		PIG_SPAWN_EGG => Some(Item::new(PIG_SPAWN_EGG)), 
		PIGLIN_SPAWN_EGG => Some(Item::new(PIGLIN_SPAWN_EGG)), 
		PILLAGER_SPAWN_EGG => Some(Item::new(PILLAGER_SPAWN_EGG)), 
		POLAR_BEAR_SPAWN_EGG => Some(Item::new(POLAR_BEAR_SPAWN_EGG)), 
		PUFFERFISH_SPAWN_EGG => Some(Item::new(PUFFERFISH_SPAWN_EGG)), 
		RABBIT_SPAWN_EGG => Some(Item::new(RABBIT_SPAWN_EGG)), 
		RAVAGER_SPAWN_EGG => Some(Item::new(RAVAGER_SPAWN_EGG)), 
		SALMON_SPAWN_EGG => Some(Item::new(SALMON_SPAWN_EGG)), 
		SHEEP_SPAWN_EGG => Some(Item::new(SHEEP_SPAWN_EGG)), 
		SHULKER_SPAWN_EGG => Some(Item::new(SHULKER_SPAWN_EGG)), 
		SILVERFISH_SPAWN_EGG => Some(Item::new(SILVERFISH_SPAWN_EGG)), 
		SKELETON_SPAWN_EGG => Some(Item::new(SKELETON_SPAWN_EGG)), 
		SKELETON_HORSE_SPAWN_EGG => Some(Item::new(SKELETON_HORSE_SPAWN_EGG)), 
		SLIME_SPAWN_EGG => Some(Item::new(SLIME_SPAWN_EGG)), 
		SPIDER_SPAWN_EGG => Some(Item::new(SPIDER_SPAWN_EGG)), 
		SQUID_SPAWN_EGG => Some(Item::new(SQUID_SPAWN_EGG)), 
		STRAY_SPAWN_EGG => Some(Item::new(STRAY_SPAWN_EGG)), 
		TRADER_LLAMA_SPAWN_EGG => Some(Item::new(TRADER_LLAMA_SPAWN_EGG)), 
		TROPICAL_FISH_SPAWN_EGG => Some(Item::new(TROPICAL_FISH_SPAWN_EGG)), 
		TURTLE_SPAWN_EGG => Some(Item::new(TURTLE_SPAWN_EGG)), 
		VEX_SPAWN_EGG => Some(Item::new(VEX_SPAWN_EGG)), 
		VILLAGER_SPAWN_EGG => Some(Item::new(VILLAGER_SPAWN_EGG)), 
		VINDICATOR_SPAWN_EGG => Some(Item::new(VINDICATOR_SPAWN_EGG)), 
		WANDERING_TRADER_SPAWN_EGG => Some(Item::new(WANDERING_TRADER_SPAWN_EGG)), 
		WITCH_SPAWN_EGG => Some(Item::new(WITCH_SPAWN_EGG)), 
		WITHER_SKELETON_SPAWN_EGG => Some(Item::new(WITHER_SKELETON_SPAWN_EGG)), 
		WOLF_SPAWN_EGG => Some(Item::new(WOLF_SPAWN_EGG)), 
		ZOMBIE_SPAWN_EGG => Some(Item::new(ZOMBIE_SPAWN_EGG)), 
		ZOMBIE_HORSE_SPAWN_EGG => Some(Item::new(ZOMBIE_HORSE_SPAWN_EGG)), 
		ZOMBIFIED_PIGLIN_SPAWN_EGG => Some(Item::new(ZOMBIFIED_PIGLIN_SPAWN_EGG)), 
		ZOMBIE_VILLAGER_SPAWN_EGG => Some(Item::new(ZOMBIE_VILLAGER_SPAWN_EGG)), 
		EXPERIENCE_BOTTLE => Some(Item::new(EXPERIENCE_BOTTLE)), 
		FIRE_CHARGE => Some(Item::new(FIRE_CHARGE)), 
		WRITABLE_BOOK => Some(Item::new(WRITABLE_BOOK)), 
		WRITTEN_BOOK => Some(Item::new(WRITTEN_BOOK)), 
		EMERALD => Some(Item::new(EMERALD)), 
		ITEM_FRAME => Some(Item::new(ITEM_FRAME)), 
		FLOWER_POT => Some(Item::new(FLOWER_POT)), 
		CARROT => Some(Item::new(CARROT)), 
		POTATO => Some(Item::new(POTATO)), 
		BAKED_POTATO => Some(Item::new(BAKED_POTATO)), 
		POISONOUS_POTATO => Some(Item::new(POISONOUS_POTATO)), 
		MAP => Some(Item::new(MAP)), 
		GOLDEN_CARROT => Some(Item::new(GOLDEN_CARROT)), 
		SKELETON_SKULL => Some(Item::new(SKELETON_SKULL)), 
		WITHER_SKELETON_SKULL => Some(Item::new(WITHER_SKELETON_SKULL)), 
		PLAYER_HEAD => Some(Item::new(PLAYER_HEAD)), 
		ZOMBIE_HEAD => Some(Item::new(ZOMBIE_HEAD)), 
		CREEPER_HEAD => Some(Item::new(CREEPER_HEAD)), 
		DRAGON_HEAD => Some(Item::new(DRAGON_HEAD)), 
		CARROT_ON_A_STICK => Some(Item::new(CARROT_ON_A_STICK)), 
		NETHER_STAR => Some(Item::new(NETHER_STAR)), 
		PUMPKIN_PIE => Some(Item::new(PUMPKIN_PIE)), 
		FIREWORK_ROCKET => Some(Item::new(FIREWORK_ROCKET)), 
		FIREWORK_STAR => Some(Item::new(FIREWORK_STAR)), 
		ENCHANTED_BOOK => Some(Item::new(ENCHANTED_BOOK)), 
		NETHER_BRICK => Some(Item::new(NETHER_BRICK)), 
		QUARTZ => Some(Item::new(QUARTZ)), 
		TNT_MINECART => Some(Item::new(TNT_MINECART)), 
		HOPPER_MINECART => Some(Item::new(HOPPER_MINECART)), 
		PRISMARINE_SHARD => Some(Item::new(PRISMARINE_SHARD)), 
		PRISMARINE_CRYSTALS => Some(Item::new(PRISMARINE_CRYSTALS)), 
		RABBIT => Some(Item::new(RABBIT)), 
		COOKED_RABBIT => Some(Item::new(COOKED_RABBIT)), 
		RABBIT_STEW => Some(Item::new(RABBIT_STEW)), 
		RABBIT_FOOT => Some(Item::new(RABBIT_FOOT)), 
		RABBIT_HIDE => Some(Item::new(RABBIT_HIDE)), 
		ARMOR_STAND => Some(Item::new(ARMOR_STAND)), 
		IRON_HORSE_ARMOR => Some(Item::new(IRON_HORSE_ARMOR)), 
		GOLDEN_HORSE_ARMOR => Some(Item::new(GOLDEN_HORSE_ARMOR)), 
		DIAMOND_HORSE_ARMOR => Some(Item::new(DIAMOND_HORSE_ARMOR)), 
		LEATHER_HORSE_ARMOR => Some(Item::new(LEATHER_HORSE_ARMOR)), 
		LEAD => Some(Item::new(LEAD)), 
		NAME_TAG => Some(Item::new(NAME_TAG)), 
		COMMAND_BLOCK_MINECART => Some(Item::new(COMMAND_BLOCK_MINECART)), 
		MUTTON => Some(Item::new(MUTTON)), 
		COOKED_MUTTON => Some(Item::new(COOKED_MUTTON)), 
		WHITE_BANNER => Some(Item::new(WHITE_BANNER)), 
		ORANGE_BANNER => Some(Item::new(ORANGE_BANNER)), 
		MAGENTA_BANNER => Some(Item::new(MAGENTA_BANNER)), 
		LIGHT_BLUE_BANNER => Some(Item::new(LIGHT_BLUE_BANNER)), 
		YELLOW_BANNER => Some(Item::new(YELLOW_BANNER)), 
		LIME_BANNER => Some(Item::new(LIME_BANNER)), 
		PINK_BANNER => Some(Item::new(PINK_BANNER)), 
		GRAY_BANNER => Some(Item::new(GRAY_BANNER)), 
		LIGHT_GRAY_BANNER => Some(Item::new(LIGHT_GRAY_BANNER)), 
		CYAN_BANNER => Some(Item::new(CYAN_BANNER)), 
		PURPLE_BANNER => Some(Item::new(PURPLE_BANNER)), 
		BLUE_BANNER => Some(Item::new(BLUE_BANNER)), 
		BROWN_BANNER => Some(Item::new(BROWN_BANNER)), 
		GREEN_BANNER => Some(Item::new(GREEN_BANNER)), 
		RED_BANNER => Some(Item::new(RED_BANNER)), 
		BLACK_BANNER => Some(Item::new(BLACK_BANNER)), 
		END_CRYSTAL => Some(Item::new(END_CRYSTAL)), 
		CHORUS_FRUIT => Some(Item::new(CHORUS_FRUIT)), 
		POPPED_CHORUS_FRUIT => Some(Item::new(POPPED_CHORUS_FRUIT)), 
		BEETROOT => Some(Item::new(BEETROOT)), 
		BEETROOT_SEEDS => Some(Item::new(BEETROOT_SEEDS)), 
		BEETROOT_SOUP => Some(Item::new(BEETROOT_SOUP)), 
		DRAGON_BREATH => Some(Item::new(DRAGON_BREATH)), 
		SPLASH_POTION => Some(Item::new(SPLASH_POTION)), 
		SPECTRAL_ARROW => Some(Item::new(SPECTRAL_ARROW)), 
		TIPPED_ARROW => Some(Item::new(TIPPED_ARROW)), 
		LINGERING_POTION => Some(Item::new(LINGERING_POTION)), 
		SHIELD => Some(Item::new(SHIELD)), 
		ELYTRA => Some(Item::new(ELYTRA)), 
		SPRUCE_BOAT => Some(Item::new(SPRUCE_BOAT)), 
		BIRCH_BOAT => Some(Item::new(BIRCH_BOAT)), 
		JUNGLE_BOAT => Some(Item::new(JUNGLE_BOAT)), 
		ACACIA_BOAT => Some(Item::new(ACACIA_BOAT)), 
		DARK_OAK_BOAT => Some(Item::new(DARK_OAK_BOAT)), 
		TOTEM_OF_UNDYING => Some(Item::new(TOTEM_OF_UNDYING)), 
		SHULKER_SHELL => Some(Item::new(SHULKER_SHELL)), 
		IRON_NUGGET => Some(Item::new(IRON_NUGGET)), 
		KNOWLEDGE_BOOK => Some(Item::new(KNOWLEDGE_BOOK)), 
		DEBUG_STICK => Some(Item::new(DEBUG_STICK)), 
		MUSIC_DISC_13 => Some(Item::new(MUSIC_DISC_13)), 
		MUSIC_DISC_CAT => Some(Item::new(MUSIC_DISC_CAT)), 
		MUSIC_DISC_BLOCKS => Some(Item::new(MUSIC_DISC_BLOCKS)), 
		MUSIC_DISC_CHIRP => Some(Item::new(MUSIC_DISC_CHIRP)), 
		MUSIC_DISC_FAR => Some(Item::new(MUSIC_DISC_FAR)), 
		MUSIC_DISC_MALL => Some(Item::new(MUSIC_DISC_MALL)), 
		MUSIC_DISC_MELLOHI => Some(Item::new(MUSIC_DISC_MELLOHI)), 
		MUSIC_DISC_STAL => Some(Item::new(MUSIC_DISC_STAL)), 
		MUSIC_DISC_STRAD => Some(Item::new(MUSIC_DISC_STRAD)), 
		MUSIC_DISC_WARD => Some(Item::new(MUSIC_DISC_WARD)), 
		MUSIC_DISC_11 => Some(Item::new(MUSIC_DISC_11)), 
		MUSIC_DISC_WAIT => Some(Item::new(MUSIC_DISC_WAIT)), 
		TRIDENT => Some(Item::new(TRIDENT)), 
		PHANTOM_MEMBRANE => Some(Item::new(PHANTOM_MEMBRANE)), 
		NAUTILUS_SHELL => Some(Item::new(NAUTILUS_SHELL)), 
		HEART_OF_THE_SEA => Some(Item::new(HEART_OF_THE_SEA)), 
		CROSSBOW => Some(Item::new(CROSSBOW)), 
		SUSPICIOUS_STEW => Some(Item::new(SUSPICIOUS_STEW)), 
		LOOM => Some(Item::new(LOOM)), 
		FLOWER_BANNER_PATTERN => Some(Item::new(FLOWER_BANNER_PATTERN)), 
		CREEPER_BANNER_PATTERN => Some(Item::new(CREEPER_BANNER_PATTERN)), 
		SKULL_BANNER_PATTERN => Some(Item::new(SKULL_BANNER_PATTERN)), 
		MOJANG_BANNER_PATTERN => Some(Item::new(MOJANG_BANNER_PATTERN)), 
		GLOBE_BANNER_PATTERN => Some(Item::new(GLOBE_BANNER_PATTERN)), 
		COMPOSTER => Some(Item::new(COMPOSTER)), 
		BARREL => Some(Item::new(BARREL)), 
		SMOKER => Some(Item::new(SMOKER)), 
		BLAST_FURNACE => Some(Item::new(BLAST_FURNACE)), 
		CARTOGRAPHY_TABLE => Some(Item::new(CARTOGRAPHY_TABLE)), 
		FLETCHING_TABLE => Some(Item::new(FLETCHING_TABLE)), 
		GRINDSTONE => Some(Item::new(GRINDSTONE)), 
		LECTERN => Some(Item::new(LECTERN)), 
		SMITHING_TABLE => Some(Item::new(SMITHING_TABLE)), 
		STONECUTTER => Some(Item::new(STONECUTTER)), 
		BELL => Some(Item::new(BELL)), 
		LANTERN => Some(Item::new(LANTERN)), 
		SOUL_FIRE_LANTERN => Some(Item::new(SOUL_FIRE_LANTERN)), 
		SWEET_BERRIES => Some(Item::new(SWEET_BERRIES)), 
		CAMPFIRE => Some(Item::new(CAMPFIRE)), 
		SHROOMLIGHT => Some(Item::new(SHROOMLIGHT)), 
		HONEYCOMB => Some(Item::new(HONEYCOMB)), 
		BEE_NEST => Some(Item::new(BEE_NEST)), 
		BEEHIVE => Some(Item::new(BEEHIVE)), 
		HONEY_BOTTLE => Some(Item::new(HONEY_BOTTLE)), 
		HONEY_BLOCK => Some(Item::new(HONEY_BLOCK)), 
		HONEYCOMB_BLOCK => Some(Item::new(HONEYCOMB_BLOCK)), 
		NETHERITE_BLOCK => Some(Item::new(NETHERITE_BLOCK)), 
		ANCIENT_DEBRIS => Some(Item::new(ANCIENT_DEBRIS)), 
		NETHERITE_INGOT => Some(Item::new(NETHERITE_INGOT)), 
		NETHERITE_SCRAP => Some(Item::new(NETHERITE_SCRAP)), 
		TARGET => Some(Item::new(TARGET)), 
		CRYING_OBSIDIAN => Some(Item::new(CRYING_OBSIDIAN)), 
		RESPAWN_ANCHOR => Some(Item::new(RESPAWN_ANCHOR)), 
		_ => None
	}
}

pub fn by_identifier(id: Identifier) -> Option<Item> {
	for item in get_items().iter() {
		if item.get_namespaced_id() == id {
			return Some(item.clone())
		}
	}
	return None
}

impl Item {
	pub fn get_namespaced_id(&self) -> Identifier {
		return Identifier::new("minecraft", match self.get_id() {
			AIR => "air",
			STONE => "stone",
			GRANITE => "granite",
			POLISHED_GRANITE => "polished_granite",
			DIORITE => "diorite",
			POLISHED_DIORITE => "polished_diorite",
			ANDESITE => "andesite",
			POLISHED_ANDESITE => "polished_andesite",
			GRASS_BLOCK => "grass_block",
			DIRT => "dirt",
			COARSE_DIRT => "coarse_dirt",
			PODZOL => "podzol",
			CRIMSON_NYLIUM => "crimson_nylium",
			WARPED_NYLIUM => "warped_nylium",
			COBBLESTONE => "cobblestone",
			OAK_PLANKS => "oak_planks",
			SPRUCE_PLANKS => "spruce_planks",
			BIRCH_PLANKS => "birch_planks",
			JUNGLE_PLANKS => "jungle_planks",
			ACACIA_PLANKS => "acacia_planks",
			DARK_OAK_PLANKS => "dark_oak_planks",
			CRIMSON_PLANKS => "crimson_planks",
			WARPED_PLANKS => "warped_planks",
			OAK_SAPLING => "oak_sapling",
			SPRUCE_SAPLING => "spruce_sapling",
			BIRCH_SAPLING => "birch_sapling",
			JUNGLE_SAPLING => "jungle_sapling",
			ACACIA_SAPLING => "acacia_sapling",
			DARK_OAK_SAPLING => "dark_oak_sapling",
			BEDROCK => "bedrock",
			SAND => "sand",
			RED_SAND => "red_sand",
			GRAVEL => "gravel",
			GOLD_ORE => "gold_ore",
			IRON_ORE => "iron_ore",
			COAL_ORE => "coal_ore",
			NETHER_GOLD_ORE => "nether_gold_ore",
			OAK_LOG => "oak_log",
			SPRUCE_LOG => "spruce_log",
			BIRCH_LOG => "birch_log",
			JUNGLE_LOG => "jungle_log",
			ACACIA_LOG => "acacia_log",
			DARK_OAK_LOG => "dark_oak_log",
			CRIMSON_STEM => "crimson_stem",
			WARPED_STEM => "warped_stem",
			STRIPPED_OAK_LOG => "stripped_oak_log",
			STRIPPED_SPRUCE_LOG => "stripped_spruce_log",
			STRIPPED_BIRCH_LOG => "stripped_birch_log",
			STRIPPED_JUNGLE_LOG => "stripped_jungle_log",
			STRIPPED_ACACIA_LOG => "stripped_acacia_log",
			STRIPPED_DARK_OAK_LOG => "stripped_dark_oak_log",
			STRIPPED_CRIMSON_STEM => "stripped_crimson_stem",
			STRIPPED_WARPED_STEM => "stripped_warped_stem",
			STRIPPED_OAK_WOOD => "stripped_oak_wood",
			STRIPPED_SPRUCE_WOOD => "stripped_spruce_wood",
			STRIPPED_BIRCH_WOOD => "stripped_birch_wood",
			STRIPPED_JUNGLE_WOOD => "stripped_jungle_wood",
			STRIPPED_ACACIA_WOOD => "stripped_acacia_wood",
			STRIPPED_DARK_OAK_WOOD => "stripped_dark_oak_wood",
			STRIPPED_CRIMSON_HYPHAE => "stripped_crimson_hyphae",
			STRIPPED_WARPED_HYPHAE => "stripped_warped_hyphae",
			OAK_WOOD => "oak_wood",
			SPRUCE_WOOD => "spruce_wood",
			BIRCH_WOOD => "birch_wood",
			JUNGLE_WOOD => "jungle_wood",
			ACACIA_WOOD => "acacia_wood",
			DARK_OAK_WOOD => "dark_oak_wood",
			CRIMSON_HYPHAE => "crimson_hyphae",
			WARPED_HYPHAE => "warped_hyphae",
			OAK_LEAVES => "oak_leaves",
			SPRUCE_LEAVES => "spruce_leaves",
			BIRCH_LEAVES => "birch_leaves",
			JUNGLE_LEAVES => "jungle_leaves",
			ACACIA_LEAVES => "acacia_leaves",
			DARK_OAK_LEAVES => "dark_oak_leaves",
			SPONGE => "sponge",
			WET_SPONGE => "wet_sponge",
			GLASS => "glass",
			LAPIS_ORE => "lapis_ore",
			LAPIS_BLOCK => "lapis_block",
			DISPENSER => "dispenser",
			SANDSTONE => "sandstone",
			CHISELED_SANDSTONE => "chiseled_sandstone",
			CUT_SANDSTONE => "cut_sandstone",
			NOTE_BLOCK => "note_block",
			POWERED_RAIL => "powered_rail",
			DETECTOR_RAIL => "detector_rail",
			STICKY_PISTON => "sticky_piston",
			COBWEB => "cobweb",
			GRASS => "grass",
			FERN => "fern",
			DEAD_BUSH => "dead_bush",
			SEAGRASS => "seagrass",
			SEA_PICKLE => "sea_pickle",
			PISTON => "piston",
			WHITE_WOOL => "white_wool",
			ORANGE_WOOL => "orange_wool",
			MAGENTA_WOOL => "magenta_wool",
			LIGHT_BLUE_WOOL => "light_blue_wool",
			YELLOW_WOOL => "yellow_wool",
			LIME_WOOL => "lime_wool",
			PINK_WOOL => "pink_wool",
			GRAY_WOOL => "gray_wool",
			LIGHT_GRAY_WOOL => "light_gray_wool",
			CYAN_WOOL => "cyan_wool",
			PURPLE_WOOL => "purple_wool",
			BLUE_WOOL => "blue_wool",
			BROWN_WOOL => "brown_wool",
			GREEN_WOOL => "green_wool",
			RED_WOOL => "red_wool",
			BLACK_WOOL => "black_wool",
			DANDELION => "dandelion",
			POPPY => "poppy",
			BLUE_ORCHID => "blue_orchid",
			ALLIUM => "allium",
			AZURE_BLUET => "azure_bluet",
			RED_TULIP => "red_tulip",
			ORANGE_TULIP => "orange_tulip",
			WHITE_TULIP => "white_tulip",
			PINK_TULIP => "pink_tulip",
			OXEYE_DAISY => "oxeye_daisy",
			CORNFLOWER => "cornflower",
			LILY_OF_THE_VALLEY => "lily_of_the_valley",
			WITHER_ROSE => "wither_rose",
			BROWN_MUSHROOM => "brown_mushroom",
			RED_MUSHROOM => "red_mushroom",
			CRIMSON_FUNGUS => "crimson_fungus",
			WARPED_FUNGUS => "warped_fungus",
			CRIMSON_ROOTS => "crimson_roots",
			WARPED_ROOTS => "warped_roots",
			NETHER_SPROUTS => "nether_sprouts",
			WEEPING_VINES => "weeping_vines",
			TWISTING_VINES => "twisting_vines",
			GOLD_BLOCK => "gold_block",
			IRON_BLOCK => "iron_block",
			OAK_SLAB => "oak_slab",
			SPRUCE_SLAB => "spruce_slab",
			BIRCH_SLAB => "birch_slab",
			JUNGLE_SLAB => "jungle_slab",
			ACACIA_SLAB => "acacia_slab",
			DARK_OAK_SLAB => "dark_oak_slab",
			CRIMSON_SLAB => "crimson_slab",
			WARPED_SLAB => "warped_slab",
			STONE_SLAB => "stone_slab",
			SMOOTH_STONE_SLAB => "smooth_stone_slab",
			SANDSTONE_SLAB => "sandstone_slab",
			CUT_SANDSTONE_SLAB => "cut_sandstone_slab",
			PETRIFIED_OAK_SLAB => "petrified_oak_slab",
			COBBLESTONE_SLAB => "cobblestone_slab",
			BRICK_SLAB => "brick_slab",
			STONE_BRICK_SLAB => "stone_brick_slab",
			NETHER_BRICK_SLAB => "nether_brick_slab",
			QUARTZ_SLAB => "quartz_slab",
			RED_SANDSTONE_SLAB => "red_sandstone_slab",
			CUT_RED_SANDSTONE_SLAB => "cut_red_sandstone_slab",
			PURPUR_SLAB => "purpur_slab",
			PRISMARINE_SLAB => "prismarine_slab",
			PRISMARINE_BRICK_SLAB => "prismarine_brick_slab",
			DARK_PRISMARINE_SLAB => "dark_prismarine_slab",
			SMOOTH_QUARTZ => "smooth_quartz",
			SMOOTH_RED_SANDSTONE => "smooth_red_sandstone",
			SMOOTH_SANDSTONE => "smooth_sandstone",
			SMOOTH_STONE => "smooth_stone",
			BRICKS => "bricks",
			TNT => "tnt",
			BOOKSHELF => "bookshelf",
			MOSSY_COBBLESTONE => "mossy_cobblestone",
			OBSIDIAN => "obsidian",
			TORCH => "torch",
			END_ROD => "end_rod",
			CHORUS_PLANT => "chorus_plant",
			CHORUS_FLOWER => "chorus_flower",
			PURPUR_BLOCK => "purpur_block",
			PURPUR_PILLAR => "purpur_pillar",
			PURPUR_STAIRS => "purpur_stairs",
			SPAWNER => "spawner",
			OAK_STAIRS => "oak_stairs",
			CHEST => "chest",
			DIAMOND_ORE => "diamond_ore",
			DIAMOND_BLOCK => "diamond_block",
			CRAFTING_TABLE => "crafting_table",
			FARMLAND => "farmland",
			FURNACE => "furnace",
			LADDER => "ladder",
			RAIL => "rail",
			COBBLESTONE_STAIRS => "cobblestone_stairs",
			LEVER => "lever",
			STONE_PRESSURE_PLATE => "stone_pressure_plate",
			OAK_PRESSURE_PLATE => "oak_pressure_plate",
			SPRUCE_PRESSURE_PLATE => "spruce_pressure_plate",
			BIRCH_PRESSURE_PLATE => "birch_pressure_plate",
			JUNGLE_PRESSURE_PLATE => "jungle_pressure_plate",
			ACACIA_PRESSURE_PLATE => "acacia_pressure_plate",
			DARK_OAK_PRESSURE_PLATE => "dark_oak_pressure_plate",
			CRIMSON_PRESSURE_PLATE => "crimson_pressure_plate",
			WARPED_PRESSURE_PLATE => "warped_pressure_plate",
			REDSTONE_ORE => "redstone_ore",
			REDSTONE_TORCH => "redstone_torch",
			STONE_BUTTON => "stone_button",
			SNOW => "snow",
			ICE => "ice",
			SNOW_BLOCK => "snow_block",
			CACTUS => "cactus",
			CLAY => "clay",
			JUKEBOX => "jukebox",
			OAK_FENCE => "oak_fence",
			SPRUCE_FENCE => "spruce_fence",
			BIRCH_FENCE => "birch_fence",
			JUNGLE_FENCE => "jungle_fence",
			ACACIA_FENCE => "acacia_fence",
			DARK_OAK_FENCE => "dark_oak_fence",
			CRIMSON_FENCE => "crimson_fence",
			WARPED_FENCE => "warped_fence",
			PUMPKIN => "pumpkin",
			CARVED_PUMPKIN => "carved_pumpkin",
			NETHERRACK => "netherrack",
			SOUL_SAND => "soul_sand",
			SOUL_SOIL => "soul_soil",
			BASALT => "basalt",
			POLISHED_BASALT => "polished_basalt",
			SOUL_FIRE_TORCH => "soul_fire_torch",
			GLOWSTONE => "glowstone",
			JACK_O_LANTERN => "jack_o_lantern",
			OAK_TRAPDOOR => "oak_trapdoor",
			SPRUCE_TRAPDOOR => "spruce_trapdoor",
			BIRCH_TRAPDOOR => "birch_trapdoor",
			JUNGLE_TRAPDOOR => "jungle_trapdoor",
			ACACIA_TRAPDOOR => "acacia_trapdoor",
			DARK_OAK_TRAPDOOR => "dark_oak_trapdoor",
			CRIMSON_TRAPDOOR => "crimson_trapdoor",
			WARPED_TRAPDOOR => "warped_trapdoor",
			INFESTED_STONE => "infested_stone",
			INFESTED_COBBLESTONE => "infested_cobblestone",
			INFESTED_STONE_BRICKS => "infested_stone_bricks",
			INFESTED_MOSSY_STONE_BRICKS => "infested_mossy_stone_bricks",
			INFESTED_CRACKED_STONE_BRICKS => "infested_cracked_stone_bricks",
			INFESTED_CHISELED_STONE_BRICKS => "infested_chiseled_stone_bricks",
			STONE_BRICKS => "stone_bricks",
			MOSSY_STONE_BRICKS => "mossy_stone_bricks",
			CRACKED_STONE_BRICKS => "cracked_stone_bricks",
			CHISELED_STONE_BRICKS => "chiseled_stone_bricks",
			BROWN_MUSHROOM_BLOCK => "brown_mushroom_block",
			RED_MUSHROOM_BLOCK => "red_mushroom_block",
			MUSHROOM_STEM => "mushroom_stem",
			IRON_BARS => "iron_bars",
			GLASS_PANE => "glass_pane",
			MELON => "melon",
			VINE => "vine",
			OAK_FENCE_GATE => "oak_fence_gate",
			SPRUCE_FENCE_GATE => "spruce_fence_gate",
			BIRCH_FENCE_GATE => "birch_fence_gate",
			JUNGLE_FENCE_GATE => "jungle_fence_gate",
			ACACIA_FENCE_GATE => "acacia_fence_gate",
			DARK_OAK_FENCE_GATE => "dark_oak_fence_gate",
			CRIMSON_FENCE_GATE => "crimson_fence_gate",
			WARPED_FENCE_GATE => "warped_fence_gate",
			BRICK_STAIRS => "brick_stairs",
			STONE_BRICK_STAIRS => "stone_brick_stairs",
			MYCELIUM => "mycelium",
			LILY_PAD => "lily_pad",
			NETHER_BRICKS => "nether_bricks",
			NETHER_BRICK_FENCE => "nether_brick_fence",
			NETHER_BRICK_STAIRS => "nether_brick_stairs",
			ENCHANTING_TABLE => "enchanting_table",
			END_PORTAL_FRAME => "end_portal_frame",
			END_STONE => "end_stone",
			END_STONE_BRICKS => "end_stone_bricks",
			DRAGON_EGG => "dragon_egg",
			REDSTONE_LAMP => "redstone_lamp",
			SANDSTONE_STAIRS => "sandstone_stairs",
			EMERALD_ORE => "emerald_ore",
			ENDER_CHEST => "ender_chest",
			TRIPWIRE_HOOK => "tripwire_hook",
			EMERALD_BLOCK => "emerald_block",
			SPRUCE_STAIRS => "spruce_stairs",
			BIRCH_STAIRS => "birch_stairs",
			JUNGLE_STAIRS => "jungle_stairs",
			CRIMSON_STAIRS => "crimson_stairs",
			WARPED_STAIRS => "warped_stairs",
			COMMAND_BLOCK => "command_block",
			BEACON => "beacon",
			COBBLESTONE_WALL => "cobblestone_wall",
			MOSSY_COBBLESTONE_WALL => "mossy_cobblestone_wall",
			BRICK_WALL => "brick_wall",
			PRISMARINE_WALL => "prismarine_wall",
			RED_SANDSTONE_WALL => "red_sandstone_wall",
			MOSSY_STONE_BRICK_WALL => "mossy_stone_brick_wall",
			GRANITE_WALL => "granite_wall",
			STONE_BRICK_WALL => "stone_brick_wall",
			NETHER_BRICK_WALL => "nether_brick_wall",
			ANDESITE_WALL => "andesite_wall",
			RED_NETHER_BRICK_WALL => "red_nether_brick_wall",
			SANDSTONE_WALL => "sandstone_wall",
			END_STONE_BRICK_WALL => "end_stone_brick_wall",
			DIORITE_WALL => "diorite_wall",
			OAK_BUTTON => "oak_button",
			SPRUCE_BUTTON => "spruce_button",
			BIRCH_BUTTON => "birch_button",
			JUNGLE_BUTTON => "jungle_button",
			ACACIA_BUTTON => "acacia_button",
			DARK_OAK_BUTTON => "dark_oak_button",
			CRIMSON_BUTTON => "crimson_button",
			WARPED_BUTTON => "warped_button",
			ANVIL => "anvil",
			CHIPPED_ANVIL => "chipped_anvil",
			DAMAGED_ANVIL => "damaged_anvil",
			TRAPPED_CHEST => "trapped_chest",
			LIGHT_WEIGHTED_PRESSURE_PLATE => "light_weighted_pressure_plate",
			HEAVY_WEIGHTED_PRESSURE_PLATE => "heavy_weighted_pressure_plate",
			DAYLIGHT_DETECTOR => "daylight_detector",
			REDSTONE_BLOCK => "redstone_block",
			NETHER_QUARTZ_ORE => "nether_quartz_ore",
			HOPPER => "hopper",
			CHISELED_QUARTZ_BLOCK => "chiseled_quartz_block",
			QUARTZ_BLOCK => "quartz_block",
			QUARTZ_PILLAR => "quartz_pillar",
			QUARTZ_STAIRS => "quartz_stairs",
			ACTIVATOR_RAIL => "activator_rail",
			DROPPER => "dropper",
			WHITE_TERRACOTTA => "white_terracotta",
			ORANGE_TERRACOTTA => "orange_terracotta",
			MAGENTA_TERRACOTTA => "magenta_terracotta",
			LIGHT_BLUE_TERRACOTTA => "light_blue_terracotta",
			YELLOW_TERRACOTTA => "yellow_terracotta",
			LIME_TERRACOTTA => "lime_terracotta",
			PINK_TERRACOTTA => "pink_terracotta",
			GRAY_TERRACOTTA => "gray_terracotta",
			LIGHT_GRAY_TERRACOTTA => "light_gray_terracotta",
			CYAN_TERRACOTTA => "cyan_terracotta",
			PURPLE_TERRACOTTA => "purple_terracotta",
			BLUE_TERRACOTTA => "blue_terracotta",
			BROWN_TERRACOTTA => "brown_terracotta",
			GREEN_TERRACOTTA => "green_terracotta",
			RED_TERRACOTTA => "red_terracotta",
			BLACK_TERRACOTTA => "black_terracotta",
			BARRIER => "barrier",
			IRON_TRAPDOOR => "iron_trapdoor",
			HAY_BLOCK => "hay_block",
			WHITE_CARPET => "white_carpet",
			ORANGE_CARPET => "orange_carpet",
			MAGENTA_CARPET => "magenta_carpet",
			LIGHT_BLUE_CARPET => "light_blue_carpet",
			YELLOW_CARPET => "yellow_carpet",
			LIME_CARPET => "lime_carpet",
			PINK_CARPET => "pink_carpet",
			GRAY_CARPET => "gray_carpet",
			LIGHT_GRAY_CARPET => "light_gray_carpet",
			CYAN_CARPET => "cyan_carpet",
			PURPLE_CARPET => "purple_carpet",
			BLUE_CARPET => "blue_carpet",
			BROWN_CARPET => "brown_carpet",
			GREEN_CARPET => "green_carpet",
			RED_CARPET => "red_carpet",
			BLACK_CARPET => "black_carpet",
			TERRACOTTA => "terracotta",
			COAL_BLOCK => "coal_block",
			PACKED_ICE => "packed_ice",
			ACACIA_STAIRS => "acacia_stairs",
			DARK_OAK_STAIRS => "dark_oak_stairs",
			SLIME_BLOCK => "slime_block",
			GRASS_PATH => "grass_path",
			SUNFLOWER => "sunflower",
			LILAC => "lilac",
			ROSE_BUSH => "rose_bush",
			PEONY => "peony",
			TALL_GRASS => "tall_grass",
			LARGE_FERN => "large_fern",
			WHITE_STAINED_GLASS => "white_stained_glass",
			ORANGE_STAINED_GLASS => "orange_stained_glass",
			MAGENTA_STAINED_GLASS => "magenta_stained_glass",
			LIGHT_BLUE_STAINED_GLASS => "light_blue_stained_glass",
			YELLOW_STAINED_GLASS => "yellow_stained_glass",
			LIME_STAINED_GLASS => "lime_stained_glass",
			PINK_STAINED_GLASS => "pink_stained_glass",
			GRAY_STAINED_GLASS => "gray_stained_glass",
			LIGHT_GRAY_STAINED_GLASS => "light_gray_stained_glass",
			CYAN_STAINED_GLASS => "cyan_stained_glass",
			PURPLE_STAINED_GLASS => "purple_stained_glass",
			BLUE_STAINED_GLASS => "blue_stained_glass",
			BROWN_STAINED_GLASS => "brown_stained_glass",
			GREEN_STAINED_GLASS => "green_stained_glass",
			RED_STAINED_GLASS => "red_stained_glass",
			BLACK_STAINED_GLASS => "black_stained_glass",
			WHITE_STAINED_GLASS_PANE => "white_stained_glass_pane",
			ORANGE_STAINED_GLASS_PANE => "orange_stained_glass_pane",
			MAGENTA_STAINED_GLASS_PANE => "magenta_stained_glass_pane",
			LIGHT_BLUE_STAINED_GLASS_PANE => "light_blue_stained_glass_pane",
			YELLOW_STAINED_GLASS_PANE => "yellow_stained_glass_pane",
			LIME_STAINED_GLASS_PANE => "lime_stained_glass_pane",
			PINK_STAINED_GLASS_PANE => "pink_stained_glass_pane",
			GRAY_STAINED_GLASS_PANE => "gray_stained_glass_pane",
			LIGHT_GRAY_STAINED_GLASS_PANE => "light_gray_stained_glass_pane",
			CYAN_STAINED_GLASS_PANE => "cyan_stained_glass_pane",
			PURPLE_STAINED_GLASS_PANE => "purple_stained_glass_pane",
			BLUE_STAINED_GLASS_PANE => "blue_stained_glass_pane",
			BROWN_STAINED_GLASS_PANE => "brown_stained_glass_pane",
			GREEN_STAINED_GLASS_PANE => "green_stained_glass_pane",
			RED_STAINED_GLASS_PANE => "red_stained_glass_pane",
			BLACK_STAINED_GLASS_PANE => "black_stained_glass_pane",
			PRISMARINE => "prismarine",
			PRISMARINE_BRICKS => "prismarine_bricks",
			DARK_PRISMARINE => "dark_prismarine",
			PRISMARINE_STAIRS => "prismarine_stairs",
			PRISMARINE_BRICK_STAIRS => "prismarine_brick_stairs",
			DARK_PRISMARINE_STAIRS => "dark_prismarine_stairs",
			SEA_LANTERN => "sea_lantern",
			RED_SANDSTONE => "red_sandstone",
			CHISELED_RED_SANDSTONE => "chiseled_red_sandstone",
			CUT_RED_SANDSTONE => "cut_red_sandstone",
			RED_SANDSTONE_STAIRS => "red_sandstone_stairs",
			REPEATING_COMMAND_BLOCK => "repeating_command_block",
			CHAIN_COMMAND_BLOCK => "chain_command_block",
			MAGMA_BLOCK => "magma_block",
			NETHER_WART_BLOCK => "nether_wart_block",
			WARPED_WART_BLOCK => "warped_wart_block",
			RED_NETHER_BRICKS => "red_nether_bricks",
			BONE_BLOCK => "bone_block",
			STRUCTURE_VOID => "structure_void",
			OBSERVER => "observer",
			SHULKER_BOX => "shulker_box",
			WHITE_SHULKER_BOX => "white_shulker_box",
			ORANGE_SHULKER_BOX => "orange_shulker_box",
			MAGENTA_SHULKER_BOX => "magenta_shulker_box",
			LIGHT_BLUE_SHULKER_BOX => "light_blue_shulker_box",
			YELLOW_SHULKER_BOX => "yellow_shulker_box",
			LIME_SHULKER_BOX => "lime_shulker_box",
			PINK_SHULKER_BOX => "pink_shulker_box",
			GRAY_SHULKER_BOX => "gray_shulker_box",
			LIGHT_GRAY_SHULKER_BOX => "light_gray_shulker_box",
			CYAN_SHULKER_BOX => "cyan_shulker_box",
			PURPLE_SHULKER_BOX => "purple_shulker_box",
			BLUE_SHULKER_BOX => "blue_shulker_box",
			BROWN_SHULKER_BOX => "brown_shulker_box",
			GREEN_SHULKER_BOX => "green_shulker_box",
			RED_SHULKER_BOX => "red_shulker_box",
			BLACK_SHULKER_BOX => "black_shulker_box",
			WHITE_GLAZED_TERRACOTTA => "white_glazed_terracotta",
			ORANGE_GLAZED_TERRACOTTA => "orange_glazed_terracotta",
			MAGENTA_GLAZED_TERRACOTTA => "magenta_glazed_terracotta",
			LIGHT_BLUE_GLAZED_TERRACOTTA => "light_blue_glazed_terracotta",
			YELLOW_GLAZED_TERRACOTTA => "yellow_glazed_terracotta",
			LIME_GLAZED_TERRACOTTA => "lime_glazed_terracotta",
			PINK_GLAZED_TERRACOTTA => "pink_glazed_terracotta",
			GRAY_GLAZED_TERRACOTTA => "gray_glazed_terracotta",
			LIGHT_GRAY_GLAZED_TERRACOTTA => "light_gray_glazed_terracotta",
			CYAN_GLAZED_TERRACOTTA => "cyan_glazed_terracotta",
			PURPLE_GLAZED_TERRACOTTA => "purple_glazed_terracotta",
			BLUE_GLAZED_TERRACOTTA => "blue_glazed_terracotta",
			BROWN_GLAZED_TERRACOTTA => "brown_glazed_terracotta",
			GREEN_GLAZED_TERRACOTTA => "green_glazed_terracotta",
			RED_GLAZED_TERRACOTTA => "red_glazed_terracotta",
			BLACK_GLAZED_TERRACOTTA => "black_glazed_terracotta",
			WHITE_CONCRETE => "white_concrete",
			ORANGE_CONCRETE => "orange_concrete",
			MAGENTA_CONCRETE => "magenta_concrete",
			LIGHT_BLUE_CONCRETE => "light_blue_concrete",
			YELLOW_CONCRETE => "yellow_concrete",
			LIME_CONCRETE => "lime_concrete",
			PINK_CONCRETE => "pink_concrete",
			GRAY_CONCRETE => "gray_concrete",
			LIGHT_GRAY_CONCRETE => "light_gray_concrete",
			CYAN_CONCRETE => "cyan_concrete",
			PURPLE_CONCRETE => "purple_concrete",
			BLUE_CONCRETE => "blue_concrete",
			BROWN_CONCRETE => "brown_concrete",
			GREEN_CONCRETE => "green_concrete",
			RED_CONCRETE => "red_concrete",
			BLACK_CONCRETE => "black_concrete",
			WHITE_CONCRETE_POWDER => "white_concrete_powder",
			ORANGE_CONCRETE_POWDER => "orange_concrete_powder",
			MAGENTA_CONCRETE_POWDER => "magenta_concrete_powder",
			LIGHT_BLUE_CONCRETE_POWDER => "light_blue_concrete_powder",
			YELLOW_CONCRETE_POWDER => "yellow_concrete_powder",
			LIME_CONCRETE_POWDER => "lime_concrete_powder",
			PINK_CONCRETE_POWDER => "pink_concrete_powder",
			GRAY_CONCRETE_POWDER => "gray_concrete_powder",
			LIGHT_GRAY_CONCRETE_POWDER => "light_gray_concrete_powder",
			CYAN_CONCRETE_POWDER => "cyan_concrete_powder",
			PURPLE_CONCRETE_POWDER => "purple_concrete_powder",
			BLUE_CONCRETE_POWDER => "blue_concrete_powder",
			BROWN_CONCRETE_POWDER => "brown_concrete_powder",
			GREEN_CONCRETE_POWDER => "green_concrete_powder",
			RED_CONCRETE_POWDER => "red_concrete_powder",
			BLACK_CONCRETE_POWDER => "black_concrete_powder",
			TURTLE_EGG => "turtle_egg",
			DEAD_TUBE_CORAL_BLOCK => "dead_tube_coral_block",
			DEAD_BRAIN_CORAL_BLOCK => "dead_brain_coral_block",
			DEAD_BUBBLE_CORAL_BLOCK => "dead_bubble_coral_block",
			DEAD_FIRE_CORAL_BLOCK => "dead_fire_coral_block",
			DEAD_HORN_CORAL_BLOCK => "dead_horn_coral_block",
			TUBE_CORAL_BLOCK => "tube_coral_block",
			BRAIN_CORAL_BLOCK => "brain_coral_block",
			BUBBLE_CORAL_BLOCK => "bubble_coral_block",
			FIRE_CORAL_BLOCK => "fire_coral_block",
			HORN_CORAL_BLOCK => "horn_coral_block",
			TUBE_CORAL => "tube_coral",
			BRAIN_CORAL => "brain_coral",
			BUBBLE_CORAL => "bubble_coral",
			FIRE_CORAL => "fire_coral",
			HORN_CORAL => "horn_coral",
			DEAD_BRAIN_CORAL => "dead_brain_coral",
			DEAD_BUBBLE_CORAL => "dead_bubble_coral",
			DEAD_FIRE_CORAL => "dead_fire_coral",
			DEAD_HORN_CORAL => "dead_horn_coral",
			DEAD_TUBE_CORAL => "dead_tube_coral",
			TUBE_CORAL_FAN => "tube_coral_fan",
			BRAIN_CORAL_FAN => "brain_coral_fan",
			BUBBLE_CORAL_FAN => "bubble_coral_fan",
			FIRE_CORAL_FAN => "fire_coral_fan",
			HORN_CORAL_FAN => "horn_coral_fan",
			DEAD_TUBE_CORAL_FAN => "dead_tube_coral_fan",
			DEAD_BRAIN_CORAL_FAN => "dead_brain_coral_fan",
			DEAD_BUBBLE_CORAL_FAN => "dead_bubble_coral_fan",
			DEAD_FIRE_CORAL_FAN => "dead_fire_coral_fan",
			DEAD_HORN_CORAL_FAN => "dead_horn_coral_fan",
			BLUE_ICE => "blue_ice",
			CONDUIT => "conduit",
			POLISHED_GRANITE_STAIRS => "polished_granite_stairs",
			SMOOTH_RED_SANDSTONE_STAIRS => "smooth_red_sandstone_stairs",
			MOSSY_STONE_BRICK_STAIRS => "mossy_stone_brick_stairs",
			POLISHED_DIORITE_STAIRS => "polished_diorite_stairs",
			MOSSY_COBBLESTONE_STAIRS => "mossy_cobblestone_stairs",
			END_STONE_BRICK_STAIRS => "end_stone_brick_stairs",
			STONE_STAIRS => "stone_stairs",
			SMOOTH_SANDSTONE_STAIRS => "smooth_sandstone_stairs",
			SMOOTH_QUARTZ_STAIRS => "smooth_quartz_stairs",
			GRANITE_STAIRS => "granite_stairs",
			ANDESITE_STAIRS => "andesite_stairs",
			RED_NETHER_BRICK_STAIRS => "red_nether_brick_stairs",
			POLISHED_ANDESITE_STAIRS => "polished_andesite_stairs",
			DIORITE_STAIRS => "diorite_stairs",
			POLISHED_GRANITE_SLAB => "polished_granite_slab",
			SMOOTH_RED_SANDSTONE_SLAB => "smooth_red_sandstone_slab",
			MOSSY_STONE_BRICK_SLAB => "mossy_stone_brick_slab",
			POLISHED_DIORITE_SLAB => "polished_diorite_slab",
			MOSSY_COBBLESTONE_SLAB => "mossy_cobblestone_slab",
			END_STONE_BRICK_SLAB => "end_stone_brick_slab",
			SMOOTH_SANDSTONE_SLAB => "smooth_sandstone_slab",
			SMOOTH_QUARTZ_SLAB => "smooth_quartz_slab",
			GRANITE_SLAB => "granite_slab",
			ANDESITE_SLAB => "andesite_slab",
			RED_NETHER_BRICK_SLAB => "red_nether_brick_slab",
			POLISHED_ANDESITE_SLAB => "polished_andesite_slab",
			DIORITE_SLAB => "diorite_slab",
			SCAFFOLDING => "scaffolding",
			IRON_DOOR => "iron_door",
			OAK_DOOR => "oak_door",
			SPRUCE_DOOR => "spruce_door",
			BIRCH_DOOR => "birch_door",
			JUNGLE_DOOR => "jungle_door",
			ACACIA_DOOR => "acacia_door",
			DARK_OAK_DOOR => "dark_oak_door",
			CRIMSON_DOOR => "crimson_door",
			WARPED_DOOR => "warped_door",
			REPEATER => "repeater",
			COMPARATOR => "comparator",
			STRUCTURE_BLOCK => "structure_block",
			JIGSAW => "jigsaw",
			TURTLE_HELMET => "turtle_helmet",
			SCUTE => "scute",
			IRON_SHOVEL => "iron_shovel",
			IRON_PICKAXE => "iron_pickaxe",
			IRON_AXE => "iron_axe",
			FLINT_AND_STEEL => "flint_and_steel",
			APPLE => "apple",
			BOW => "bow",
			ARROW => "arrow",
			COAL => "coal",
			CHARCOAL => "charcoal",
			DIAMOND => "diamond",
			IRON_INGOT => "iron_ingot",
			GOLD_INGOT => "gold_ingot",
			IRON_SWORD => "iron_sword",
			WOODEN_SWORD => "wooden_sword",
			WOODEN_SHOVEL => "wooden_shovel",
			WOODEN_PICKAXE => "wooden_pickaxe",
			WOODEN_AXE => "wooden_axe",
			STONE_SWORD => "stone_sword",
			STONE_SHOVEL => "stone_shovel",
			STONE_PICKAXE => "stone_pickaxe",
			STONE_AXE => "stone_axe",
			DIAMOND_SWORD => "diamond_sword",
			DIAMOND_SHOVEL => "diamond_shovel",
			DIAMOND_PICKAXE => "diamond_pickaxe",
			DIAMOND_AXE => "diamond_axe",
			STICK => "stick",
			BOWL => "bowl",
			MUSHROOM_STEW => "mushroom_stew",
			GOLDEN_SWORD => "golden_sword",
			GOLDEN_SHOVEL => "golden_shovel",
			GOLDEN_PICKAXE => "golden_pickaxe",
			GOLDEN_AXE => "golden_axe",
			NETHERITE_SWORD => "netherite_sword",
			NETHERITE_SHOVEL => "netherite_shovel",
			NETHERITE_PICKAXE => "netherite_pickaxe",
			NETHERITE_AXE => "netherite_axe",
			STRING => "string",
			FEATHER => "feather",
			GUNPOWDER => "gunpowder",
			WOODEN_HOE => "wooden_hoe",
			STONE_HOE => "stone_hoe",
			IRON_HOE => "iron_hoe",
			DIAMOND_HOE => "diamond_hoe",
			GOLDEN_HOE => "golden_hoe",
			NETHERITE_HOE => "netherite_hoe",
			WHEAT_SEEDS => "wheat_seeds",
			WHEAT => "wheat",
			BREAD => "bread",
			LEATHER_HELMET => "leather_helmet",
			LEATHER_CHESTPLATE => "leather_chestplate",
			LEATHER_LEGGINGS => "leather_leggings",
			LEATHER_BOOTS => "leather_boots",
			CHAINMAIL_HELMET => "chainmail_helmet",
			CHAINMAIL_CHESTPLATE => "chainmail_chestplate",
			CHAINMAIL_LEGGINGS => "chainmail_leggings",
			CHAINMAIL_BOOTS => "chainmail_boots",
			IRON_HELMET => "iron_helmet",
			IRON_CHESTPLATE => "iron_chestplate",
			IRON_LEGGINGS => "iron_leggings",
			IRON_BOOTS => "iron_boots",
			DIAMOND_HELMET => "diamond_helmet",
			DIAMOND_CHESTPLATE => "diamond_chestplate",
			DIAMOND_LEGGINGS => "diamond_leggings",
			DIAMOND_BOOTS => "diamond_boots",
			GOLDEN_HELMET => "golden_helmet",
			GOLDEN_CHESTPLATE => "golden_chestplate",
			GOLDEN_LEGGINGS => "golden_leggings",
			GOLDEN_BOOTS => "golden_boots",
			NETHERITE_HELMET => "netherite_helmet",
			NETHERITE_CHESTPLATE => "netherite_chestplate",
			NETHERITE_LEGGINGS => "netherite_leggings",
			NETHERITE_BOOTS => "netherite_boots",
			FLINT => "flint",
			PORKCHOP => "porkchop",
			COOKED_PORKCHOP => "cooked_porkchop",
			PAINTING => "painting",
			GOLDEN_APPLE => "golden_apple",
			ENCHANTED_GOLDEN_APPLE => "enchanted_golden_apple",
			OAK_SIGN => "oak_sign",
			SPRUCE_SIGN => "spruce_sign",
			BIRCH_SIGN => "birch_sign",
			JUNGLE_SIGN => "jungle_sign",
			ACACIA_SIGN => "acacia_sign",
			DARK_OAK_SIGN => "dark_oak_sign",
			CRIMSON_SIGN => "crimson_sign",
			WARPED_SIGN => "warped_sign",
			BUCKET => "bucket",
			WATER_BUCKET => "water_bucket",
			LAVA_BUCKET => "lava_bucket",
			MINECART => "minecart",
			SADDLE => "saddle",
			REDSTONE => "redstone",
			SNOWBALL => "snowball",
			OAK_BOAT => "oak_boat",
			LEATHER => "leather",
			MILK_BUCKET => "milk_bucket",
			PUFFERFISH_BUCKET => "pufferfish_bucket",
			SALMON_BUCKET => "salmon_bucket",
			COD_BUCKET => "cod_bucket",
			TROPICAL_FISH_BUCKET => "tropical_fish_bucket",
			BRICK => "brick",
			CLAY_BALL => "clay_ball",
			SUGAR_CANE => "sugar_cane",
			KELP => "kelp",
			DRIED_KELP_BLOCK => "dried_kelp_block",
			BAMBOO => "bamboo",
			PAPER => "paper",
			BOOK => "book",
			SLIME_BALL => "slime_ball",
			CHEST_MINECART => "chest_minecart",
			FURNACE_MINECART => "furnace_minecart",
			EGG => "egg",
			COMPASS => "compass",
			FISHING_ROD => "fishing_rod",
			CLOCK => "clock",
			GLOWSTONE_DUST => "glowstone_dust",
			COD => "cod",
			SALMON => "salmon",
			TROPICAL_FISH => "tropical_fish",
			PUFFERFISH => "pufferfish",
			COOKED_COD => "cooked_cod",
			COOKED_SALMON => "cooked_salmon",
			INK_SAC => "ink_sac",
			RED_DYE => "red_dye",
			GREEN_DYE => "green_dye",
			COCOA_BEANS => "cocoa_beans",
			LAPIS_LAZULI => "lapis_lazuli",
			PURPLE_DYE => "purple_dye",
			CYAN_DYE => "cyan_dye",
			LIGHT_GRAY_DYE => "light_gray_dye",
			GRAY_DYE => "gray_dye",
			PINK_DYE => "pink_dye",
			LIME_DYE => "lime_dye",
			YELLOW_DYE => "yellow_dye",
			LIGHT_BLUE_DYE => "light_blue_dye",
			MAGENTA_DYE => "magenta_dye",
			ORANGE_DYE => "orange_dye",
			BONE_MEAL => "bone_meal",
			BLUE_DYE => "blue_dye",
			BROWN_DYE => "brown_dye",
			BLACK_DYE => "black_dye",
			WHITE_DYE => "white_dye",
			BONE => "bone",
			SUGAR => "sugar",
			CAKE => "cake",
			WHITE_BED => "white_bed",
			ORANGE_BED => "orange_bed",
			MAGENTA_BED => "magenta_bed",
			LIGHT_BLUE_BED => "light_blue_bed",
			YELLOW_BED => "yellow_bed",
			LIME_BED => "lime_bed",
			PINK_BED => "pink_bed",
			GRAY_BED => "gray_bed",
			LIGHT_GRAY_BED => "light_gray_bed",
			CYAN_BED => "cyan_bed",
			PURPLE_BED => "purple_bed",
			BLUE_BED => "blue_bed",
			BROWN_BED => "brown_bed",
			GREEN_BED => "green_bed",
			RED_BED => "red_bed",
			BLACK_BED => "black_bed",
			COOKIE => "cookie",
			FILLED_MAP => "filled_map",
			SHEARS => "shears",
			MELON_SLICE => "melon_slice",
			DRIED_KELP => "dried_kelp",
			PUMPKIN_SEEDS => "pumpkin_seeds",
			MELON_SEEDS => "melon_seeds",
			BEEF => "beef",
			COOKED_BEEF => "cooked_beef",
			CHICKEN => "chicken",
			COOKED_CHICKEN => "cooked_chicken",
			ROTTEN_FLESH => "rotten_flesh",
			ENDER_PEARL => "ender_pearl",
			BLAZE_ROD => "blaze_rod",
			GHAST_TEAR => "ghast_tear",
			GOLD_NUGGET => "gold_nugget",
			NETHER_WART => "nether_wart",
			POTION => "potion",
			GLASS_BOTTLE => "glass_bottle",
			SPIDER_EYE => "spider_eye",
			FERMENTED_SPIDER_EYE => "fermented_spider_eye",
			BLAZE_POWDER => "blaze_powder",
			MAGMA_CREAM => "magma_cream",
			BREWING_STAND => "brewing_stand",
			CAULDRON => "cauldron",
			ENDER_EYE => "ender_eye",
			GLISTERING_MELON_SLICE => "glistering_melon_slice",
			BAT_SPAWN_EGG => "bat_spawn_egg",
			BEE_SPAWN_EGG => "bee_spawn_egg",
			BLAZE_SPAWN_EGG => "blaze_spawn_egg",
			CAT_SPAWN_EGG => "cat_spawn_egg",
			CAVE_SPIDER_SPAWN_EGG => "cave_spider_spawn_egg",
			CHICKEN_SPAWN_EGG => "chicken_spawn_egg",
			COD_SPAWN_EGG => "cod_spawn_egg",
			COW_SPAWN_EGG => "cow_spawn_egg",
			CREEPER_SPAWN_EGG => "creeper_spawn_egg",
			DOLPHIN_SPAWN_EGG => "dolphin_spawn_egg",
			DONKEY_SPAWN_EGG => "donkey_spawn_egg",
			DROWNED_SPAWN_EGG => "drowned_spawn_egg",
			ELDER_GUARDIAN_SPAWN_EGG => "elder_guardian_spawn_egg",
			ENDERMAN_SPAWN_EGG => "enderman_spawn_egg",
			ENDERMITE_SPAWN_EGG => "endermite_spawn_egg",
			EVOKER_SPAWN_EGG => "evoker_spawn_egg",
			FOX_SPAWN_EGG => "fox_spawn_egg",
			GHAST_SPAWN_EGG => "ghast_spawn_egg",
			GUARDIAN_SPAWN_EGG => "guardian_spawn_egg",
			HOGLIN_SPAWN_EGG => "hoglin_spawn_egg",
			HORSE_SPAWN_EGG => "horse_spawn_egg",
			HUSK_SPAWN_EGG => "husk_spawn_egg",
			LLAMA_SPAWN_EGG => "llama_spawn_egg",
			MAGMA_CUBE_SPAWN_EGG => "magma_cube_spawn_egg",
			MOOSHROOM_SPAWN_EGG => "mooshroom_spawn_egg",
			MULE_SPAWN_EGG => "mule_spawn_egg",
			OCELOT_SPAWN_EGG => "ocelot_spawn_egg",
			PANDA_SPAWN_EGG => "panda_spawn_egg",
			PARROT_SPAWN_EGG => "parrot_spawn_egg",
			PHANTOM_SPAWN_EGG => "phantom_spawn_egg",
			PIG_SPAWN_EGG => "pig_spawn_egg",
			PIGLIN_SPAWN_EGG => "piglin_spawn_egg",
			PILLAGER_SPAWN_EGG => "pillager_spawn_egg",
			POLAR_BEAR_SPAWN_EGG => "polar_bear_spawn_egg",
			PUFFERFISH_SPAWN_EGG => "pufferfish_spawn_egg",
			RABBIT_SPAWN_EGG => "rabbit_spawn_egg",
			RAVAGER_SPAWN_EGG => "ravager_spawn_egg",
			SALMON_SPAWN_EGG => "salmon_spawn_egg",
			SHEEP_SPAWN_EGG => "sheep_spawn_egg",
			SHULKER_SPAWN_EGG => "shulker_spawn_egg",
			SILVERFISH_SPAWN_EGG => "silverfish_spawn_egg",
			SKELETON_SPAWN_EGG => "skeleton_spawn_egg",
			SKELETON_HORSE_SPAWN_EGG => "skeleton_horse_spawn_egg",
			SLIME_SPAWN_EGG => "slime_spawn_egg",
			SPIDER_SPAWN_EGG => "spider_spawn_egg",
			SQUID_SPAWN_EGG => "squid_spawn_egg",
			STRAY_SPAWN_EGG => "stray_spawn_egg",
			TRADER_LLAMA_SPAWN_EGG => "trader_llama_spawn_egg",
			TROPICAL_FISH_SPAWN_EGG => "tropical_fish_spawn_egg",
			TURTLE_SPAWN_EGG => "turtle_spawn_egg",
			VEX_SPAWN_EGG => "vex_spawn_egg",
			VILLAGER_SPAWN_EGG => "villager_spawn_egg",
			VINDICATOR_SPAWN_EGG => "vindicator_spawn_egg",
			WANDERING_TRADER_SPAWN_EGG => "wandering_trader_spawn_egg",
			WITCH_SPAWN_EGG => "witch_spawn_egg",
			WITHER_SKELETON_SPAWN_EGG => "wither_skeleton_spawn_egg",
			WOLF_SPAWN_EGG => "wolf_spawn_egg",
			ZOMBIE_SPAWN_EGG => "zombie_spawn_egg",
			ZOMBIE_HORSE_SPAWN_EGG => "zombie_horse_spawn_egg",
			ZOMBIFIED_PIGLIN_SPAWN_EGG => "zombified_piglin_spawn_egg",
			ZOMBIE_VILLAGER_SPAWN_EGG => "zombie_villager_spawn_egg",
			EXPERIENCE_BOTTLE => "experience_bottle",
			FIRE_CHARGE => "fire_charge",
			WRITABLE_BOOK => "writable_book",
			WRITTEN_BOOK => "written_book",
			EMERALD => "emerald",
			ITEM_FRAME => "item_frame",
			FLOWER_POT => "flower_pot",
			CARROT => "carrot",
			POTATO => "potato",
			BAKED_POTATO => "baked_potato",
			POISONOUS_POTATO => "poisonous_potato",
			MAP => "map",
			GOLDEN_CARROT => "golden_carrot",
			SKELETON_SKULL => "skeleton_skull",
			WITHER_SKELETON_SKULL => "wither_skeleton_skull",
			PLAYER_HEAD => "player_head",
			ZOMBIE_HEAD => "zombie_head",
			CREEPER_HEAD => "creeper_head",
			DRAGON_HEAD => "dragon_head",
			CARROT_ON_A_STICK => "carrot_on_a_stick",
			NETHER_STAR => "nether_star",
			PUMPKIN_PIE => "pumpkin_pie",
			FIREWORK_ROCKET => "firework_rocket",
			FIREWORK_STAR => "firework_star",
			ENCHANTED_BOOK => "enchanted_book",
			NETHER_BRICK => "nether_brick",
			QUARTZ => "quartz",
			TNT_MINECART => "tnt_minecart",
			HOPPER_MINECART => "hopper_minecart",
			PRISMARINE_SHARD => "prismarine_shard",
			PRISMARINE_CRYSTALS => "prismarine_crystals",
			RABBIT => "rabbit",
			COOKED_RABBIT => "cooked_rabbit",
			RABBIT_STEW => "rabbit_stew",
			RABBIT_FOOT => "rabbit_foot",
			RABBIT_HIDE => "rabbit_hide",
			ARMOR_STAND => "armor_stand",
			IRON_HORSE_ARMOR => "iron_horse_armor",
			GOLDEN_HORSE_ARMOR => "golden_horse_armor",
			DIAMOND_HORSE_ARMOR => "diamond_horse_armor",
			LEATHER_HORSE_ARMOR => "leather_horse_armor",
			LEAD => "lead",
			NAME_TAG => "name_tag",
			COMMAND_BLOCK_MINECART => "command_block_minecart",
			MUTTON => "mutton",
			COOKED_MUTTON => "cooked_mutton",
			WHITE_BANNER => "white_banner",
			ORANGE_BANNER => "orange_banner",
			MAGENTA_BANNER => "magenta_banner",
			LIGHT_BLUE_BANNER => "light_blue_banner",
			YELLOW_BANNER => "yellow_banner",
			LIME_BANNER => "lime_banner",
			PINK_BANNER => "pink_banner",
			GRAY_BANNER => "gray_banner",
			LIGHT_GRAY_BANNER => "light_gray_banner",
			CYAN_BANNER => "cyan_banner",
			PURPLE_BANNER => "purple_banner",
			BLUE_BANNER => "blue_banner",
			BROWN_BANNER => "brown_banner",
			GREEN_BANNER => "green_banner",
			RED_BANNER => "red_banner",
			BLACK_BANNER => "black_banner",
			END_CRYSTAL => "end_crystal",
			CHORUS_FRUIT => "chorus_fruit",
			POPPED_CHORUS_FRUIT => "popped_chorus_fruit",
			BEETROOT => "beetroot",
			BEETROOT_SEEDS => "beetroot_seeds",
			BEETROOT_SOUP => "beetroot_soup",
			DRAGON_BREATH => "dragon_breath",
			SPLASH_POTION => "splash_potion",
			SPECTRAL_ARROW => "spectral_arrow",
			TIPPED_ARROW => "tipped_arrow",
			LINGERING_POTION => "lingering_potion",
			SHIELD => "shield",
			ELYTRA => "elytra",
			SPRUCE_BOAT => "spruce_boat",
			BIRCH_BOAT => "birch_boat",
			JUNGLE_BOAT => "jungle_boat",
			ACACIA_BOAT => "acacia_boat",
			DARK_OAK_BOAT => "dark_oak_boat",
			TOTEM_OF_UNDYING => "totem_of_undying",
			SHULKER_SHELL => "shulker_shell",
			IRON_NUGGET => "iron_nugget",
			KNOWLEDGE_BOOK => "knowledge_book",
			DEBUG_STICK => "debug_stick",
			MUSIC_DISC_13 => "music_disc_13",
			MUSIC_DISC_CAT => "music_disc_cat",
			MUSIC_DISC_BLOCKS => "music_disc_blocks",
			MUSIC_DISC_CHIRP => "music_disc_chirp",
			MUSIC_DISC_FAR => "music_disc_far",
			MUSIC_DISC_MALL => "music_disc_mall",
			MUSIC_DISC_MELLOHI => "music_disc_mellohi",
			MUSIC_DISC_STAL => "music_disc_stal",
			MUSIC_DISC_STRAD => "music_disc_strad",
			MUSIC_DISC_WARD => "music_disc_ward",
			MUSIC_DISC_11 => "music_disc_11",
			MUSIC_DISC_WAIT => "music_disc_wait",
			TRIDENT => "trident",
			PHANTOM_MEMBRANE => "phantom_membrane",
			NAUTILUS_SHELL => "nautilus_shell",
			HEART_OF_THE_SEA => "heart_of_the_sea",
			CROSSBOW => "crossbow",
			SUSPICIOUS_STEW => "suspicious_stew",
			LOOM => "loom",
			FLOWER_BANNER_PATTERN => "flower_banner_pattern",
			CREEPER_BANNER_PATTERN => "creeper_banner_pattern",
			SKULL_BANNER_PATTERN => "skull_banner_pattern",
			MOJANG_BANNER_PATTERN => "mojang_banner_pattern",
			GLOBE_BANNER_PATTERN => "globe_banner_pattern",
			COMPOSTER => "composter",
			BARREL => "barrel",
			SMOKER => "smoker",
			BLAST_FURNACE => "blast_furnace",
			CARTOGRAPHY_TABLE => "cartography_table",
			FLETCHING_TABLE => "fletching_table",
			GRINDSTONE => "grindstone",
			LECTERN => "lectern",
			SMITHING_TABLE => "smithing_table",
			STONECUTTER => "stonecutter",
			BELL => "bell",
			LANTERN => "lantern",
			SOUL_FIRE_LANTERN => "soul_fire_lantern",
			SWEET_BERRIES => "sweet_berries",
			CAMPFIRE => "campfire",
			SHROOMLIGHT => "shroomlight",
			HONEYCOMB => "honeycomb",
			BEE_NEST => "bee_nest",
			BEEHIVE => "beehive",
			HONEY_BOTTLE => "honey_bottle",
			HONEY_BLOCK => "honey_block",
			HONEYCOMB_BLOCK => "honeycomb_block",
			NETHERITE_BLOCK => "netherite_block",
			ANCIENT_DEBRIS => "ancient_debris",
			NETHERITE_INGOT => "netherite_ingot",
			NETHERITE_SCRAP => "netherite_scrap",
			TARGET => "target",
			CRYING_OBSIDIAN => "crying_obsidian",
			RESPAWN_ANCHOR => "respawn_anchor",
			_ => "air"
		})
	}
}

pub const AIR: i32 = 0;
pub const STONE: i32 = 1;
pub const GRANITE: i32 = 2;
pub const POLISHED_GRANITE: i32 = 3;
pub const DIORITE: i32 = 4;
pub const POLISHED_DIORITE: i32 = 5;
pub const ANDESITE: i32 = 6;
pub const POLISHED_ANDESITE: i32 = 7;
pub const GRASS_BLOCK: i32 = 8;
pub const DIRT: i32 = 9;
pub const COARSE_DIRT: i32 = 10;
pub const PODZOL: i32 = 11;
pub const CRIMSON_NYLIUM: i32 = 12;
pub const WARPED_NYLIUM: i32 = 13;
pub const COBBLESTONE: i32 = 14;
pub const OAK_PLANKS: i32 = 15;
pub const SPRUCE_PLANKS: i32 = 16;
pub const BIRCH_PLANKS: i32 = 17;
pub const JUNGLE_PLANKS: i32 = 18;
pub const ACACIA_PLANKS: i32 = 19;
pub const DARK_OAK_PLANKS: i32 = 20;
pub const CRIMSON_PLANKS: i32 = 21;
pub const WARPED_PLANKS: i32 = 22;
pub const OAK_SAPLING: i32 = 23;
pub const SPRUCE_SAPLING: i32 = 24;
pub const BIRCH_SAPLING: i32 = 25;
pub const JUNGLE_SAPLING: i32 = 26;
pub const ACACIA_SAPLING: i32 = 27;
pub const DARK_OAK_SAPLING: i32 = 28;
pub const BEDROCK: i32 = 29;
pub const SAND: i32 = 30;
pub const RED_SAND: i32 = 31;
pub const GRAVEL: i32 = 32;
pub const GOLD_ORE: i32 = 33;
pub const IRON_ORE: i32 = 34;
pub const COAL_ORE: i32 = 35;
pub const NETHER_GOLD_ORE: i32 = 36;
pub const OAK_LOG: i32 = 37;
pub const SPRUCE_LOG: i32 = 38;
pub const BIRCH_LOG: i32 = 39;
pub const JUNGLE_LOG: i32 = 40;
pub const ACACIA_LOG: i32 = 41;
pub const DARK_OAK_LOG: i32 = 42;
pub const CRIMSON_STEM: i32 = 43;
pub const WARPED_STEM: i32 = 44;
pub const STRIPPED_OAK_LOG: i32 = 45;
pub const STRIPPED_SPRUCE_LOG: i32 = 46;
pub const STRIPPED_BIRCH_LOG: i32 = 47;
pub const STRIPPED_JUNGLE_LOG: i32 = 48;
pub const STRIPPED_ACACIA_LOG: i32 = 49;
pub const STRIPPED_DARK_OAK_LOG: i32 = 50;
pub const STRIPPED_CRIMSON_STEM: i32 = 51;
pub const STRIPPED_WARPED_STEM: i32 = 52;
pub const STRIPPED_OAK_WOOD: i32 = 53;
pub const STRIPPED_SPRUCE_WOOD: i32 = 54;
pub const STRIPPED_BIRCH_WOOD: i32 = 55;
pub const STRIPPED_JUNGLE_WOOD: i32 = 56;
pub const STRIPPED_ACACIA_WOOD: i32 = 57;
pub const STRIPPED_DARK_OAK_WOOD: i32 = 58;
pub const STRIPPED_CRIMSON_HYPHAE: i32 = 59;
pub const STRIPPED_WARPED_HYPHAE: i32 = 60;
pub const OAK_WOOD: i32 = 61;
pub const SPRUCE_WOOD: i32 = 62;
pub const BIRCH_WOOD: i32 = 63;
pub const JUNGLE_WOOD: i32 = 64;
pub const ACACIA_WOOD: i32 = 65;
pub const DARK_OAK_WOOD: i32 = 66;
pub const CRIMSON_HYPHAE: i32 = 67;
pub const WARPED_HYPHAE: i32 = 68;
pub const OAK_LEAVES: i32 = 69;
pub const SPRUCE_LEAVES: i32 = 70;
pub const BIRCH_LEAVES: i32 = 71;
pub const JUNGLE_LEAVES: i32 = 72;
pub const ACACIA_LEAVES: i32 = 73;
pub const DARK_OAK_LEAVES: i32 = 74;
pub const SPONGE: i32 = 75;
pub const WET_SPONGE: i32 = 76;
pub const GLASS: i32 = 77;
pub const LAPIS_ORE: i32 = 78;
pub const LAPIS_BLOCK: i32 = 79;
pub const DISPENSER: i32 = 80;
pub const SANDSTONE: i32 = 81;
pub const CHISELED_SANDSTONE: i32 = 82;
pub const CUT_SANDSTONE: i32 = 83;
pub const NOTE_BLOCK: i32 = 84;
pub const POWERED_RAIL: i32 = 85;
pub const DETECTOR_RAIL: i32 = 86;
pub const STICKY_PISTON: i32 = 87;
pub const COBWEB: i32 = 88;
pub const GRASS: i32 = 89;
pub const FERN: i32 = 90;
pub const DEAD_BUSH: i32 = 91;
pub const SEAGRASS: i32 = 92;
pub const SEA_PICKLE: i32 = 93;
pub const PISTON: i32 = 94;
pub const WHITE_WOOL: i32 = 95;
pub const ORANGE_WOOL: i32 = 96;
pub const MAGENTA_WOOL: i32 = 97;
pub const LIGHT_BLUE_WOOL: i32 = 98;
pub const YELLOW_WOOL: i32 = 99;
pub const LIME_WOOL: i32 = 100;
pub const PINK_WOOL: i32 = 101;
pub const GRAY_WOOL: i32 = 102;
pub const LIGHT_GRAY_WOOL: i32 = 103;
pub const CYAN_WOOL: i32 = 104;
pub const PURPLE_WOOL: i32 = 105;
pub const BLUE_WOOL: i32 = 106;
pub const BROWN_WOOL: i32 = 107;
pub const GREEN_WOOL: i32 = 108;
pub const RED_WOOL: i32 = 109;
pub const BLACK_WOOL: i32 = 110;
pub const DANDELION: i32 = 111;
pub const POPPY: i32 = 112;
pub const BLUE_ORCHID: i32 = 113;
pub const ALLIUM: i32 = 114;
pub const AZURE_BLUET: i32 = 115;
pub const RED_TULIP: i32 = 116;
pub const ORANGE_TULIP: i32 = 117;
pub const WHITE_TULIP: i32 = 118;
pub const PINK_TULIP: i32 = 119;
pub const OXEYE_DAISY: i32 = 120;
pub const CORNFLOWER: i32 = 121;
pub const LILY_OF_THE_VALLEY: i32 = 122;
pub const WITHER_ROSE: i32 = 123;
pub const BROWN_MUSHROOM: i32 = 124;
pub const RED_MUSHROOM: i32 = 125;
pub const CRIMSON_FUNGUS: i32 = 126;
pub const WARPED_FUNGUS: i32 = 127;
pub const CRIMSON_ROOTS: i32 = 128;
pub const WARPED_ROOTS: i32 = 129;
pub const NETHER_SPROUTS: i32 = 130;
pub const WEEPING_VINES: i32 = 131;
pub const TWISTING_VINES: i32 = 132;
pub const GOLD_BLOCK: i32 = 133;
pub const IRON_BLOCK: i32 = 134;
pub const OAK_SLAB: i32 = 135;
pub const SPRUCE_SLAB: i32 = 136;
pub const BIRCH_SLAB: i32 = 137;
pub const JUNGLE_SLAB: i32 = 138;
pub const ACACIA_SLAB: i32 = 139;
pub const DARK_OAK_SLAB: i32 = 140;
pub const CRIMSON_SLAB: i32 = 141;
pub const WARPED_SLAB: i32 = 142;
pub const STONE_SLAB: i32 = 143;
pub const SMOOTH_STONE_SLAB: i32 = 144;
pub const SANDSTONE_SLAB: i32 = 145;
pub const CUT_SANDSTONE_SLAB: i32 = 146;
pub const PETRIFIED_OAK_SLAB: i32 = 147;
pub const COBBLESTONE_SLAB: i32 = 148;
pub const BRICK_SLAB: i32 = 149;
pub const STONE_BRICK_SLAB: i32 = 150;
pub const NETHER_BRICK_SLAB: i32 = 151;
pub const QUARTZ_SLAB: i32 = 152;
pub const RED_SANDSTONE_SLAB: i32 = 153;
pub const CUT_RED_SANDSTONE_SLAB: i32 = 154;
pub const PURPUR_SLAB: i32 = 155;
pub const PRISMARINE_SLAB: i32 = 156;
pub const PRISMARINE_BRICK_SLAB: i32 = 157;
pub const DARK_PRISMARINE_SLAB: i32 = 158;
pub const SMOOTH_QUARTZ: i32 = 159;
pub const SMOOTH_RED_SANDSTONE: i32 = 160;
pub const SMOOTH_SANDSTONE: i32 = 161;
pub const SMOOTH_STONE: i32 = 162;
pub const BRICKS: i32 = 163;
pub const TNT: i32 = 164;
pub const BOOKSHELF: i32 = 165;
pub const MOSSY_COBBLESTONE: i32 = 166;
pub const OBSIDIAN: i32 = 167;
pub const TORCH: i32 = 168;
pub const END_ROD: i32 = 169;
pub const CHORUS_PLANT: i32 = 170;
pub const CHORUS_FLOWER: i32 = 171;
pub const PURPUR_BLOCK: i32 = 172;
pub const PURPUR_PILLAR: i32 = 173;
pub const PURPUR_STAIRS: i32 = 174;
pub const SPAWNER: i32 = 175;
pub const OAK_STAIRS: i32 = 176;
pub const CHEST: i32 = 177;
pub const DIAMOND_ORE: i32 = 178;
pub const DIAMOND_BLOCK: i32 = 179;
pub const CRAFTING_TABLE: i32 = 180;
pub const FARMLAND: i32 = 181;
pub const FURNACE: i32 = 182;
pub const LADDER: i32 = 183;
pub const RAIL: i32 = 184;
pub const COBBLESTONE_STAIRS: i32 = 185;
pub const LEVER: i32 = 186;
pub const STONE_PRESSURE_PLATE: i32 = 187;
pub const OAK_PRESSURE_PLATE: i32 = 188;
pub const SPRUCE_PRESSURE_PLATE: i32 = 189;
pub const BIRCH_PRESSURE_PLATE: i32 = 190;
pub const JUNGLE_PRESSURE_PLATE: i32 = 191;
pub const ACACIA_PRESSURE_PLATE: i32 = 192;
pub const DARK_OAK_PRESSURE_PLATE: i32 = 193;
pub const CRIMSON_PRESSURE_PLATE: i32 = 194;
pub const WARPED_PRESSURE_PLATE: i32 = 195;
pub const REDSTONE_ORE: i32 = 196;
pub const REDSTONE_TORCH: i32 = 197;
pub const STONE_BUTTON: i32 = 198;
pub const SNOW: i32 = 199;
pub const ICE: i32 = 200;
pub const SNOW_BLOCK: i32 = 201;
pub const CACTUS: i32 = 202;
pub const CLAY: i32 = 203;
pub const JUKEBOX: i32 = 204;
pub const OAK_FENCE: i32 = 205;
pub const SPRUCE_FENCE: i32 = 206;
pub const BIRCH_FENCE: i32 = 207;
pub const JUNGLE_FENCE: i32 = 208;
pub const ACACIA_FENCE: i32 = 209;
pub const DARK_OAK_FENCE: i32 = 210;
pub const CRIMSON_FENCE: i32 = 211;
pub const WARPED_FENCE: i32 = 212;
pub const PUMPKIN: i32 = 213;
pub const CARVED_PUMPKIN: i32 = 214;
pub const NETHERRACK: i32 = 215;
pub const SOUL_SAND: i32 = 216;
pub const SOUL_SOIL: i32 = 217;
pub const BASALT: i32 = 218;
pub const POLISHED_BASALT: i32 = 219;
pub const SOUL_FIRE_TORCH: i32 = 220;
pub const GLOWSTONE: i32 = 221;
pub const JACK_O_LANTERN: i32 = 222;
pub const OAK_TRAPDOOR: i32 = 223;
pub const SPRUCE_TRAPDOOR: i32 = 224;
pub const BIRCH_TRAPDOOR: i32 = 225;
pub const JUNGLE_TRAPDOOR: i32 = 226;
pub const ACACIA_TRAPDOOR: i32 = 227;
pub const DARK_OAK_TRAPDOOR: i32 = 228;
pub const CRIMSON_TRAPDOOR: i32 = 229;
pub const WARPED_TRAPDOOR: i32 = 230;
pub const INFESTED_STONE: i32 = 231;
pub const INFESTED_COBBLESTONE: i32 = 232;
pub const INFESTED_STONE_BRICKS: i32 = 233;
pub const INFESTED_MOSSY_STONE_BRICKS: i32 = 234;
pub const INFESTED_CRACKED_STONE_BRICKS: i32 = 235;
pub const INFESTED_CHISELED_STONE_BRICKS: i32 = 236;
pub const STONE_BRICKS: i32 = 237;
pub const MOSSY_STONE_BRICKS: i32 = 238;
pub const CRACKED_STONE_BRICKS: i32 = 239;
pub const CHISELED_STONE_BRICKS: i32 = 240;
pub const BROWN_MUSHROOM_BLOCK: i32 = 241;
pub const RED_MUSHROOM_BLOCK: i32 = 242;
pub const MUSHROOM_STEM: i32 = 243;
pub const IRON_BARS: i32 = 244;
pub const GLASS_PANE: i32 = 245;
pub const MELON: i32 = 246;
pub const VINE: i32 = 247;
pub const OAK_FENCE_GATE: i32 = 248;
pub const SPRUCE_FENCE_GATE: i32 = 249;
pub const BIRCH_FENCE_GATE: i32 = 250;
pub const JUNGLE_FENCE_GATE: i32 = 251;
pub const ACACIA_FENCE_GATE: i32 = 252;
pub const DARK_OAK_FENCE_GATE: i32 = 253;
pub const CRIMSON_FENCE_GATE: i32 = 254;
pub const WARPED_FENCE_GATE: i32 = 255;
pub const BRICK_STAIRS: i32 = 256;
pub const STONE_BRICK_STAIRS: i32 = 257;
pub const MYCELIUM: i32 = 258;
pub const LILY_PAD: i32 = 259;
pub const NETHER_BRICKS: i32 = 260;
pub const NETHER_BRICK_FENCE: i32 = 261;
pub const NETHER_BRICK_STAIRS: i32 = 262;
pub const ENCHANTING_TABLE: i32 = 263;
pub const END_PORTAL_FRAME: i32 = 264;
pub const END_STONE: i32 = 265;
pub const END_STONE_BRICKS: i32 = 266;
pub const DRAGON_EGG: i32 = 267;
pub const REDSTONE_LAMP: i32 = 268;
pub const SANDSTONE_STAIRS: i32 = 269;
pub const EMERALD_ORE: i32 = 270;
pub const ENDER_CHEST: i32 = 271;
pub const TRIPWIRE_HOOK: i32 = 272;
pub const EMERALD_BLOCK: i32 = 273;
pub const SPRUCE_STAIRS: i32 = 274;
pub const BIRCH_STAIRS: i32 = 275;
pub const JUNGLE_STAIRS: i32 = 276;
pub const CRIMSON_STAIRS: i32 = 277;
pub const WARPED_STAIRS: i32 = 278;
pub const COMMAND_BLOCK: i32 = 279;
pub const BEACON: i32 = 280;
pub const COBBLESTONE_WALL: i32 = 281;
pub const MOSSY_COBBLESTONE_WALL: i32 = 282;
pub const BRICK_WALL: i32 = 283;
pub const PRISMARINE_WALL: i32 = 284;
pub const RED_SANDSTONE_WALL: i32 = 285;
pub const MOSSY_STONE_BRICK_WALL: i32 = 286;
pub const GRANITE_WALL: i32 = 287;
pub const STONE_BRICK_WALL: i32 = 288;
pub const NETHER_BRICK_WALL: i32 = 289;
pub const ANDESITE_WALL: i32 = 290;
pub const RED_NETHER_BRICK_WALL: i32 = 291;
pub const SANDSTONE_WALL: i32 = 292;
pub const END_STONE_BRICK_WALL: i32 = 293;
pub const DIORITE_WALL: i32 = 294;
pub const OAK_BUTTON: i32 = 295;
pub const SPRUCE_BUTTON: i32 = 296;
pub const BIRCH_BUTTON: i32 = 297;
pub const JUNGLE_BUTTON: i32 = 298;
pub const ACACIA_BUTTON: i32 = 299;
pub const DARK_OAK_BUTTON: i32 = 300;
pub const CRIMSON_BUTTON: i32 = 301;
pub const WARPED_BUTTON: i32 = 302;
pub const ANVIL: i32 = 303;
pub const CHIPPED_ANVIL: i32 = 304;
pub const DAMAGED_ANVIL: i32 = 305;
pub const TRAPPED_CHEST: i32 = 306;
pub const LIGHT_WEIGHTED_PRESSURE_PLATE: i32 = 307;
pub const HEAVY_WEIGHTED_PRESSURE_PLATE: i32 = 308;
pub const DAYLIGHT_DETECTOR: i32 = 309;
pub const REDSTONE_BLOCK: i32 = 310;
pub const NETHER_QUARTZ_ORE: i32 = 311;
pub const HOPPER: i32 = 312;
pub const CHISELED_QUARTZ_BLOCK: i32 = 313;
pub const QUARTZ_BLOCK: i32 = 314;
pub const QUARTZ_PILLAR: i32 = 315;
pub const QUARTZ_STAIRS: i32 = 316;
pub const ACTIVATOR_RAIL: i32 = 317;
pub const DROPPER: i32 = 318;
pub const WHITE_TERRACOTTA: i32 = 319;
pub const ORANGE_TERRACOTTA: i32 = 320;
pub const MAGENTA_TERRACOTTA: i32 = 321;
pub const LIGHT_BLUE_TERRACOTTA: i32 = 322;
pub const YELLOW_TERRACOTTA: i32 = 323;
pub const LIME_TERRACOTTA: i32 = 324;
pub const PINK_TERRACOTTA: i32 = 325;
pub const GRAY_TERRACOTTA: i32 = 326;
pub const LIGHT_GRAY_TERRACOTTA: i32 = 327;
pub const CYAN_TERRACOTTA: i32 = 328;
pub const PURPLE_TERRACOTTA: i32 = 329;
pub const BLUE_TERRACOTTA: i32 = 330;
pub const BROWN_TERRACOTTA: i32 = 331;
pub const GREEN_TERRACOTTA: i32 = 332;
pub const RED_TERRACOTTA: i32 = 333;
pub const BLACK_TERRACOTTA: i32 = 334;
pub const BARRIER: i32 = 335;
pub const IRON_TRAPDOOR: i32 = 336;
pub const HAY_BLOCK: i32 = 337;
pub const WHITE_CARPET: i32 = 338;
pub const ORANGE_CARPET: i32 = 339;
pub const MAGENTA_CARPET: i32 = 340;
pub const LIGHT_BLUE_CARPET: i32 = 341;
pub const YELLOW_CARPET: i32 = 342;
pub const LIME_CARPET: i32 = 343;
pub const PINK_CARPET: i32 = 344;
pub const GRAY_CARPET: i32 = 345;
pub const LIGHT_GRAY_CARPET: i32 = 346;
pub const CYAN_CARPET: i32 = 347;
pub const PURPLE_CARPET: i32 = 348;
pub const BLUE_CARPET: i32 = 349;
pub const BROWN_CARPET: i32 = 350;
pub const GREEN_CARPET: i32 = 351;
pub const RED_CARPET: i32 = 352;
pub const BLACK_CARPET: i32 = 353;
pub const TERRACOTTA: i32 = 354;
pub const COAL_BLOCK: i32 = 355;
pub const PACKED_ICE: i32 = 356;
pub const ACACIA_STAIRS: i32 = 357;
pub const DARK_OAK_STAIRS: i32 = 358;
pub const SLIME_BLOCK: i32 = 359;
pub const GRASS_PATH: i32 = 360;
pub const SUNFLOWER: i32 = 361;
pub const LILAC: i32 = 362;
pub const ROSE_BUSH: i32 = 363;
pub const PEONY: i32 = 364;
pub const TALL_GRASS: i32 = 365;
pub const LARGE_FERN: i32 = 366;
pub const WHITE_STAINED_GLASS: i32 = 367;
pub const ORANGE_STAINED_GLASS: i32 = 368;
pub const MAGENTA_STAINED_GLASS: i32 = 369;
pub const LIGHT_BLUE_STAINED_GLASS: i32 = 370;
pub const YELLOW_STAINED_GLASS: i32 = 371;
pub const LIME_STAINED_GLASS: i32 = 372;
pub const PINK_STAINED_GLASS: i32 = 373;
pub const GRAY_STAINED_GLASS: i32 = 374;
pub const LIGHT_GRAY_STAINED_GLASS: i32 = 375;
pub const CYAN_STAINED_GLASS: i32 = 376;
pub const PURPLE_STAINED_GLASS: i32 = 377;
pub const BLUE_STAINED_GLASS: i32 = 378;
pub const BROWN_STAINED_GLASS: i32 = 379;
pub const GREEN_STAINED_GLASS: i32 = 380;
pub const RED_STAINED_GLASS: i32 = 381;
pub const BLACK_STAINED_GLASS: i32 = 382;
pub const WHITE_STAINED_GLASS_PANE: i32 = 383;
pub const ORANGE_STAINED_GLASS_PANE: i32 = 384;
pub const MAGENTA_STAINED_GLASS_PANE: i32 = 385;
pub const LIGHT_BLUE_STAINED_GLASS_PANE: i32 = 386;
pub const YELLOW_STAINED_GLASS_PANE: i32 = 387;
pub const LIME_STAINED_GLASS_PANE: i32 = 388;
pub const PINK_STAINED_GLASS_PANE: i32 = 389;
pub const GRAY_STAINED_GLASS_PANE: i32 = 390;
pub const LIGHT_GRAY_STAINED_GLASS_PANE: i32 = 391;
pub const CYAN_STAINED_GLASS_PANE: i32 = 392;
pub const PURPLE_STAINED_GLASS_PANE: i32 = 393;
pub const BLUE_STAINED_GLASS_PANE: i32 = 394;
pub const BROWN_STAINED_GLASS_PANE: i32 = 395;
pub const GREEN_STAINED_GLASS_PANE: i32 = 396;
pub const RED_STAINED_GLASS_PANE: i32 = 397;
pub const BLACK_STAINED_GLASS_PANE: i32 = 398;
pub const PRISMARINE: i32 = 399;
pub const PRISMARINE_BRICKS: i32 = 400;
pub const DARK_PRISMARINE: i32 = 401;
pub const PRISMARINE_STAIRS: i32 = 402;
pub const PRISMARINE_BRICK_STAIRS: i32 = 403;
pub const DARK_PRISMARINE_STAIRS: i32 = 404;
pub const SEA_LANTERN: i32 = 405;
pub const RED_SANDSTONE: i32 = 406;
pub const CHISELED_RED_SANDSTONE: i32 = 407;
pub const CUT_RED_SANDSTONE: i32 = 408;
pub const RED_SANDSTONE_STAIRS: i32 = 409;
pub const REPEATING_COMMAND_BLOCK: i32 = 410;
pub const CHAIN_COMMAND_BLOCK: i32 = 411;
pub const MAGMA_BLOCK: i32 = 412;
pub const NETHER_WART_BLOCK: i32 = 413;
pub const WARPED_WART_BLOCK: i32 = 414;
pub const RED_NETHER_BRICKS: i32 = 415;
pub const BONE_BLOCK: i32 = 416;
pub const STRUCTURE_VOID: i32 = 417;
pub const OBSERVER: i32 = 418;
pub const SHULKER_BOX: i32 = 419;
pub const WHITE_SHULKER_BOX: i32 = 420;
pub const ORANGE_SHULKER_BOX: i32 = 421;
pub const MAGENTA_SHULKER_BOX: i32 = 422;
pub const LIGHT_BLUE_SHULKER_BOX: i32 = 423;
pub const YELLOW_SHULKER_BOX: i32 = 424;
pub const LIME_SHULKER_BOX: i32 = 425;
pub const PINK_SHULKER_BOX: i32 = 426;
pub const GRAY_SHULKER_BOX: i32 = 427;
pub const LIGHT_GRAY_SHULKER_BOX: i32 = 428;
pub const CYAN_SHULKER_BOX: i32 = 429;
pub const PURPLE_SHULKER_BOX: i32 = 430;
pub const BLUE_SHULKER_BOX: i32 = 431;
pub const BROWN_SHULKER_BOX: i32 = 432;
pub const GREEN_SHULKER_BOX: i32 = 433;
pub const RED_SHULKER_BOX: i32 = 434;
pub const BLACK_SHULKER_BOX: i32 = 435;
pub const WHITE_GLAZED_TERRACOTTA: i32 = 436;
pub const ORANGE_GLAZED_TERRACOTTA: i32 = 437;
pub const MAGENTA_GLAZED_TERRACOTTA: i32 = 438;
pub const LIGHT_BLUE_GLAZED_TERRACOTTA: i32 = 439;
pub const YELLOW_GLAZED_TERRACOTTA: i32 = 440;
pub const LIME_GLAZED_TERRACOTTA: i32 = 441;
pub const PINK_GLAZED_TERRACOTTA: i32 = 442;
pub const GRAY_GLAZED_TERRACOTTA: i32 = 443;
pub const LIGHT_GRAY_GLAZED_TERRACOTTA: i32 = 444;
pub const CYAN_GLAZED_TERRACOTTA: i32 = 445;
pub const PURPLE_GLAZED_TERRACOTTA: i32 = 446;
pub const BLUE_GLAZED_TERRACOTTA: i32 = 447;
pub const BROWN_GLAZED_TERRACOTTA: i32 = 448;
pub const GREEN_GLAZED_TERRACOTTA: i32 = 449;
pub const RED_GLAZED_TERRACOTTA: i32 = 450;
pub const BLACK_GLAZED_TERRACOTTA: i32 = 451;
pub const WHITE_CONCRETE: i32 = 452;
pub const ORANGE_CONCRETE: i32 = 453;
pub const MAGENTA_CONCRETE: i32 = 454;
pub const LIGHT_BLUE_CONCRETE: i32 = 455;
pub const YELLOW_CONCRETE: i32 = 456;
pub const LIME_CONCRETE: i32 = 457;
pub const PINK_CONCRETE: i32 = 458;
pub const GRAY_CONCRETE: i32 = 459;
pub const LIGHT_GRAY_CONCRETE: i32 = 460;
pub const CYAN_CONCRETE: i32 = 461;
pub const PURPLE_CONCRETE: i32 = 462;
pub const BLUE_CONCRETE: i32 = 463;
pub const BROWN_CONCRETE: i32 = 464;
pub const GREEN_CONCRETE: i32 = 465;
pub const RED_CONCRETE: i32 = 466;
pub const BLACK_CONCRETE: i32 = 467;
pub const WHITE_CONCRETE_POWDER: i32 = 468;
pub const ORANGE_CONCRETE_POWDER: i32 = 469;
pub const MAGENTA_CONCRETE_POWDER: i32 = 470;
pub const LIGHT_BLUE_CONCRETE_POWDER: i32 = 471;
pub const YELLOW_CONCRETE_POWDER: i32 = 472;
pub const LIME_CONCRETE_POWDER: i32 = 473;
pub const PINK_CONCRETE_POWDER: i32 = 474;
pub const GRAY_CONCRETE_POWDER: i32 = 475;
pub const LIGHT_GRAY_CONCRETE_POWDER: i32 = 476;
pub const CYAN_CONCRETE_POWDER: i32 = 477;
pub const PURPLE_CONCRETE_POWDER: i32 = 478;
pub const BLUE_CONCRETE_POWDER: i32 = 479;
pub const BROWN_CONCRETE_POWDER: i32 = 480;
pub const GREEN_CONCRETE_POWDER: i32 = 481;
pub const RED_CONCRETE_POWDER: i32 = 482;
pub const BLACK_CONCRETE_POWDER: i32 = 483;
pub const TURTLE_EGG: i32 = 484;
pub const DEAD_TUBE_CORAL_BLOCK: i32 = 485;
pub const DEAD_BRAIN_CORAL_BLOCK: i32 = 486;
pub const DEAD_BUBBLE_CORAL_BLOCK: i32 = 487;
pub const DEAD_FIRE_CORAL_BLOCK: i32 = 488;
pub const DEAD_HORN_CORAL_BLOCK: i32 = 489;
pub const TUBE_CORAL_BLOCK: i32 = 490;
pub const BRAIN_CORAL_BLOCK: i32 = 491;
pub const BUBBLE_CORAL_BLOCK: i32 = 492;
pub const FIRE_CORAL_BLOCK: i32 = 493;
pub const HORN_CORAL_BLOCK: i32 = 494;
pub const TUBE_CORAL: i32 = 495;
pub const BRAIN_CORAL: i32 = 496;
pub const BUBBLE_CORAL: i32 = 497;
pub const FIRE_CORAL: i32 = 498;
pub const HORN_CORAL: i32 = 499;
pub const DEAD_BRAIN_CORAL: i32 = 500;
pub const DEAD_BUBBLE_CORAL: i32 = 501;
pub const DEAD_FIRE_CORAL: i32 = 502;
pub const DEAD_HORN_CORAL: i32 = 503;
pub const DEAD_TUBE_CORAL: i32 = 504;
pub const TUBE_CORAL_FAN: i32 = 505;
pub const BRAIN_CORAL_FAN: i32 = 506;
pub const BUBBLE_CORAL_FAN: i32 = 507;
pub const FIRE_CORAL_FAN: i32 = 508;
pub const HORN_CORAL_FAN: i32 = 509;
pub const DEAD_TUBE_CORAL_FAN: i32 = 510;
pub const DEAD_BRAIN_CORAL_FAN: i32 = 511;
pub const DEAD_BUBBLE_CORAL_FAN: i32 = 512;
pub const DEAD_FIRE_CORAL_FAN: i32 = 513;
pub const DEAD_HORN_CORAL_FAN: i32 = 514;
pub const BLUE_ICE: i32 = 515;
pub const CONDUIT: i32 = 516;
pub const POLISHED_GRANITE_STAIRS: i32 = 517;
pub const SMOOTH_RED_SANDSTONE_STAIRS: i32 = 518;
pub const MOSSY_STONE_BRICK_STAIRS: i32 = 519;
pub const POLISHED_DIORITE_STAIRS: i32 = 520;
pub const MOSSY_COBBLESTONE_STAIRS: i32 = 521;
pub const END_STONE_BRICK_STAIRS: i32 = 522;
pub const STONE_STAIRS: i32 = 523;
pub const SMOOTH_SANDSTONE_STAIRS: i32 = 524;
pub const SMOOTH_QUARTZ_STAIRS: i32 = 525;
pub const GRANITE_STAIRS: i32 = 526;
pub const ANDESITE_STAIRS: i32 = 527;
pub const RED_NETHER_BRICK_STAIRS: i32 = 528;
pub const POLISHED_ANDESITE_STAIRS: i32 = 529;
pub const DIORITE_STAIRS: i32 = 530;
pub const POLISHED_GRANITE_SLAB: i32 = 531;
pub const SMOOTH_RED_SANDSTONE_SLAB: i32 = 532;
pub const MOSSY_STONE_BRICK_SLAB: i32 = 533;
pub const POLISHED_DIORITE_SLAB: i32 = 534;
pub const MOSSY_COBBLESTONE_SLAB: i32 = 535;
pub const END_STONE_BRICK_SLAB: i32 = 536;
pub const SMOOTH_SANDSTONE_SLAB: i32 = 537;
pub const SMOOTH_QUARTZ_SLAB: i32 = 538;
pub const GRANITE_SLAB: i32 = 539;
pub const ANDESITE_SLAB: i32 = 540;
pub const RED_NETHER_BRICK_SLAB: i32 = 541;
pub const POLISHED_ANDESITE_SLAB: i32 = 542;
pub const DIORITE_SLAB: i32 = 543;
pub const SCAFFOLDING: i32 = 544;
pub const IRON_DOOR: i32 = 545;
pub const OAK_DOOR: i32 = 546;
pub const SPRUCE_DOOR: i32 = 547;
pub const BIRCH_DOOR: i32 = 548;
pub const JUNGLE_DOOR: i32 = 549;
pub const ACACIA_DOOR: i32 = 550;
pub const DARK_OAK_DOOR: i32 = 551;
pub const CRIMSON_DOOR: i32 = 552;
pub const WARPED_DOOR: i32 = 553;
pub const REPEATER: i32 = 554;
pub const COMPARATOR: i32 = 555;
pub const STRUCTURE_BLOCK: i32 = 556;
pub const JIGSAW: i32 = 557;
pub const TURTLE_HELMET: i32 = 558;
pub const SCUTE: i32 = 559;
pub const IRON_SHOVEL: i32 = 560;
pub const IRON_PICKAXE: i32 = 561;
pub const IRON_AXE: i32 = 562;
pub const FLINT_AND_STEEL: i32 = 563;
pub const APPLE: i32 = 564;
pub const BOW: i32 = 565;
pub const ARROW: i32 = 566;
pub const COAL: i32 = 567;
pub const CHARCOAL: i32 = 568;
pub const DIAMOND: i32 = 569;
pub const IRON_INGOT: i32 = 570;
pub const GOLD_INGOT: i32 = 571;
pub const IRON_SWORD: i32 = 572;
pub const WOODEN_SWORD: i32 = 573;
pub const WOODEN_SHOVEL: i32 = 574;
pub const WOODEN_PICKAXE: i32 = 575;
pub const WOODEN_AXE: i32 = 576;
pub const STONE_SWORD: i32 = 577;
pub const STONE_SHOVEL: i32 = 578;
pub const STONE_PICKAXE: i32 = 579;
pub const STONE_AXE: i32 = 580;
pub const DIAMOND_SWORD: i32 = 581;
pub const DIAMOND_SHOVEL: i32 = 582;
pub const DIAMOND_PICKAXE: i32 = 583;
pub const DIAMOND_AXE: i32 = 584;
pub const STICK: i32 = 585;
pub const BOWL: i32 = 586;
pub const MUSHROOM_STEW: i32 = 587;
pub const GOLDEN_SWORD: i32 = 588;
pub const GOLDEN_SHOVEL: i32 = 589;
pub const GOLDEN_PICKAXE: i32 = 590;
pub const GOLDEN_AXE: i32 = 591;
pub const NETHERITE_SWORD: i32 = 592;
pub const NETHERITE_SHOVEL: i32 = 593;
pub const NETHERITE_PICKAXE: i32 = 594;
pub const NETHERITE_AXE: i32 = 595;
pub const STRING: i32 = 596;
pub const FEATHER: i32 = 597;
pub const GUNPOWDER: i32 = 598;
pub const WOODEN_HOE: i32 = 599;
pub const STONE_HOE: i32 = 600;
pub const IRON_HOE: i32 = 601;
pub const DIAMOND_HOE: i32 = 602;
pub const GOLDEN_HOE: i32 = 603;
pub const NETHERITE_HOE: i32 = 604;
pub const WHEAT_SEEDS: i32 = 605;
pub const WHEAT: i32 = 606;
pub const BREAD: i32 = 607;
pub const LEATHER_HELMET: i32 = 608;
pub const LEATHER_CHESTPLATE: i32 = 609;
pub const LEATHER_LEGGINGS: i32 = 610;
pub const LEATHER_BOOTS: i32 = 611;
pub const CHAINMAIL_HELMET: i32 = 612;
pub const CHAINMAIL_CHESTPLATE: i32 = 613;
pub const CHAINMAIL_LEGGINGS: i32 = 614;
pub const CHAINMAIL_BOOTS: i32 = 615;
pub const IRON_HELMET: i32 = 616;
pub const IRON_CHESTPLATE: i32 = 617;
pub const IRON_LEGGINGS: i32 = 618;
pub const IRON_BOOTS: i32 = 619;
pub const DIAMOND_HELMET: i32 = 620;
pub const DIAMOND_CHESTPLATE: i32 = 621;
pub const DIAMOND_LEGGINGS: i32 = 622;
pub const DIAMOND_BOOTS: i32 = 623;
pub const GOLDEN_HELMET: i32 = 624;
pub const GOLDEN_CHESTPLATE: i32 = 625;
pub const GOLDEN_LEGGINGS: i32 = 626;
pub const GOLDEN_BOOTS: i32 = 627;
pub const NETHERITE_HELMET: i32 = 628;
pub const NETHERITE_CHESTPLATE: i32 = 629;
pub const NETHERITE_LEGGINGS: i32 = 630;
pub const NETHERITE_BOOTS: i32 = 631;
pub const FLINT: i32 = 632;
pub const PORKCHOP: i32 = 633;
pub const COOKED_PORKCHOP: i32 = 634;
pub const PAINTING: i32 = 635;
pub const GOLDEN_APPLE: i32 = 636;
pub const ENCHANTED_GOLDEN_APPLE: i32 = 637;
pub const OAK_SIGN: i32 = 638;
pub const SPRUCE_SIGN: i32 = 639;
pub const BIRCH_SIGN: i32 = 640;
pub const JUNGLE_SIGN: i32 = 641;
pub const ACACIA_SIGN: i32 = 642;
pub const DARK_OAK_SIGN: i32 = 643;
pub const CRIMSON_SIGN: i32 = 644;
pub const WARPED_SIGN: i32 = 645;
pub const BUCKET: i32 = 646;
pub const WATER_BUCKET: i32 = 647;
pub const LAVA_BUCKET: i32 = 648;
pub const MINECART: i32 = 649;
pub const SADDLE: i32 = 650;
pub const REDSTONE: i32 = 651;
pub const SNOWBALL: i32 = 652;
pub const OAK_BOAT: i32 = 653;
pub const LEATHER: i32 = 654;
pub const MILK_BUCKET: i32 = 655;
pub const PUFFERFISH_BUCKET: i32 = 656;
pub const SALMON_BUCKET: i32 = 657;
pub const COD_BUCKET: i32 = 658;
pub const TROPICAL_FISH_BUCKET: i32 = 659;
pub const BRICK: i32 = 660;
pub const CLAY_BALL: i32 = 661;
pub const SUGAR_CANE: i32 = 662;
pub const KELP: i32 = 663;
pub const DRIED_KELP_BLOCK: i32 = 664;
pub const BAMBOO: i32 = 665;
pub const PAPER: i32 = 666;
pub const BOOK: i32 = 667;
pub const SLIME_BALL: i32 = 668;
pub const CHEST_MINECART: i32 = 669;
pub const FURNACE_MINECART: i32 = 670;
pub const EGG: i32 = 671;
pub const COMPASS: i32 = 672;
pub const FISHING_ROD: i32 = 673;
pub const CLOCK: i32 = 674;
pub const GLOWSTONE_DUST: i32 = 675;
pub const COD: i32 = 676;
pub const SALMON: i32 = 677;
pub const TROPICAL_FISH: i32 = 678;
pub const PUFFERFISH: i32 = 679;
pub const COOKED_COD: i32 = 680;
pub const COOKED_SALMON: i32 = 681;
pub const INK_SAC: i32 = 682;
pub const RED_DYE: i32 = 683;
pub const GREEN_DYE: i32 = 684;
pub const COCOA_BEANS: i32 = 685;
pub const LAPIS_LAZULI: i32 = 686;
pub const PURPLE_DYE: i32 = 687;
pub const CYAN_DYE: i32 = 688;
pub const LIGHT_GRAY_DYE: i32 = 689;
pub const GRAY_DYE: i32 = 690;
pub const PINK_DYE: i32 = 691;
pub const LIME_DYE: i32 = 692;
pub const YELLOW_DYE: i32 = 693;
pub const LIGHT_BLUE_DYE: i32 = 694;
pub const MAGENTA_DYE: i32 = 695;
pub const ORANGE_DYE: i32 = 696;
pub const BONE_MEAL: i32 = 697;
pub const BLUE_DYE: i32 = 698;
pub const BROWN_DYE: i32 = 699;
pub const BLACK_DYE: i32 = 700;
pub const WHITE_DYE: i32 = 701;
pub const BONE: i32 = 702;
pub const SUGAR: i32 = 703;
pub const CAKE: i32 = 704;
pub const WHITE_BED: i32 = 705;
pub const ORANGE_BED: i32 = 706;
pub const MAGENTA_BED: i32 = 707;
pub const LIGHT_BLUE_BED: i32 = 708;
pub const YELLOW_BED: i32 = 709;
pub const LIME_BED: i32 = 710;
pub const PINK_BED: i32 = 711;
pub const GRAY_BED: i32 = 712;
pub const LIGHT_GRAY_BED: i32 = 713;
pub const CYAN_BED: i32 = 714;
pub const PURPLE_BED: i32 = 715;
pub const BLUE_BED: i32 = 716;
pub const BROWN_BED: i32 = 717;
pub const GREEN_BED: i32 = 718;
pub const RED_BED: i32 = 719;
pub const BLACK_BED: i32 = 720;
pub const COOKIE: i32 = 721;
pub const FILLED_MAP: i32 = 722;
pub const SHEARS: i32 = 723;
pub const MELON_SLICE: i32 = 724;
pub const DRIED_KELP: i32 = 725;
pub const PUMPKIN_SEEDS: i32 = 726;
pub const MELON_SEEDS: i32 = 727;
pub const BEEF: i32 = 728;
pub const COOKED_BEEF: i32 = 729;
pub const CHICKEN: i32 = 730;
pub const COOKED_CHICKEN: i32 = 731;
pub const ROTTEN_FLESH: i32 = 732;
pub const ENDER_PEARL: i32 = 733;
pub const BLAZE_ROD: i32 = 734;
pub const GHAST_TEAR: i32 = 735;
pub const GOLD_NUGGET: i32 = 736;
pub const NETHER_WART: i32 = 737;
pub const POTION: i32 = 738;
pub const GLASS_BOTTLE: i32 = 739;
pub const SPIDER_EYE: i32 = 740;
pub const FERMENTED_SPIDER_EYE: i32 = 741;
pub const BLAZE_POWDER: i32 = 742;
pub const MAGMA_CREAM: i32 = 743;
pub const BREWING_STAND: i32 = 744;
pub const CAULDRON: i32 = 745;
pub const ENDER_EYE: i32 = 746;
pub const GLISTERING_MELON_SLICE: i32 = 747;
pub const BAT_SPAWN_EGG: i32 = 748;
pub const BEE_SPAWN_EGG: i32 = 749;
pub const BLAZE_SPAWN_EGG: i32 = 750;
pub const CAT_SPAWN_EGG: i32 = 751;
pub const CAVE_SPIDER_SPAWN_EGG: i32 = 752;
pub const CHICKEN_SPAWN_EGG: i32 = 753;
pub const COD_SPAWN_EGG: i32 = 754;
pub const COW_SPAWN_EGG: i32 = 755;
pub const CREEPER_SPAWN_EGG: i32 = 756;
pub const DOLPHIN_SPAWN_EGG: i32 = 757;
pub const DONKEY_SPAWN_EGG: i32 = 758;
pub const DROWNED_SPAWN_EGG: i32 = 759;
pub const ELDER_GUARDIAN_SPAWN_EGG: i32 = 760;
pub const ENDERMAN_SPAWN_EGG: i32 = 761;
pub const ENDERMITE_SPAWN_EGG: i32 = 762;
pub const EVOKER_SPAWN_EGG: i32 = 763;
pub const FOX_SPAWN_EGG: i32 = 764;
pub const GHAST_SPAWN_EGG: i32 = 765;
pub const GUARDIAN_SPAWN_EGG: i32 = 766;
pub const HOGLIN_SPAWN_EGG: i32 = 767;
pub const HORSE_SPAWN_EGG: i32 = 768;
pub const HUSK_SPAWN_EGG: i32 = 769;
pub const LLAMA_SPAWN_EGG: i32 = 770;
pub const MAGMA_CUBE_SPAWN_EGG: i32 = 771;
pub const MOOSHROOM_SPAWN_EGG: i32 = 772;
pub const MULE_SPAWN_EGG: i32 = 773;
pub const OCELOT_SPAWN_EGG: i32 = 774;
pub const PANDA_SPAWN_EGG: i32 = 775;
pub const PARROT_SPAWN_EGG: i32 = 776;
pub const PHANTOM_SPAWN_EGG: i32 = 777;
pub const PIG_SPAWN_EGG: i32 = 778;
pub const PIGLIN_SPAWN_EGG: i32 = 779;
pub const PILLAGER_SPAWN_EGG: i32 = 780;
pub const POLAR_BEAR_SPAWN_EGG: i32 = 781;
pub const PUFFERFISH_SPAWN_EGG: i32 = 782;
pub const RABBIT_SPAWN_EGG: i32 = 783;
pub const RAVAGER_SPAWN_EGG: i32 = 784;
pub const SALMON_SPAWN_EGG: i32 = 785;
pub const SHEEP_SPAWN_EGG: i32 = 786;
pub const SHULKER_SPAWN_EGG: i32 = 787;
pub const SILVERFISH_SPAWN_EGG: i32 = 788;
pub const SKELETON_SPAWN_EGG: i32 = 789;
pub const SKELETON_HORSE_SPAWN_EGG: i32 = 790;
pub const SLIME_SPAWN_EGG: i32 = 791;
pub const SPIDER_SPAWN_EGG: i32 = 792;
pub const SQUID_SPAWN_EGG: i32 = 793;
pub const STRAY_SPAWN_EGG: i32 = 794;
pub const TRADER_LLAMA_SPAWN_EGG: i32 = 795;
pub const TROPICAL_FISH_SPAWN_EGG: i32 = 796;
pub const TURTLE_SPAWN_EGG: i32 = 797;
pub const VEX_SPAWN_EGG: i32 = 798;
pub const VILLAGER_SPAWN_EGG: i32 = 799;
pub const VINDICATOR_SPAWN_EGG: i32 = 800;
pub const WANDERING_TRADER_SPAWN_EGG: i32 = 801;
pub const WITCH_SPAWN_EGG: i32 = 802;
pub const WITHER_SKELETON_SPAWN_EGG: i32 = 803;
pub const WOLF_SPAWN_EGG: i32 = 804;
pub const ZOMBIE_SPAWN_EGG: i32 = 805;
pub const ZOMBIE_HORSE_SPAWN_EGG: i32 = 806;
pub const ZOMBIFIED_PIGLIN_SPAWN_EGG: i32 = 807;
pub const ZOMBIE_VILLAGER_SPAWN_EGG: i32 = 808;
pub const EXPERIENCE_BOTTLE: i32 = 809;
pub const FIRE_CHARGE: i32 = 810;
pub const WRITABLE_BOOK: i32 = 811;
pub const WRITTEN_BOOK: i32 = 812;
pub const EMERALD: i32 = 813;
pub const ITEM_FRAME: i32 = 814;
pub const FLOWER_POT: i32 = 815;
pub const CARROT: i32 = 816;
pub const POTATO: i32 = 817;
pub const BAKED_POTATO: i32 = 818;
pub const POISONOUS_POTATO: i32 = 819;
pub const MAP: i32 = 820;
pub const GOLDEN_CARROT: i32 = 821;
pub const SKELETON_SKULL: i32 = 822;
pub const WITHER_SKELETON_SKULL: i32 = 823;
pub const PLAYER_HEAD: i32 = 824;
pub const ZOMBIE_HEAD: i32 = 825;
pub const CREEPER_HEAD: i32 = 826;
pub const DRAGON_HEAD: i32 = 827;
pub const CARROT_ON_A_STICK: i32 = 828;
pub const NETHER_STAR: i32 = 829;
pub const PUMPKIN_PIE: i32 = 830;
pub const FIREWORK_ROCKET: i32 = 831;
pub const FIREWORK_STAR: i32 = 832;
pub const ENCHANTED_BOOK: i32 = 833;
pub const NETHER_BRICK: i32 = 834;
pub const QUARTZ: i32 = 835;
pub const TNT_MINECART: i32 = 836;
pub const HOPPER_MINECART: i32 = 837;
pub const PRISMARINE_SHARD: i32 = 838;
pub const PRISMARINE_CRYSTALS: i32 = 839;
pub const RABBIT: i32 = 840;
pub const COOKED_RABBIT: i32 = 841;
pub const RABBIT_STEW: i32 = 842;
pub const RABBIT_FOOT: i32 = 843;
pub const RABBIT_HIDE: i32 = 844;
pub const ARMOR_STAND: i32 = 845;
pub const IRON_HORSE_ARMOR: i32 = 846;
pub const GOLDEN_HORSE_ARMOR: i32 = 847;
pub const DIAMOND_HORSE_ARMOR: i32 = 848;
pub const LEATHER_HORSE_ARMOR: i32 = 849;
pub const LEAD: i32 = 850;
pub const NAME_TAG: i32 = 851;
pub const COMMAND_BLOCK_MINECART: i32 = 852;
pub const MUTTON: i32 = 853;
pub const COOKED_MUTTON: i32 = 854;
pub const WHITE_BANNER: i32 = 855;
pub const ORANGE_BANNER: i32 = 856;
pub const MAGENTA_BANNER: i32 = 857;
pub const LIGHT_BLUE_BANNER: i32 = 858;
pub const YELLOW_BANNER: i32 = 859;
pub const LIME_BANNER: i32 = 860;
pub const PINK_BANNER: i32 = 861;
pub const GRAY_BANNER: i32 = 862;
pub const LIGHT_GRAY_BANNER: i32 = 863;
pub const CYAN_BANNER: i32 = 864;
pub const PURPLE_BANNER: i32 = 865;
pub const BLUE_BANNER: i32 = 866;
pub const BROWN_BANNER: i32 = 867;
pub const GREEN_BANNER: i32 = 868;
pub const RED_BANNER: i32 = 869;
pub const BLACK_BANNER: i32 = 870;
pub const END_CRYSTAL: i32 = 871;
pub const CHORUS_FRUIT: i32 = 872;
pub const POPPED_CHORUS_FRUIT: i32 = 873;
pub const BEETROOT: i32 = 874;
pub const BEETROOT_SEEDS: i32 = 875;
pub const BEETROOT_SOUP: i32 = 876;
pub const DRAGON_BREATH: i32 = 877;
pub const SPLASH_POTION: i32 = 878;
pub const SPECTRAL_ARROW: i32 = 879;
pub const TIPPED_ARROW: i32 = 880;
pub const LINGERING_POTION: i32 = 881;
pub const SHIELD: i32 = 882;
pub const ELYTRA: i32 = 883;
pub const SPRUCE_BOAT: i32 = 884;
pub const BIRCH_BOAT: i32 = 885;
pub const JUNGLE_BOAT: i32 = 886;
pub const ACACIA_BOAT: i32 = 887;
pub const DARK_OAK_BOAT: i32 = 888;
pub const TOTEM_OF_UNDYING: i32 = 889;
pub const SHULKER_SHELL: i32 = 890;
pub const IRON_NUGGET: i32 = 891;
pub const KNOWLEDGE_BOOK: i32 = 892;
pub const DEBUG_STICK: i32 = 893;
pub const MUSIC_DISC_13: i32 = 894;
pub const MUSIC_DISC_CAT: i32 = 895;
pub const MUSIC_DISC_BLOCKS: i32 = 896;
pub const MUSIC_DISC_CHIRP: i32 = 897;
pub const MUSIC_DISC_FAR: i32 = 898;
pub const MUSIC_DISC_MALL: i32 = 899;
pub const MUSIC_DISC_MELLOHI: i32 = 900;
pub const MUSIC_DISC_STAL: i32 = 901;
pub const MUSIC_DISC_STRAD: i32 = 902;
pub const MUSIC_DISC_WARD: i32 = 903;
pub const MUSIC_DISC_11: i32 = 904;
pub const MUSIC_DISC_WAIT: i32 = 905;
pub const TRIDENT: i32 = 906;
pub const PHANTOM_MEMBRANE: i32 = 907;
pub const NAUTILUS_SHELL: i32 = 908;
pub const HEART_OF_THE_SEA: i32 = 909;
pub const CROSSBOW: i32 = 910;
pub const SUSPICIOUS_STEW: i32 = 911;
pub const LOOM: i32 = 912;
pub const FLOWER_BANNER_PATTERN: i32 = 913;
pub const CREEPER_BANNER_PATTERN: i32 = 914;
pub const SKULL_BANNER_PATTERN: i32 = 915;
pub const MOJANG_BANNER_PATTERN: i32 = 916;
pub const GLOBE_BANNER_PATTERN: i32 = 917;
pub const COMPOSTER: i32 = 918;
pub const BARREL: i32 = 919;
pub const SMOKER: i32 = 920;
pub const BLAST_FURNACE: i32 = 921;
pub const CARTOGRAPHY_TABLE: i32 = 922;
pub const FLETCHING_TABLE: i32 = 923;
pub const GRINDSTONE: i32 = 924;
pub const LECTERN: i32 = 925;
pub const SMITHING_TABLE: i32 = 926;
pub const STONECUTTER: i32 = 927;
pub const BELL: i32 = 928;
pub const LANTERN: i32 = 929;
pub const SOUL_FIRE_LANTERN: i32 = 930;
pub const SWEET_BERRIES: i32 = 931;
pub const CAMPFIRE: i32 = 932;
pub const SHROOMLIGHT: i32 = 933;
pub const HONEYCOMB: i32 = 934;
pub const BEE_NEST: i32 = 935;
pub const BEEHIVE: i32 = 936;
pub const HONEY_BOTTLE: i32 = 937;
pub const HONEY_BLOCK: i32 = 938;
pub const HONEYCOMB_BLOCK: i32 = 939;
pub const NETHERITE_BLOCK: i32 = 940;
pub const ANCIENT_DEBRIS: i32 = 941;
pub const NETHERITE_INGOT: i32 = 942;
pub const NETHERITE_SCRAP: i32 = 943;
pub const TARGET: i32 = 944;
pub const CRYING_OBSIDIAN: i32 = 945;
pub const RESPAWN_ANCHOR: i32 = 946;