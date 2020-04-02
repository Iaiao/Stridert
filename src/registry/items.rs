use crate::inventory::item::Item;
use crate::registry::identifier::Identifier;

pub fn get_items() -> [Item; 947] {
	[
		Item::new("minecraft", "air", 0),
		Item::new("minecraft", "stone", 1),
		Item::new("minecraft", "granite", 2),
		Item::new("minecraft", "polished_granite", 3),
		Item::new("minecraft", "diorite", 4),
		Item::new("minecraft", "polished_diorite", 5),
		Item::new("minecraft", "andesite", 6),
		Item::new("minecraft", "polished_andesite", 7),
		Item::new("minecraft", "grass_block", 8),
		Item::new("minecraft", "dirt", 9),
		Item::new("minecraft", "coarse_dirt", 10),
		Item::new("minecraft", "podzol", 11),
		Item::new("minecraft", "crimson_nylium", 12),
		Item::new("minecraft", "warped_nylium", 13),
		Item::new("minecraft", "cobblestone", 14),
		Item::new("minecraft", "oak_planks", 15),
		Item::new("minecraft", "spruce_planks", 16),
		Item::new("minecraft", "birch_planks", 17),
		Item::new("minecraft", "jungle_planks", 18),
		Item::new("minecraft", "acacia_planks", 19),
		Item::new("minecraft", "dark_oak_planks", 20),
		Item::new("minecraft", "crimson_planks", 21),
		Item::new("minecraft", "warped_planks", 22),
		Item::new("minecraft", "oak_sapling", 23),
		Item::new("minecraft", "spruce_sapling", 24),
		Item::new("minecraft", "birch_sapling", 25),
		Item::new("minecraft", "jungle_sapling", 26),
		Item::new("minecraft", "acacia_sapling", 27),
		Item::new("minecraft", "dark_oak_sapling", 28),
		Item::new("minecraft", "bedrock", 29),
		Item::new("minecraft", "sand", 30),
		Item::new("minecraft", "red_sand", 31),
		Item::new("minecraft", "gravel", 32),
		Item::new("minecraft", "gold_ore", 33),
		Item::new("minecraft", "iron_ore", 34),
		Item::new("minecraft", "coal_ore", 35),
		Item::new("minecraft", "nether_gold_ore", 36),
		Item::new("minecraft", "oak_log", 37),
		Item::new("minecraft", "spruce_log", 38),
		Item::new("minecraft", "birch_log", 39),
		Item::new("minecraft", "jungle_log", 40),
		Item::new("minecraft", "acacia_log", 41),
		Item::new("minecraft", "dark_oak_log", 42),
		Item::new("minecraft", "crimson_stem", 43),
		Item::new("minecraft", "warped_stem", 44),
		Item::new("minecraft", "stripped_oak_log", 45),
		Item::new("minecraft", "stripped_spruce_log", 46),
		Item::new("minecraft", "stripped_birch_log", 47),
		Item::new("minecraft", "stripped_jungle_log", 48),
		Item::new("minecraft", "stripped_acacia_log", 49),
		Item::new("minecraft", "stripped_dark_oak_log", 50),
		Item::new("minecraft", "stripped_crimson_stem", 51),
		Item::new("minecraft", "stripped_warped_stem", 52),
		Item::new("minecraft", "stripped_oak_wood", 53),
		Item::new("minecraft", "stripped_spruce_wood", 54),
		Item::new("minecraft", "stripped_birch_wood", 55),
		Item::new("minecraft", "stripped_jungle_wood", 56),
		Item::new("minecraft", "stripped_acacia_wood", 57),
		Item::new("minecraft", "stripped_dark_oak_wood", 58),
		Item::new("minecraft", "stripped_crimson_hyphae", 59),
		Item::new("minecraft", "stripped_warped_hyphae", 60),
		Item::new("minecraft", "oak_wood", 61),
		Item::new("minecraft", "spruce_wood", 62),
		Item::new("minecraft", "birch_wood", 63),
		Item::new("minecraft", "jungle_wood", 64),
		Item::new("minecraft", "acacia_wood", 65),
		Item::new("minecraft", "dark_oak_wood", 66),
		Item::new("minecraft", "crimson_hyphae", 67),
		Item::new("minecraft", "warped_hyphae", 68),
		Item::new("minecraft", "oak_leaves", 69),
		Item::new("minecraft", "spruce_leaves", 70),
		Item::new("minecraft", "birch_leaves", 71),
		Item::new("minecraft", "jungle_leaves", 72),
		Item::new("minecraft", "acacia_leaves", 73),
		Item::new("minecraft", "dark_oak_leaves", 74),
		Item::new("minecraft", "sponge", 75),
		Item::new("minecraft", "wet_sponge", 76),
		Item::new("minecraft", "glass", 77),
		Item::new("minecraft", "lapis_ore", 78),
		Item::new("minecraft", "lapis_block", 79),
		Item::new("minecraft", "dispenser", 80),
		Item::new("minecraft", "sandstone", 81),
		Item::new("minecraft", "chiseled_sandstone", 82),
		Item::new("minecraft", "cut_sandstone", 83),
		Item::new("minecraft", "note_block", 84),
		Item::new("minecraft", "powered_rail", 85),
		Item::new("minecraft", "detector_rail", 86),
		Item::new("minecraft", "sticky_piston", 87),
		Item::new("minecraft", "cobweb", 88),
		Item::new("minecraft", "grass", 89),
		Item::new("minecraft", "fern", 90),
		Item::new("minecraft", "dead_bush", 91),
		Item::new("minecraft", "seagrass", 92),
		Item::new("minecraft", "sea_pickle", 93),
		Item::new("minecraft", "piston", 94),
		Item::new("minecraft", "white_wool", 95),
		Item::new("minecraft", "orange_wool", 96),
		Item::new("minecraft", "magenta_wool", 97),
		Item::new("minecraft", "light_blue_wool", 98),
		Item::new("minecraft", "yellow_wool", 99),
		Item::new("minecraft", "lime_wool", 100),
		Item::new("minecraft", "pink_wool", 101),
		Item::new("minecraft", "gray_wool", 102),
		Item::new("minecraft", "light_gray_wool", 103),
		Item::new("minecraft", "cyan_wool", 104),
		Item::new("minecraft", "purple_wool", 105),
		Item::new("minecraft", "blue_wool", 106),
		Item::new("minecraft", "brown_wool", 107),
		Item::new("minecraft", "green_wool", 108),
		Item::new("minecraft", "red_wool", 109),
		Item::new("minecraft", "black_wool", 110),
		Item::new("minecraft", "dandelion", 111),
		Item::new("minecraft", "poppy", 112),
		Item::new("minecraft", "blue_orchid", 113),
		Item::new("minecraft", "allium", 114),
		Item::new("minecraft", "azure_bluet", 115),
		Item::new("minecraft", "red_tulip", 116),
		Item::new("minecraft", "orange_tulip", 117),
		Item::new("minecraft", "white_tulip", 118),
		Item::new("minecraft", "pink_tulip", 119),
		Item::new("minecraft", "oxeye_daisy", 120),
		Item::new("minecraft", "cornflower", 121),
		Item::new("minecraft", "lily_of_the_valley", 122),
		Item::new("minecraft", "wither_rose", 123),
		Item::new("minecraft", "brown_mushroom", 124),
		Item::new("minecraft", "red_mushroom", 125),
		Item::new("minecraft", "crimson_fungus", 126),
		Item::new("minecraft", "warped_fungus", 127),
		Item::new("minecraft", "crimson_roots", 128),
		Item::new("minecraft", "warped_roots", 129),
		Item::new("minecraft", "nether_sprouts", 130),
		Item::new("minecraft", "weeping_vines", 131),
		Item::new("minecraft", "twisting_vines", 132),
		Item::new("minecraft", "gold_block", 133),
		Item::new("minecraft", "iron_block", 134),
		Item::new("minecraft", "oak_slab", 135),
		Item::new("minecraft", "spruce_slab", 136),
		Item::new("minecraft", "birch_slab", 137),
		Item::new("minecraft", "jungle_slab", 138),
		Item::new("minecraft", "acacia_slab", 139),
		Item::new("minecraft", "dark_oak_slab", 140),
		Item::new("minecraft", "crimson_slab", 141),
		Item::new("minecraft", "warped_slab", 142),
		Item::new("minecraft", "stone_slab", 143),
		Item::new("minecraft", "smooth_stone_slab", 144),
		Item::new("minecraft", "sandstone_slab", 145),
		Item::new("minecraft", "cut_sandstone_slab", 146),
		Item::new("minecraft", "petrified_oak_slab", 147),
		Item::new("minecraft", "cobblestone_slab", 148),
		Item::new("minecraft", "brick_slab", 149),
		Item::new("minecraft", "stone_brick_slab", 150),
		Item::new("minecraft", "nether_brick_slab", 151),
		Item::new("minecraft", "quartz_slab", 152),
		Item::new("minecraft", "red_sandstone_slab", 153),
		Item::new("minecraft", "cut_red_sandstone_slab", 154),
		Item::new("minecraft", "purpur_slab", 155),
		Item::new("minecraft", "prismarine_slab", 156),
		Item::new("minecraft", "prismarine_brick_slab", 157),
		Item::new("minecraft", "dark_prismarine_slab", 158),
		Item::new("minecraft", "smooth_quartz", 159),
		Item::new("minecraft", "smooth_red_sandstone", 160),
		Item::new("minecraft", "smooth_sandstone", 161),
		Item::new("minecraft", "smooth_stone", 162),
		Item::new("minecraft", "bricks", 163),
		Item::new("minecraft", "tnt", 164),
		Item::new("minecraft", "bookshelf", 165),
		Item::new("minecraft", "mossy_cobblestone", 166),
		Item::new("minecraft", "obsidian", 167),
		Item::new("minecraft", "torch", 168),
		Item::new("minecraft", "end_rod", 169),
		Item::new("minecraft", "chorus_plant", 170),
		Item::new("minecraft", "chorus_flower", 171),
		Item::new("minecraft", "purpur_block", 172),
		Item::new("minecraft", "purpur_pillar", 173),
		Item::new("minecraft", "purpur_stairs", 174),
		Item::new("minecraft", "spawner", 175),
		Item::new("minecraft", "oak_stairs", 176),
		Item::new("minecraft", "chest", 177),
		Item::new("minecraft", "diamond_ore", 178),
		Item::new("minecraft", "diamond_block", 179),
		Item::new("minecraft", "crafting_table", 180),
		Item::new("minecraft", "farmland", 181),
		Item::new("minecraft", "furnace", 182),
		Item::new("minecraft", "ladder", 183),
		Item::new("minecraft", "rail", 184),
		Item::new("minecraft", "cobblestone_stairs", 185),
		Item::new("minecraft", "lever", 186),
		Item::new("minecraft", "stone_pressure_plate", 187),
		Item::new("minecraft", "oak_pressure_plate", 188),
		Item::new("minecraft", "spruce_pressure_plate", 189),
		Item::new("minecraft", "birch_pressure_plate", 190),
		Item::new("minecraft", "jungle_pressure_plate", 191),
		Item::new("minecraft", "acacia_pressure_plate", 192),
		Item::new("minecraft", "dark_oak_pressure_plate", 193),
		Item::new("minecraft", "crimson_pressure_plate", 194),
		Item::new("minecraft", "warped_pressure_plate", 195),
		Item::new("minecraft", "redstone_ore", 196),
		Item::new("minecraft", "redstone_torch", 197),
		Item::new("minecraft", "stone_button", 198),
		Item::new("minecraft", "snow", 199),
		Item::new("minecraft", "ice", 200),
		Item::new("minecraft", "snow_block", 201),
		Item::new("minecraft", "cactus", 202),
		Item::new("minecraft", "clay", 203),
		Item::new("minecraft", "jukebox", 204),
		Item::new("minecraft", "oak_fence", 205),
		Item::new("minecraft", "spruce_fence", 206),
		Item::new("minecraft", "birch_fence", 207),
		Item::new("minecraft", "jungle_fence", 208),
		Item::new("minecraft", "acacia_fence", 209),
		Item::new("minecraft", "dark_oak_fence", 210),
		Item::new("minecraft", "crimson_fence", 211),
		Item::new("minecraft", "warped_fence", 212),
		Item::new("minecraft", "pumpkin", 213),
		Item::new("minecraft", "carved_pumpkin", 214),
		Item::new("minecraft", "netherrack", 215),
		Item::new("minecraft", "soul_sand", 216),
		Item::new("minecraft", "soul_soil", 217),
		Item::new("minecraft", "basalt", 218),
		Item::new("minecraft", "polished_basalt", 219),
		Item::new("minecraft", "soul_fire_torch", 220),
		Item::new("minecraft", "glowstone", 221),
		Item::new("minecraft", "jack_o_lantern", 222),
		Item::new("minecraft", "oak_trapdoor", 223),
		Item::new("minecraft", "spruce_trapdoor", 224),
		Item::new("minecraft", "birch_trapdoor", 225),
		Item::new("minecraft", "jungle_trapdoor", 226),
		Item::new("minecraft", "acacia_trapdoor", 227),
		Item::new("minecraft", "dark_oak_trapdoor", 228),
		Item::new("minecraft", "crimson_trapdoor", 229),
		Item::new("minecraft", "warped_trapdoor", 230),
		Item::new("minecraft", "infested_stone", 231),
		Item::new("minecraft", "infested_cobblestone", 232),
		Item::new("minecraft", "infested_stone_bricks", 233),
		Item::new("minecraft", "infested_mossy_stone_bricks", 234),
		Item::new("minecraft", "infested_cracked_stone_bricks", 235),
		Item::new("minecraft", "infested_chiseled_stone_bricks", 236),
		Item::new("minecraft", "stone_bricks", 237),
		Item::new("minecraft", "mossy_stone_bricks", 238),
		Item::new("minecraft", "cracked_stone_bricks", 239),
		Item::new("minecraft", "chiseled_stone_bricks", 240),
		Item::new("minecraft", "brown_mushroom_block", 241),
		Item::new("minecraft", "red_mushroom_block", 242),
		Item::new("minecraft", "mushroom_stem", 243),
		Item::new("minecraft", "iron_bars", 244),
		Item::new("minecraft", "glass_pane", 245),
		Item::new("minecraft", "melon", 246),
		Item::new("minecraft", "vine", 247),
		Item::new("minecraft", "oak_fence_gate", 248),
		Item::new("minecraft", "spruce_fence_gate", 249),
		Item::new("minecraft", "birch_fence_gate", 250),
		Item::new("minecraft", "jungle_fence_gate", 251),
		Item::new("minecraft", "acacia_fence_gate", 252),
		Item::new("minecraft", "dark_oak_fence_gate", 253),
		Item::new("minecraft", "crimson_fence_gate", 254),
		Item::new("minecraft", "warped_fence_gate", 255),
		Item::new("minecraft", "brick_stairs", 256),
		Item::new("minecraft", "stone_brick_stairs", 257),
		Item::new("minecraft", "mycelium", 258),
		Item::new("minecraft", "lily_pad", 259),
		Item::new("minecraft", "nether_bricks", 260),
		Item::new("minecraft", "nether_brick_fence", 261),
		Item::new("minecraft", "nether_brick_stairs", 262),
		Item::new("minecraft", "enchanting_table", 263),
		Item::new("minecraft", "end_portal_frame", 264),
		Item::new("minecraft", "end_stone", 265),
		Item::new("minecraft", "end_stone_bricks", 266),
		Item::new("minecraft", "dragon_egg", 267),
		Item::new("minecraft", "redstone_lamp", 268),
		Item::new("minecraft", "sandstone_stairs", 269),
		Item::new("minecraft", "emerald_ore", 270),
		Item::new("minecraft", "ender_chest", 271),
		Item::new("minecraft", "tripwire_hook", 272),
		Item::new("minecraft", "emerald_block", 273),
		Item::new("minecraft", "spruce_stairs", 274),
		Item::new("minecraft", "birch_stairs", 275),
		Item::new("minecraft", "jungle_stairs", 276),
		Item::new("minecraft", "crimson_stairs", 277),
		Item::new("minecraft", "warped_stairs", 278),
		Item::new("minecraft", "command_block", 279),
		Item::new("minecraft", "beacon", 280),
		Item::new("minecraft", "cobblestone_wall", 281),
		Item::new("minecraft", "mossy_cobblestone_wall", 282),
		Item::new("minecraft", "brick_wall", 283),
		Item::new("minecraft", "prismarine_wall", 284),
		Item::new("minecraft", "red_sandstone_wall", 285),
		Item::new("minecraft", "mossy_stone_brick_wall", 286),
		Item::new("minecraft", "granite_wall", 287),
		Item::new("minecraft", "stone_brick_wall", 288),
		Item::new("minecraft", "nether_brick_wall", 289),
		Item::new("minecraft", "andesite_wall", 290),
		Item::new("minecraft", "red_nether_brick_wall", 291),
		Item::new("minecraft", "sandstone_wall", 292),
		Item::new("minecraft", "end_stone_brick_wall", 293),
		Item::new("minecraft", "diorite_wall", 294),
		Item::new("minecraft", "oak_button", 295),
		Item::new("minecraft", "spruce_button", 296),
		Item::new("minecraft", "birch_button", 297),
		Item::new("minecraft", "jungle_button", 298),
		Item::new("minecraft", "acacia_button", 299),
		Item::new("minecraft", "dark_oak_button", 300),
		Item::new("minecraft", "crimson_button", 301),
		Item::new("minecraft", "warped_button", 302),
		Item::new("minecraft", "anvil", 303),
		Item::new("minecraft", "chipped_anvil", 304),
		Item::new("minecraft", "damaged_anvil", 305),
		Item::new("minecraft", "trapped_chest", 306),
		Item::new("minecraft", "light_weighted_pressure_plate", 307),
		Item::new("minecraft", "heavy_weighted_pressure_plate", 308),
		Item::new("minecraft", "daylight_detector", 309),
		Item::new("minecraft", "redstone_block", 310),
		Item::new("minecraft", "nether_quartz_ore", 311),
		Item::new("minecraft", "hopper", 312),
		Item::new("minecraft", "chiseled_quartz_block", 313),
		Item::new("minecraft", "quartz_block", 314),
		Item::new("minecraft", "quartz_pillar", 315),
		Item::new("minecraft", "quartz_stairs", 316),
		Item::new("minecraft", "activator_rail", 317),
		Item::new("minecraft", "dropper", 318),
		Item::new("minecraft", "white_terracotta", 319),
		Item::new("minecraft", "orange_terracotta", 320),
		Item::new("minecraft", "magenta_terracotta", 321),
		Item::new("minecraft", "light_blue_terracotta", 322),
		Item::new("minecraft", "yellow_terracotta", 323),
		Item::new("minecraft", "lime_terracotta", 324),
		Item::new("minecraft", "pink_terracotta", 325),
		Item::new("minecraft", "gray_terracotta", 326),
		Item::new("minecraft", "light_gray_terracotta", 327),
		Item::new("minecraft", "cyan_terracotta", 328),
		Item::new("minecraft", "purple_terracotta", 329),
		Item::new("minecraft", "blue_terracotta", 330),
		Item::new("minecraft", "brown_terracotta", 331),
		Item::new("minecraft", "green_terracotta", 332),
		Item::new("minecraft", "red_terracotta", 333),
		Item::new("minecraft", "black_terracotta", 334),
		Item::new("minecraft", "barrier", 335),
		Item::new("minecraft", "iron_trapdoor", 336),
		Item::new("minecraft", "hay_block", 337),
		Item::new("minecraft", "white_carpet", 338),
		Item::new("minecraft", "orange_carpet", 339),
		Item::new("minecraft", "magenta_carpet", 340),
		Item::new("minecraft", "light_blue_carpet", 341),
		Item::new("minecraft", "yellow_carpet", 342),
		Item::new("minecraft", "lime_carpet", 343),
		Item::new("minecraft", "pink_carpet", 344),
		Item::new("minecraft", "gray_carpet", 345),
		Item::new("minecraft", "light_gray_carpet", 346),
		Item::new("minecraft", "cyan_carpet", 347),
		Item::new("minecraft", "purple_carpet", 348),
		Item::new("minecraft", "blue_carpet", 349),
		Item::new("minecraft", "brown_carpet", 350),
		Item::new("minecraft", "green_carpet", 351),
		Item::new("minecraft", "red_carpet", 352),
		Item::new("minecraft", "black_carpet", 353),
		Item::new("minecraft", "terracotta", 354),
		Item::new("minecraft", "coal_block", 355),
		Item::new("minecraft", "packed_ice", 356),
		Item::new("minecraft", "acacia_stairs", 357),
		Item::new("minecraft", "dark_oak_stairs", 358),
		Item::new("minecraft", "slime_block", 359),
		Item::new("minecraft", "grass_path", 360),
		Item::new("minecraft", "sunflower", 361),
		Item::new("minecraft", "lilac", 362),
		Item::new("minecraft", "rose_bush", 363),
		Item::new("minecraft", "peony", 364),
		Item::new("minecraft", "tall_grass", 365),
		Item::new("minecraft", "large_fern", 366),
		Item::new("minecraft", "white_stained_glass", 367),
		Item::new("minecraft", "orange_stained_glass", 368),
		Item::new("minecraft", "magenta_stained_glass", 369),
		Item::new("minecraft", "light_blue_stained_glass", 370),
		Item::new("minecraft", "yellow_stained_glass", 371),
		Item::new("minecraft", "lime_stained_glass", 372),
		Item::new("minecraft", "pink_stained_glass", 373),
		Item::new("minecraft", "gray_stained_glass", 374),
		Item::new("minecraft", "light_gray_stained_glass", 375),
		Item::new("minecraft", "cyan_stained_glass", 376),
		Item::new("minecraft", "purple_stained_glass", 377),
		Item::new("minecraft", "blue_stained_glass", 378),
		Item::new("minecraft", "brown_stained_glass", 379),
		Item::new("minecraft", "green_stained_glass", 380),
		Item::new("minecraft", "red_stained_glass", 381),
		Item::new("minecraft", "black_stained_glass", 382),
		Item::new("minecraft", "white_stained_glass_pane", 383),
		Item::new("minecraft", "orange_stained_glass_pane", 384),
		Item::new("minecraft", "magenta_stained_glass_pane", 385),
		Item::new("minecraft", "light_blue_stained_glass_pane", 386),
		Item::new("minecraft", "yellow_stained_glass_pane", 387),
		Item::new("minecraft", "lime_stained_glass_pane", 388),
		Item::new("minecraft", "pink_stained_glass_pane", 389),
		Item::new("minecraft", "gray_stained_glass_pane", 390),
		Item::new("minecraft", "light_gray_stained_glass_pane", 391),
		Item::new("minecraft", "cyan_stained_glass_pane", 392),
		Item::new("minecraft", "purple_stained_glass_pane", 393),
		Item::new("minecraft", "blue_stained_glass_pane", 394),
		Item::new("minecraft", "brown_stained_glass_pane", 395),
		Item::new("minecraft", "green_stained_glass_pane", 396),
		Item::new("minecraft", "red_stained_glass_pane", 397),
		Item::new("minecraft", "black_stained_glass_pane", 398),
		Item::new("minecraft", "prismarine", 399),
		Item::new("minecraft", "prismarine_bricks", 400),
		Item::new("minecraft", "dark_prismarine", 401),
		Item::new("minecraft", "prismarine_stairs", 402),
		Item::new("minecraft", "prismarine_brick_stairs", 403),
		Item::new("minecraft", "dark_prismarine_stairs", 404),
		Item::new("minecraft", "sea_lantern", 405),
		Item::new("minecraft", "red_sandstone", 406),
		Item::new("minecraft", "chiseled_red_sandstone", 407),
		Item::new("minecraft", "cut_red_sandstone", 408),
		Item::new("minecraft", "red_sandstone_stairs", 409),
		Item::new("minecraft", "repeating_command_block", 410),
		Item::new("minecraft", "chain_command_block", 411),
		Item::new("minecraft", "magma_block", 412),
		Item::new("minecraft", "nether_wart_block", 413),
		Item::new("minecraft", "warped_wart_block", 414),
		Item::new("minecraft", "red_nether_bricks", 415),
		Item::new("minecraft", "bone_block", 416),
		Item::new("minecraft", "structure_void", 417),
		Item::new("minecraft", "observer", 418),
		Item::new("minecraft", "shulker_box", 419),
		Item::new("minecraft", "white_shulker_box", 420),
		Item::new("minecraft", "orange_shulker_box", 421),
		Item::new("minecraft", "magenta_shulker_box", 422),
		Item::new("minecraft", "light_blue_shulker_box", 423),
		Item::new("minecraft", "yellow_shulker_box", 424),
		Item::new("minecraft", "lime_shulker_box", 425),
		Item::new("minecraft", "pink_shulker_box", 426),
		Item::new("minecraft", "gray_shulker_box", 427),
		Item::new("minecraft", "light_gray_shulker_box", 428),
		Item::new("minecraft", "cyan_shulker_box", 429),
		Item::new("minecraft", "purple_shulker_box", 430),
		Item::new("minecraft", "blue_shulker_box", 431),
		Item::new("minecraft", "brown_shulker_box", 432),
		Item::new("minecraft", "green_shulker_box", 433),
		Item::new("minecraft", "red_shulker_box", 434),
		Item::new("minecraft", "black_shulker_box", 435),
		Item::new("minecraft", "white_glazed_terracotta", 436),
		Item::new("minecraft", "orange_glazed_terracotta", 437),
		Item::new("minecraft", "magenta_glazed_terracotta", 438),
		Item::new("minecraft", "light_blue_glazed_terracotta", 439),
		Item::new("minecraft", "yellow_glazed_terracotta", 440),
		Item::new("minecraft", "lime_glazed_terracotta", 441),
		Item::new("minecraft", "pink_glazed_terracotta", 442),
		Item::new("minecraft", "gray_glazed_terracotta", 443),
		Item::new("minecraft", "light_gray_glazed_terracotta", 444),
		Item::new("minecraft", "cyan_glazed_terracotta", 445),
		Item::new("minecraft", "purple_glazed_terracotta", 446),
		Item::new("minecraft", "blue_glazed_terracotta", 447),
		Item::new("minecraft", "brown_glazed_terracotta", 448),
		Item::new("minecraft", "green_glazed_terracotta", 449),
		Item::new("minecraft", "red_glazed_terracotta", 450),
		Item::new("minecraft", "black_glazed_terracotta", 451),
		Item::new("minecraft", "white_concrete", 452),
		Item::new("minecraft", "orange_concrete", 453),
		Item::new("minecraft", "magenta_concrete", 454),
		Item::new("minecraft", "light_blue_concrete", 455),
		Item::new("minecraft", "yellow_concrete", 456),
		Item::new("minecraft", "lime_concrete", 457),
		Item::new("minecraft", "pink_concrete", 458),
		Item::new("minecraft", "gray_concrete", 459),
		Item::new("minecraft", "light_gray_concrete", 460),
		Item::new("minecraft", "cyan_concrete", 461),
		Item::new("minecraft", "purple_concrete", 462),
		Item::new("minecraft", "blue_concrete", 463),
		Item::new("minecraft", "brown_concrete", 464),
		Item::new("minecraft", "green_concrete", 465),
		Item::new("minecraft", "red_concrete", 466),
		Item::new("minecraft", "black_concrete", 467),
		Item::new("minecraft", "white_concrete_powder", 468),
		Item::new("minecraft", "orange_concrete_powder", 469),
		Item::new("minecraft", "magenta_concrete_powder", 470),
		Item::new("minecraft", "light_blue_concrete_powder", 471),
		Item::new("minecraft", "yellow_concrete_powder", 472),
		Item::new("minecraft", "lime_concrete_powder", 473),
		Item::new("minecraft", "pink_concrete_powder", 474),
		Item::new("minecraft", "gray_concrete_powder", 475),
		Item::new("minecraft", "light_gray_concrete_powder", 476),
		Item::new("minecraft", "cyan_concrete_powder", 477),
		Item::new("minecraft", "purple_concrete_powder", 478),
		Item::new("minecraft", "blue_concrete_powder", 479),
		Item::new("minecraft", "brown_concrete_powder", 480),
		Item::new("minecraft", "green_concrete_powder", 481),
		Item::new("minecraft", "red_concrete_powder", 482),
		Item::new("minecraft", "black_concrete_powder", 483),
		Item::new("minecraft", "turtle_egg", 484),
		Item::new("minecraft", "dead_tube_coral_block", 485),
		Item::new("minecraft", "dead_brain_coral_block", 486),
		Item::new("minecraft", "dead_bubble_coral_block", 487),
		Item::new("minecraft", "dead_fire_coral_block", 488),
		Item::new("minecraft", "dead_horn_coral_block", 489),
		Item::new("minecraft", "tube_coral_block", 490),
		Item::new("minecraft", "brain_coral_block", 491),
		Item::new("minecraft", "bubble_coral_block", 492),
		Item::new("minecraft", "fire_coral_block", 493),
		Item::new("minecraft", "horn_coral_block", 494),
		Item::new("minecraft", "tube_coral", 495),
		Item::new("minecraft", "brain_coral", 496),
		Item::new("minecraft", "bubble_coral", 497),
		Item::new("minecraft", "fire_coral", 498),
		Item::new("minecraft", "horn_coral", 499),
		Item::new("minecraft", "dead_brain_coral", 500),
		Item::new("minecraft", "dead_bubble_coral", 501),
		Item::new("minecraft", "dead_fire_coral", 502),
		Item::new("minecraft", "dead_horn_coral", 503),
		Item::new("minecraft", "dead_tube_coral", 504),
		Item::new("minecraft", "tube_coral_fan", 505),
		Item::new("minecraft", "brain_coral_fan", 506),
		Item::new("minecraft", "bubble_coral_fan", 507),
		Item::new("minecraft", "fire_coral_fan", 508),
		Item::new("minecraft", "horn_coral_fan", 509),
		Item::new("minecraft", "dead_tube_coral_fan", 510),
		Item::new("minecraft", "dead_brain_coral_fan", 511),
		Item::new("minecraft", "dead_bubble_coral_fan", 512),
		Item::new("minecraft", "dead_fire_coral_fan", 513),
		Item::new("minecraft", "dead_horn_coral_fan", 514),
		Item::new("minecraft", "blue_ice", 515),
		Item::new("minecraft", "conduit", 516),
		Item::new("minecraft", "polished_granite_stairs", 517),
		Item::new("minecraft", "smooth_red_sandstone_stairs", 518),
		Item::new("minecraft", "mossy_stone_brick_stairs", 519),
		Item::new("minecraft", "polished_diorite_stairs", 520),
		Item::new("minecraft", "mossy_cobblestone_stairs", 521),
		Item::new("minecraft", "end_stone_brick_stairs", 522),
		Item::new("minecraft", "stone_stairs", 523),
		Item::new("minecraft", "smooth_sandstone_stairs", 524),
		Item::new("minecraft", "smooth_quartz_stairs", 525),
		Item::new("minecraft", "granite_stairs", 526),
		Item::new("minecraft", "andesite_stairs", 527),
		Item::new("minecraft", "red_nether_brick_stairs", 528),
		Item::new("minecraft", "polished_andesite_stairs", 529),
		Item::new("minecraft", "diorite_stairs", 530),
		Item::new("minecraft", "polished_granite_slab", 531),
		Item::new("minecraft", "smooth_red_sandstone_slab", 532),
		Item::new("minecraft", "mossy_stone_brick_slab", 533),
		Item::new("minecraft", "polished_diorite_slab", 534),
		Item::new("minecraft", "mossy_cobblestone_slab", 535),
		Item::new("minecraft", "end_stone_brick_slab", 536),
		Item::new("minecraft", "smooth_sandstone_slab", 537),
		Item::new("minecraft", "smooth_quartz_slab", 538),
		Item::new("minecraft", "granite_slab", 539),
		Item::new("minecraft", "andesite_slab", 540),
		Item::new("minecraft", "red_nether_brick_slab", 541),
		Item::new("minecraft", "polished_andesite_slab", 542),
		Item::new("minecraft", "diorite_slab", 543),
		Item::new("minecraft", "scaffolding", 544),
		Item::new("minecraft", "iron_door", 545),
		Item::new("minecraft", "oak_door", 546),
		Item::new("minecraft", "spruce_door", 547),
		Item::new("minecraft", "birch_door", 548),
		Item::new("minecraft", "jungle_door", 549),
		Item::new("minecraft", "acacia_door", 550),
		Item::new("minecraft", "dark_oak_door", 551),
		Item::new("minecraft", "crimson_door", 552),
		Item::new("minecraft", "warped_door", 553),
		Item::new("minecraft", "repeater", 554),
		Item::new("minecraft", "comparator", 555),
		Item::new("minecraft", "structure_block", 556),
		Item::new("minecraft", "jigsaw", 557),
		Item::new("minecraft", "turtle_helmet", 558),
		Item::new("minecraft", "scute", 559),
		Item::new("minecraft", "iron_shovel", 560),
		Item::new("minecraft", "iron_pickaxe", 561),
		Item::new("minecraft", "iron_axe", 562),
		Item::new("minecraft", "flint_and_steel", 563),
		Item::new("minecraft", "apple", 564),
		Item::new("minecraft", "bow", 565),
		Item::new("minecraft", "arrow", 566),
		Item::new("minecraft", "coal", 567),
		Item::new("minecraft", "charcoal", 568),
		Item::new("minecraft", "diamond", 569),
		Item::new("minecraft", "iron_ingot", 570),
		Item::new("minecraft", "gold_ingot", 571),
		Item::new("minecraft", "iron_sword", 572),
		Item::new("minecraft", "wooden_sword", 573),
		Item::new("minecraft", "wooden_shovel", 574),
		Item::new("minecraft", "wooden_pickaxe", 575),
		Item::new("minecraft", "wooden_axe", 576),
		Item::new("minecraft", "stone_sword", 577),
		Item::new("minecraft", "stone_shovel", 578),
		Item::new("minecraft", "stone_pickaxe", 579),
		Item::new("minecraft", "stone_axe", 580),
		Item::new("minecraft", "diamond_sword", 581),
		Item::new("minecraft", "diamond_shovel", 582),
		Item::new("minecraft", "diamond_pickaxe", 583),
		Item::new("minecraft", "diamond_axe", 584),
		Item::new("minecraft", "stick", 585),
		Item::new("minecraft", "bowl", 586),
		Item::new("minecraft", "mushroom_stew", 587),
		Item::new("minecraft", "golden_sword", 588),
		Item::new("minecraft", "golden_shovel", 589),
		Item::new("minecraft", "golden_pickaxe", 590),
		Item::new("minecraft", "golden_axe", 591),
		Item::new("minecraft", "netherite_sword", 592),
		Item::new("minecraft", "netherite_shovel", 593),
		Item::new("minecraft", "netherite_pickaxe", 594),
		Item::new("minecraft", "netherite_axe", 595),
		Item::new("minecraft", "string", 596),
		Item::new("minecraft", "feather", 597),
		Item::new("minecraft", "gunpowder", 598),
		Item::new("minecraft", "wooden_hoe", 599),
		Item::new("minecraft", "stone_hoe", 600),
		Item::new("minecraft", "iron_hoe", 601),
		Item::new("minecraft", "diamond_hoe", 602),
		Item::new("minecraft", "golden_hoe", 603),
		Item::new("minecraft", "netherite_hoe", 604),
		Item::new("minecraft", "wheat_seeds", 605),
		Item::new("minecraft", "wheat", 606),
		Item::new("minecraft", "bread", 607),
		Item::new("minecraft", "leather_helmet", 608),
		Item::new("minecraft", "leather_chestplate", 609),
		Item::new("minecraft", "leather_leggings", 610),
		Item::new("minecraft", "leather_boots", 611),
		Item::new("minecraft", "chainmail_helmet", 612),
		Item::new("minecraft", "chainmail_chestplate", 613),
		Item::new("minecraft", "chainmail_leggings", 614),
		Item::new("minecraft", "chainmail_boots", 615),
		Item::new("minecraft", "iron_helmet", 616),
		Item::new("minecraft", "iron_chestplate", 617),
		Item::new("minecraft", "iron_leggings", 618),
		Item::new("minecraft", "iron_boots", 619),
		Item::new("minecraft", "diamond_helmet", 620),
		Item::new("minecraft", "diamond_chestplate", 621),
		Item::new("minecraft", "diamond_leggings", 622),
		Item::new("minecraft", "diamond_boots", 623),
		Item::new("minecraft", "golden_helmet", 624),
		Item::new("minecraft", "golden_chestplate", 625),
		Item::new("minecraft", "golden_leggings", 626),
		Item::new("minecraft", "golden_boots", 627),
		Item::new("minecraft", "netherite_helmet", 628),
		Item::new("minecraft", "netherite_chestplate", 629),
		Item::new("minecraft", "netherite_leggings", 630),
		Item::new("minecraft", "netherite_boots", 631),
		Item::new("minecraft", "flint", 632),
		Item::new("minecraft", "porkchop", 633),
		Item::new("minecraft", "cooked_porkchop", 634),
		Item::new("minecraft", "painting", 635),
		Item::new("minecraft", "golden_apple", 636),
		Item::new("minecraft", "enchanted_golden_apple", 637),
		Item::new("minecraft", "oak_sign", 638),
		Item::new("minecraft", "spruce_sign", 639),
		Item::new("minecraft", "birch_sign", 640),
		Item::new("minecraft", "jungle_sign", 641),
		Item::new("minecraft", "acacia_sign", 642),
		Item::new("minecraft", "dark_oak_sign", 643),
		Item::new("minecraft", "crimson_sign", 644),
		Item::new("minecraft", "warped_sign", 645),
		Item::new("minecraft", "bucket", 646),
		Item::new("minecraft", "water_bucket", 647),
		Item::new("minecraft", "lava_bucket", 648),
		Item::new("minecraft", "minecart", 649),
		Item::new("minecraft", "saddle", 650),
		Item::new("minecraft", "redstone", 651),
		Item::new("minecraft", "snowball", 652),
		Item::new("minecraft", "oak_boat", 653),
		Item::new("minecraft", "leather", 654),
		Item::new("minecraft", "milk_bucket", 655),
		Item::new("minecraft", "pufferfish_bucket", 656),
		Item::new("minecraft", "salmon_bucket", 657),
		Item::new("minecraft", "cod_bucket", 658),
		Item::new("minecraft", "tropical_fish_bucket", 659),
		Item::new("minecraft", "brick", 660),
		Item::new("minecraft", "clay_ball", 661),
		Item::new("minecraft", "sugar_cane", 662),
		Item::new("minecraft", "kelp", 663),
		Item::new("minecraft", "dried_kelp_block", 664),
		Item::new("minecraft", "bamboo", 665),
		Item::new("minecraft", "paper", 666),
		Item::new("minecraft", "book", 667),
		Item::new("minecraft", "slime_ball", 668),
		Item::new("minecraft", "chest_minecart", 669),
		Item::new("minecraft", "furnace_minecart", 670),
		Item::new("minecraft", "egg", 671),
		Item::new("minecraft", "compass", 672),
		Item::new("minecraft", "fishing_rod", 673),
		Item::new("minecraft", "clock", 674),
		Item::new("minecraft", "glowstone_dust", 675),
		Item::new("minecraft", "cod", 676),
		Item::new("minecraft", "salmon", 677),
		Item::new("minecraft", "tropical_fish", 678),
		Item::new("minecraft", "pufferfish", 679),
		Item::new("minecraft", "cooked_cod", 680),
		Item::new("minecraft", "cooked_salmon", 681),
		Item::new("minecraft", "ink_sac", 682),
		Item::new("minecraft", "red_dye", 683),
		Item::new("minecraft", "green_dye", 684),
		Item::new("minecraft", "cocoa_beans", 685),
		Item::new("minecraft", "lapis_lazuli", 686),
		Item::new("minecraft", "purple_dye", 687),
		Item::new("minecraft", "cyan_dye", 688),
		Item::new("minecraft", "light_gray_dye", 689),
		Item::new("minecraft", "gray_dye", 690),
		Item::new("minecraft", "pink_dye", 691),
		Item::new("minecraft", "lime_dye", 692),
		Item::new("minecraft", "yellow_dye", 693),
		Item::new("minecraft", "light_blue_dye", 694),
		Item::new("minecraft", "magenta_dye", 695),
		Item::new("minecraft", "orange_dye", 696),
		Item::new("minecraft", "bone_meal", 697),
		Item::new("minecraft", "blue_dye", 698),
		Item::new("minecraft", "brown_dye", 699),
		Item::new("minecraft", "black_dye", 700),
		Item::new("minecraft", "white_dye", 701),
		Item::new("minecraft", "bone", 702),
		Item::new("minecraft", "sugar", 703),
		Item::new("minecraft", "cake", 704),
		Item::new("minecraft", "white_bed", 705),
		Item::new("minecraft", "orange_bed", 706),
		Item::new("minecraft", "magenta_bed", 707),
		Item::new("minecraft", "light_blue_bed", 708),
		Item::new("minecraft", "yellow_bed", 709),
		Item::new("minecraft", "lime_bed", 710),
		Item::new("minecraft", "pink_bed", 711),
		Item::new("minecraft", "gray_bed", 712),
		Item::new("minecraft", "light_gray_bed", 713),
		Item::new("minecraft", "cyan_bed", 714),
		Item::new("minecraft", "purple_bed", 715),
		Item::new("minecraft", "blue_bed", 716),
		Item::new("minecraft", "brown_bed", 717),
		Item::new("minecraft", "green_bed", 718),
		Item::new("minecraft", "red_bed", 719),
		Item::new("minecraft", "black_bed", 720),
		Item::new("minecraft", "cookie", 721),
		Item::new("minecraft", "filled_map", 722),
		Item::new("minecraft", "shears", 723),
		Item::new("minecraft", "melon_slice", 724),
		Item::new("minecraft", "dried_kelp", 725),
		Item::new("minecraft", "pumpkin_seeds", 726),
		Item::new("minecraft", "melon_seeds", 727),
		Item::new("minecraft", "beef", 728),
		Item::new("minecraft", "cooked_beef", 729),
		Item::new("minecraft", "chicken", 730),
		Item::new("minecraft", "cooked_chicken", 731),
		Item::new("minecraft", "rotten_flesh", 732),
		Item::new("minecraft", "ender_pearl", 733),
		Item::new("minecraft", "blaze_rod", 734),
		Item::new("minecraft", "ghast_tear", 735),
		Item::new("minecraft", "gold_nugget", 736),
		Item::new("minecraft", "nether_wart", 737),
		Item::new("minecraft", "potion", 738),
		Item::new("minecraft", "glass_bottle", 739),
		Item::new("minecraft", "spider_eye", 740),
		Item::new("minecraft", "fermented_spider_eye", 741),
		Item::new("minecraft", "blaze_powder", 742),
		Item::new("minecraft", "magma_cream", 743),
		Item::new("minecraft", "brewing_stand", 744),
		Item::new("minecraft", "cauldron", 745),
		Item::new("minecraft", "ender_eye", 746),
		Item::new("minecraft", "glistering_melon_slice", 747),
		Item::new("minecraft", "bat_spawn_egg", 748),
		Item::new("minecraft", "bee_spawn_egg", 749),
		Item::new("minecraft", "blaze_spawn_egg", 750),
		Item::new("minecraft", "cat_spawn_egg", 751),
		Item::new("minecraft", "cave_spider_spawn_egg", 752),
		Item::new("minecraft", "chicken_spawn_egg", 753),
		Item::new("minecraft", "cod_spawn_egg", 754),
		Item::new("minecraft", "cow_spawn_egg", 755),
		Item::new("minecraft", "creeper_spawn_egg", 756),
		Item::new("minecraft", "dolphin_spawn_egg", 757),
		Item::new("minecraft", "donkey_spawn_egg", 758),
		Item::new("minecraft", "drowned_spawn_egg", 759),
		Item::new("minecraft", "elder_guardian_spawn_egg", 760),
		Item::new("minecraft", "enderman_spawn_egg", 761),
		Item::new("minecraft", "endermite_spawn_egg", 762),
		Item::new("minecraft", "evoker_spawn_egg", 763),
		Item::new("minecraft", "fox_spawn_egg", 764),
		Item::new("minecraft", "ghast_spawn_egg", 765),
		Item::new("minecraft", "guardian_spawn_egg", 766),
		Item::new("minecraft", "hoglin_spawn_egg", 767),
		Item::new("minecraft", "horse_spawn_egg", 768),
		Item::new("minecraft", "husk_spawn_egg", 769),
		Item::new("minecraft", "llama_spawn_egg", 770),
		Item::new("minecraft", "magma_cube_spawn_egg", 771),
		Item::new("minecraft", "mooshroom_spawn_egg", 772),
		Item::new("minecraft", "mule_spawn_egg", 773),
		Item::new("minecraft", "ocelot_spawn_egg", 774),
		Item::new("minecraft", "panda_spawn_egg", 775),
		Item::new("minecraft", "parrot_spawn_egg", 776),
		Item::new("minecraft", "phantom_spawn_egg", 777),
		Item::new("minecraft", "pig_spawn_egg", 778),
		Item::new("minecraft", "piglin_spawn_egg", 779),
		Item::new("minecraft", "pillager_spawn_egg", 780),
		Item::new("minecraft", "polar_bear_spawn_egg", 781),
		Item::new("minecraft", "pufferfish_spawn_egg", 782),
		Item::new("minecraft", "rabbit_spawn_egg", 783),
		Item::new("minecraft", "ravager_spawn_egg", 784),
		Item::new("minecraft", "salmon_spawn_egg", 785),
		Item::new("minecraft", "sheep_spawn_egg", 786),
		Item::new("minecraft", "shulker_spawn_egg", 787),
		Item::new("minecraft", "silverfish_spawn_egg", 788),
		Item::new("minecraft", "skeleton_spawn_egg", 789),
		Item::new("minecraft", "skeleton_horse_spawn_egg", 790),
		Item::new("minecraft", "slime_spawn_egg", 791),
		Item::new("minecraft", "spider_spawn_egg", 792),
		Item::new("minecraft", "squid_spawn_egg", 793),
		Item::new("minecraft", "stray_spawn_egg", 794),
		Item::new("minecraft", "trader_llama_spawn_egg", 795),
		Item::new("minecraft", "tropical_fish_spawn_egg", 796),
		Item::new("minecraft", "turtle_spawn_egg", 797),
		Item::new("minecraft", "vex_spawn_egg", 798),
		Item::new("minecraft", "villager_spawn_egg", 799),
		Item::new("minecraft", "vindicator_spawn_egg", 800),
		Item::new("minecraft", "wandering_trader_spawn_egg", 801),
		Item::new("minecraft", "witch_spawn_egg", 802),
		Item::new("minecraft", "wither_skeleton_spawn_egg", 803),
		Item::new("minecraft", "wolf_spawn_egg", 804),
		Item::new("minecraft", "zombie_spawn_egg", 805),
		Item::new("minecraft", "zombie_horse_spawn_egg", 806),
		Item::new("minecraft", "zombified_piglin_spawn_egg", 807),
		Item::new("minecraft", "zombie_villager_spawn_egg", 808),
		Item::new("minecraft", "experience_bottle", 809),
		Item::new("minecraft", "fire_charge", 810),
		Item::new("minecraft", "writable_book", 811),
		Item::new("minecraft", "written_book", 812),
		Item::new("minecraft", "emerald", 813),
		Item::new("minecraft", "item_frame", 814),
		Item::new("minecraft", "flower_pot", 815),
		Item::new("minecraft", "carrot", 816),
		Item::new("minecraft", "potato", 817),
		Item::new("minecraft", "baked_potato", 818),
		Item::new("minecraft", "poisonous_potato", 819),
		Item::new("minecraft", "map", 820),
		Item::new("minecraft", "golden_carrot", 821),
		Item::new("minecraft", "skeleton_skull", 822),
		Item::new("minecraft", "wither_skeleton_skull", 823),
		Item::new("minecraft", "player_head", 824),
		Item::new("minecraft", "zombie_head", 825),
		Item::new("minecraft", "creeper_head", 826),
		Item::new("minecraft", "dragon_head", 827),
		Item::new("minecraft", "carrot_on_a_stick", 828),
		Item::new("minecraft", "nether_star", 829),
		Item::new("minecraft", "pumpkin_pie", 830),
		Item::new("minecraft", "firework_rocket", 831),
		Item::new("minecraft", "firework_star", 832),
		Item::new("minecraft", "enchanted_book", 833),
		Item::new("minecraft", "nether_brick", 834),
		Item::new("minecraft", "quartz", 835),
		Item::new("minecraft", "tnt_minecart", 836),
		Item::new("minecraft", "hopper_minecart", 837),
		Item::new("minecraft", "prismarine_shard", 838),
		Item::new("minecraft", "prismarine_crystals", 839),
		Item::new("minecraft", "rabbit", 840),
		Item::new("minecraft", "cooked_rabbit", 841),
		Item::new("minecraft", "rabbit_stew", 842),
		Item::new("minecraft", "rabbit_foot", 843),
		Item::new("minecraft", "rabbit_hide", 844),
		Item::new("minecraft", "armor_stand", 845),
		Item::new("minecraft", "iron_horse_armor", 846),
		Item::new("minecraft", "golden_horse_armor", 847),
		Item::new("minecraft", "diamond_horse_armor", 848),
		Item::new("minecraft", "leather_horse_armor", 849),
		Item::new("minecraft", "lead", 850),
		Item::new("minecraft", "name_tag", 851),
		Item::new("minecraft", "command_block_minecart", 852),
		Item::new("minecraft", "mutton", 853),
		Item::new("minecraft", "cooked_mutton", 854),
		Item::new("minecraft", "white_banner", 855),
		Item::new("minecraft", "orange_banner", 856),
		Item::new("minecraft", "magenta_banner", 857),
		Item::new("minecraft", "light_blue_banner", 858),
		Item::new("minecraft", "yellow_banner", 859),
		Item::new("minecraft", "lime_banner", 860),
		Item::new("minecraft", "pink_banner", 861),
		Item::new("minecraft", "gray_banner", 862),
		Item::new("minecraft", "light_gray_banner", 863),
		Item::new("minecraft", "cyan_banner", 864),
		Item::new("minecraft", "purple_banner", 865),
		Item::new("minecraft", "blue_banner", 866),
		Item::new("minecraft", "brown_banner", 867),
		Item::new("minecraft", "green_banner", 868),
		Item::new("minecraft", "red_banner", 869),
		Item::new("minecraft", "black_banner", 870),
		Item::new("minecraft", "end_crystal", 871),
		Item::new("minecraft", "chorus_fruit", 872),
		Item::new("minecraft", "popped_chorus_fruit", 873),
		Item::new("minecraft", "beetroot", 874),
		Item::new("minecraft", "beetroot_seeds", 875),
		Item::new("minecraft", "beetroot_soup", 876),
		Item::new("minecraft", "dragon_breath", 877),
		Item::new("minecraft", "splash_potion", 878),
		Item::new("minecraft", "spectral_arrow", 879),
		Item::new("minecraft", "tipped_arrow", 880),
		Item::new("minecraft", "lingering_potion", 881),
		Item::new("minecraft", "shield", 882),
		Item::new("minecraft", "elytra", 883),
		Item::new("minecraft", "spruce_boat", 884),
		Item::new("minecraft", "birch_boat", 885),
		Item::new("minecraft", "jungle_boat", 886),
		Item::new("minecraft", "acacia_boat", 887),
		Item::new("minecraft", "dark_oak_boat", 888),
		Item::new("minecraft", "totem_of_undying", 889),
		Item::new("minecraft", "shulker_shell", 890),
		Item::new("minecraft", "iron_nugget", 891),
		Item::new("minecraft", "knowledge_book", 892),
		Item::new("minecraft", "debug_stick", 893),
		Item::new("minecraft", "music_disc_13", 894),
		Item::new("minecraft", "music_disc_cat", 895),
		Item::new("minecraft", "music_disc_blocks", 896),
		Item::new("minecraft", "music_disc_chirp", 897),
		Item::new("minecraft", "music_disc_far", 898),
		Item::new("minecraft", "music_disc_mall", 899),
		Item::new("minecraft", "music_disc_mellohi", 900),
		Item::new("minecraft", "music_disc_stal", 901),
		Item::new("minecraft", "music_disc_strad", 902),
		Item::new("minecraft", "music_disc_ward", 903),
		Item::new("minecraft", "music_disc_11", 904),
		Item::new("minecraft", "music_disc_wait", 905),
		Item::new("minecraft", "trident", 906),
		Item::new("minecraft", "phantom_membrane", 907),
		Item::new("minecraft", "nautilus_shell", 908),
		Item::new("minecraft", "heart_of_the_sea", 909),
		Item::new("minecraft", "crossbow", 910),
		Item::new("minecraft", "suspicious_stew", 911),
		Item::new("minecraft", "loom", 912),
		Item::new("minecraft", "flower_banner_pattern", 913),
		Item::new("minecraft", "creeper_banner_pattern", 914),
		Item::new("minecraft", "skull_banner_pattern", 915),
		Item::new("minecraft", "mojang_banner_pattern", 916),
		Item::new("minecraft", "globe_banner_pattern", 917),
		Item::new("minecraft", "composter", 918),
		Item::new("minecraft", "barrel", 919),
		Item::new("minecraft", "smoker", 920),
		Item::new("minecraft", "blast_furnace", 921),
		Item::new("minecraft", "cartography_table", 922),
		Item::new("minecraft", "fletching_table", 923),
		Item::new("minecraft", "grindstone", 924),
		Item::new("minecraft", "lectern", 925),
		Item::new("minecraft", "smithing_table", 926),
		Item::new("minecraft", "stonecutter", 927),
		Item::new("minecraft", "bell", 928),
		Item::new("minecraft", "lantern", 929),
		Item::new("minecraft", "soul_fire_lantern", 930),
		Item::new("minecraft", "sweet_berries", 931),
		Item::new("minecraft", "campfire", 932),
		Item::new("minecraft", "shroomlight", 933),
		Item::new("minecraft", "honeycomb", 934),
		Item::new("minecraft", "bee_nest", 935),
		Item::new("minecraft", "beehive", 936),
		Item::new("minecraft", "honey_bottle", 937),
		Item::new("minecraft", "honey_block", 938),
		Item::new("minecraft", "honeycomb_block", 939),
		Item::new("minecraft", "netherite_block", 940),
		Item::new("minecraft", "ancient_debris", 941),
		Item::new("minecraft", "netherite_ingot", 942),
		Item::new("minecraft", "netherite_scrap", 943),
		Item::new("minecraft", "target", 944),
		Item::new("minecraft", "crying_obsidian", 945),
		Item::new("minecraft", "respawn_anchor", 946)
	]
}

pub fn get(item: i32) -> Option<Item> {
	match item {
		0 => Some(Item::new("minecraft", "air", 0)), 
		1 => Some(Item::new("minecraft", "stone", 1)), 
		2 => Some(Item::new("minecraft", "granite", 2)), 
		3 => Some(Item::new("minecraft", "polished_granite", 3)), 
		4 => Some(Item::new("minecraft", "diorite", 4)), 
		5 => Some(Item::new("minecraft", "polished_diorite", 5)), 
		6 => Some(Item::new("minecraft", "andesite", 6)), 
		7 => Some(Item::new("minecraft", "polished_andesite", 7)), 
		8 => Some(Item::new("minecraft", "grass_block", 8)), 
		9 => Some(Item::new("minecraft", "dirt", 9)), 
		10 => Some(Item::new("minecraft", "coarse_dirt", 10)), 
		11 => Some(Item::new("minecraft", "podzol", 11)), 
		12 => Some(Item::new("minecraft", "crimson_nylium", 12)), 
		13 => Some(Item::new("minecraft", "warped_nylium", 13)), 
		14 => Some(Item::new("minecraft", "cobblestone", 14)), 
		15 => Some(Item::new("minecraft", "oak_planks", 15)), 
		16 => Some(Item::new("minecraft", "spruce_planks", 16)), 
		17 => Some(Item::new("minecraft", "birch_planks", 17)), 
		18 => Some(Item::new("minecraft", "jungle_planks", 18)), 
		19 => Some(Item::new("minecraft", "acacia_planks", 19)), 
		20 => Some(Item::new("minecraft", "dark_oak_planks", 20)), 
		21 => Some(Item::new("minecraft", "crimson_planks", 21)), 
		22 => Some(Item::new("minecraft", "warped_planks", 22)), 
		23 => Some(Item::new("minecraft", "oak_sapling", 23)), 
		24 => Some(Item::new("minecraft", "spruce_sapling", 24)), 
		25 => Some(Item::new("minecraft", "birch_sapling", 25)), 
		26 => Some(Item::new("minecraft", "jungle_sapling", 26)), 
		27 => Some(Item::new("minecraft", "acacia_sapling", 27)), 
		28 => Some(Item::new("minecraft", "dark_oak_sapling", 28)), 
		29 => Some(Item::new("minecraft", "bedrock", 29)), 
		30 => Some(Item::new("minecraft", "sand", 30)), 
		31 => Some(Item::new("minecraft", "red_sand", 31)), 
		32 => Some(Item::new("minecraft", "gravel", 32)), 
		33 => Some(Item::new("minecraft", "gold_ore", 33)), 
		34 => Some(Item::new("minecraft", "iron_ore", 34)), 
		35 => Some(Item::new("minecraft", "coal_ore", 35)), 
		36 => Some(Item::new("minecraft", "nether_gold_ore", 36)), 
		37 => Some(Item::new("minecraft", "oak_log", 37)), 
		38 => Some(Item::new("minecraft", "spruce_log", 38)), 
		39 => Some(Item::new("minecraft", "birch_log", 39)), 
		40 => Some(Item::new("minecraft", "jungle_log", 40)), 
		41 => Some(Item::new("minecraft", "acacia_log", 41)), 
		42 => Some(Item::new("minecraft", "dark_oak_log", 42)), 
		43 => Some(Item::new("minecraft", "crimson_stem", 43)), 
		44 => Some(Item::new("minecraft", "warped_stem", 44)), 
		45 => Some(Item::new("minecraft", "stripped_oak_log", 45)), 
		46 => Some(Item::new("minecraft", "stripped_spruce_log", 46)), 
		47 => Some(Item::new("minecraft", "stripped_birch_log", 47)), 
		48 => Some(Item::new("minecraft", "stripped_jungle_log", 48)), 
		49 => Some(Item::new("minecraft", "stripped_acacia_log", 49)), 
		50 => Some(Item::new("minecraft", "stripped_dark_oak_log", 50)), 
		51 => Some(Item::new("minecraft", "stripped_crimson_stem", 51)), 
		52 => Some(Item::new("minecraft", "stripped_warped_stem", 52)), 
		53 => Some(Item::new("minecraft", "stripped_oak_wood", 53)), 
		54 => Some(Item::new("minecraft", "stripped_spruce_wood", 54)), 
		55 => Some(Item::new("minecraft", "stripped_birch_wood", 55)), 
		56 => Some(Item::new("minecraft", "stripped_jungle_wood", 56)), 
		57 => Some(Item::new("minecraft", "stripped_acacia_wood", 57)), 
		58 => Some(Item::new("minecraft", "stripped_dark_oak_wood", 58)), 
		59 => Some(Item::new("minecraft", "stripped_crimson_hyphae", 59)), 
		60 => Some(Item::new("minecraft", "stripped_warped_hyphae", 60)), 
		61 => Some(Item::new("minecraft", "oak_wood", 61)), 
		62 => Some(Item::new("minecraft", "spruce_wood", 62)), 
		63 => Some(Item::new("minecraft", "birch_wood", 63)), 
		64 => Some(Item::new("minecraft", "jungle_wood", 64)), 
		65 => Some(Item::new("minecraft", "acacia_wood", 65)), 
		66 => Some(Item::new("minecraft", "dark_oak_wood", 66)), 
		67 => Some(Item::new("minecraft", "crimson_hyphae", 67)), 
		68 => Some(Item::new("minecraft", "warped_hyphae", 68)), 
		69 => Some(Item::new("minecraft", "oak_leaves", 69)), 
		70 => Some(Item::new("minecraft", "spruce_leaves", 70)), 
		71 => Some(Item::new("minecraft", "birch_leaves", 71)), 
		72 => Some(Item::new("minecraft", "jungle_leaves", 72)), 
		73 => Some(Item::new("minecraft", "acacia_leaves", 73)), 
		74 => Some(Item::new("minecraft", "dark_oak_leaves", 74)), 
		75 => Some(Item::new("minecraft", "sponge", 75)), 
		76 => Some(Item::new("minecraft", "wet_sponge", 76)), 
		77 => Some(Item::new("minecraft", "glass", 77)), 
		78 => Some(Item::new("minecraft", "lapis_ore", 78)), 
		79 => Some(Item::new("minecraft", "lapis_block", 79)), 
		80 => Some(Item::new("minecraft", "dispenser", 80)), 
		81 => Some(Item::new("minecraft", "sandstone", 81)), 
		82 => Some(Item::new("minecraft", "chiseled_sandstone", 82)), 
		83 => Some(Item::new("minecraft", "cut_sandstone", 83)), 
		84 => Some(Item::new("minecraft", "note_block", 84)), 
		85 => Some(Item::new("minecraft", "powered_rail", 85)), 
		86 => Some(Item::new("minecraft", "detector_rail", 86)), 
		87 => Some(Item::new("minecraft", "sticky_piston", 87)), 
		88 => Some(Item::new("minecraft", "cobweb", 88)), 
		89 => Some(Item::new("minecraft", "grass", 89)), 
		90 => Some(Item::new("minecraft", "fern", 90)), 
		91 => Some(Item::new("minecraft", "dead_bush", 91)), 
		92 => Some(Item::new("minecraft", "seagrass", 92)), 
		93 => Some(Item::new("minecraft", "sea_pickle", 93)), 
		94 => Some(Item::new("minecraft", "piston", 94)), 
		95 => Some(Item::new("minecraft", "white_wool", 95)), 
		96 => Some(Item::new("minecraft", "orange_wool", 96)), 
		97 => Some(Item::new("minecraft", "magenta_wool", 97)), 
		98 => Some(Item::new("minecraft", "light_blue_wool", 98)), 
		99 => Some(Item::new("minecraft", "yellow_wool", 99)), 
		100 => Some(Item::new("minecraft", "lime_wool", 100)), 
		101 => Some(Item::new("minecraft", "pink_wool", 101)), 
		102 => Some(Item::new("minecraft", "gray_wool", 102)), 
		103 => Some(Item::new("minecraft", "light_gray_wool", 103)), 
		104 => Some(Item::new("minecraft", "cyan_wool", 104)), 
		105 => Some(Item::new("minecraft", "purple_wool", 105)), 
		106 => Some(Item::new("minecraft", "blue_wool", 106)), 
		107 => Some(Item::new("minecraft", "brown_wool", 107)), 
		108 => Some(Item::new("minecraft", "green_wool", 108)), 
		109 => Some(Item::new("minecraft", "red_wool", 109)), 
		110 => Some(Item::new("minecraft", "black_wool", 110)), 
		111 => Some(Item::new("minecraft", "dandelion", 111)), 
		112 => Some(Item::new("minecraft", "poppy", 112)), 
		113 => Some(Item::new("minecraft", "blue_orchid", 113)), 
		114 => Some(Item::new("minecraft", "allium", 114)), 
		115 => Some(Item::new("minecraft", "azure_bluet", 115)), 
		116 => Some(Item::new("minecraft", "red_tulip", 116)), 
		117 => Some(Item::new("minecraft", "orange_tulip", 117)), 
		118 => Some(Item::new("minecraft", "white_tulip", 118)), 
		119 => Some(Item::new("minecraft", "pink_tulip", 119)), 
		120 => Some(Item::new("minecraft", "oxeye_daisy", 120)), 
		121 => Some(Item::new("minecraft", "cornflower", 121)), 
		122 => Some(Item::new("minecraft", "lily_of_the_valley", 122)), 
		123 => Some(Item::new("minecraft", "wither_rose", 123)), 
		124 => Some(Item::new("minecraft", "brown_mushroom", 124)), 
		125 => Some(Item::new("minecraft", "red_mushroom", 125)), 
		126 => Some(Item::new("minecraft", "crimson_fungus", 126)), 
		127 => Some(Item::new("minecraft", "warped_fungus", 127)), 
		128 => Some(Item::new("minecraft", "crimson_roots", 128)), 
		129 => Some(Item::new("minecraft", "warped_roots", 129)), 
		130 => Some(Item::new("minecraft", "nether_sprouts", 130)), 
		131 => Some(Item::new("minecraft", "weeping_vines", 131)), 
		132 => Some(Item::new("minecraft", "twisting_vines", 132)), 
		133 => Some(Item::new("minecraft", "gold_block", 133)), 
		134 => Some(Item::new("minecraft", "iron_block", 134)), 
		135 => Some(Item::new("minecraft", "oak_slab", 135)), 
		136 => Some(Item::new("minecraft", "spruce_slab", 136)), 
		137 => Some(Item::new("minecraft", "birch_slab", 137)), 
		138 => Some(Item::new("minecraft", "jungle_slab", 138)), 
		139 => Some(Item::new("minecraft", "acacia_slab", 139)), 
		140 => Some(Item::new("minecraft", "dark_oak_slab", 140)), 
		141 => Some(Item::new("minecraft", "crimson_slab", 141)), 
		142 => Some(Item::new("minecraft", "warped_slab", 142)), 
		143 => Some(Item::new("minecraft", "stone_slab", 143)), 
		144 => Some(Item::new("minecraft", "smooth_stone_slab", 144)), 
		145 => Some(Item::new("minecraft", "sandstone_slab", 145)), 
		146 => Some(Item::new("minecraft", "cut_sandstone_slab", 146)), 
		147 => Some(Item::new("minecraft", "petrified_oak_slab", 147)), 
		148 => Some(Item::new("minecraft", "cobblestone_slab", 148)), 
		149 => Some(Item::new("minecraft", "brick_slab", 149)), 
		150 => Some(Item::new("minecraft", "stone_brick_slab", 150)), 
		151 => Some(Item::new("minecraft", "nether_brick_slab", 151)), 
		152 => Some(Item::new("minecraft", "quartz_slab", 152)), 
		153 => Some(Item::new("minecraft", "red_sandstone_slab", 153)), 
		154 => Some(Item::new("minecraft", "cut_red_sandstone_slab", 154)), 
		155 => Some(Item::new("minecraft", "purpur_slab", 155)), 
		156 => Some(Item::new("minecraft", "prismarine_slab", 156)), 
		157 => Some(Item::new("minecraft", "prismarine_brick_slab", 157)), 
		158 => Some(Item::new("minecraft", "dark_prismarine_slab", 158)), 
		159 => Some(Item::new("minecraft", "smooth_quartz", 159)), 
		160 => Some(Item::new("minecraft", "smooth_red_sandstone", 160)), 
		161 => Some(Item::new("minecraft", "smooth_sandstone", 161)), 
		162 => Some(Item::new("minecraft", "smooth_stone", 162)), 
		163 => Some(Item::new("minecraft", "bricks", 163)), 
		164 => Some(Item::new("minecraft", "tnt", 164)), 
		165 => Some(Item::new("minecraft", "bookshelf", 165)), 
		166 => Some(Item::new("minecraft", "mossy_cobblestone", 166)), 
		167 => Some(Item::new("minecraft", "obsidian", 167)), 
		168 => Some(Item::new("minecraft", "torch", 168)), 
		169 => Some(Item::new("minecraft", "end_rod", 169)), 
		170 => Some(Item::new("minecraft", "chorus_plant", 170)), 
		171 => Some(Item::new("minecraft", "chorus_flower", 171)), 
		172 => Some(Item::new("minecraft", "purpur_block", 172)), 
		173 => Some(Item::new("minecraft", "purpur_pillar", 173)), 
		174 => Some(Item::new("minecraft", "purpur_stairs", 174)), 
		175 => Some(Item::new("minecraft", "spawner", 175)), 
		176 => Some(Item::new("minecraft", "oak_stairs", 176)), 
		177 => Some(Item::new("minecraft", "chest", 177)), 
		178 => Some(Item::new("minecraft", "diamond_ore", 178)), 
		179 => Some(Item::new("minecraft", "diamond_block", 179)), 
		180 => Some(Item::new("minecraft", "crafting_table", 180)), 
		181 => Some(Item::new("minecraft", "farmland", 181)), 
		182 => Some(Item::new("minecraft", "furnace", 182)), 
		183 => Some(Item::new("minecraft", "ladder", 183)), 
		184 => Some(Item::new("minecraft", "rail", 184)), 
		185 => Some(Item::new("minecraft", "cobblestone_stairs", 185)), 
		186 => Some(Item::new("minecraft", "lever", 186)), 
		187 => Some(Item::new("minecraft", "stone_pressure_plate", 187)), 
		188 => Some(Item::new("minecraft", "oak_pressure_plate", 188)), 
		189 => Some(Item::new("minecraft", "spruce_pressure_plate", 189)), 
		190 => Some(Item::new("minecraft", "birch_pressure_plate", 190)), 
		191 => Some(Item::new("minecraft", "jungle_pressure_plate", 191)), 
		192 => Some(Item::new("minecraft", "acacia_pressure_plate", 192)), 
		193 => Some(Item::new("minecraft", "dark_oak_pressure_plate", 193)), 
		194 => Some(Item::new("minecraft", "crimson_pressure_plate", 194)), 
		195 => Some(Item::new("minecraft", "warped_pressure_plate", 195)), 
		196 => Some(Item::new("minecraft", "redstone_ore", 196)), 
		197 => Some(Item::new("minecraft", "redstone_torch", 197)), 
		198 => Some(Item::new("minecraft", "stone_button", 198)), 
		199 => Some(Item::new("minecraft", "snow", 199)), 
		200 => Some(Item::new("minecraft", "ice", 200)), 
		201 => Some(Item::new("minecraft", "snow_block", 201)), 
		202 => Some(Item::new("minecraft", "cactus", 202)), 
		203 => Some(Item::new("minecraft", "clay", 203)), 
		204 => Some(Item::new("minecraft", "jukebox", 204)), 
		205 => Some(Item::new("minecraft", "oak_fence", 205)), 
		206 => Some(Item::new("minecraft", "spruce_fence", 206)), 
		207 => Some(Item::new("minecraft", "birch_fence", 207)), 
		208 => Some(Item::new("minecraft", "jungle_fence", 208)), 
		209 => Some(Item::new("minecraft", "acacia_fence", 209)), 
		210 => Some(Item::new("minecraft", "dark_oak_fence", 210)), 
		211 => Some(Item::new("minecraft", "crimson_fence", 211)), 
		212 => Some(Item::new("minecraft", "warped_fence", 212)), 
		213 => Some(Item::new("minecraft", "pumpkin", 213)), 
		214 => Some(Item::new("minecraft", "carved_pumpkin", 214)), 
		215 => Some(Item::new("minecraft", "netherrack", 215)), 
		216 => Some(Item::new("minecraft", "soul_sand", 216)), 
		217 => Some(Item::new("minecraft", "soul_soil", 217)), 
		218 => Some(Item::new("minecraft", "basalt", 218)), 
		219 => Some(Item::new("minecraft", "polished_basalt", 219)), 
		220 => Some(Item::new("minecraft", "soul_fire_torch", 220)), 
		221 => Some(Item::new("minecraft", "glowstone", 221)), 
		222 => Some(Item::new("minecraft", "jack_o_lantern", 222)), 
		223 => Some(Item::new("minecraft", "oak_trapdoor", 223)), 
		224 => Some(Item::new("minecraft", "spruce_trapdoor", 224)), 
		225 => Some(Item::new("minecraft", "birch_trapdoor", 225)), 
		226 => Some(Item::new("minecraft", "jungle_trapdoor", 226)), 
		227 => Some(Item::new("minecraft", "acacia_trapdoor", 227)), 
		228 => Some(Item::new("minecraft", "dark_oak_trapdoor", 228)), 
		229 => Some(Item::new("minecraft", "crimson_trapdoor", 229)), 
		230 => Some(Item::new("minecraft", "warped_trapdoor", 230)), 
		231 => Some(Item::new("minecraft", "infested_stone", 231)), 
		232 => Some(Item::new("minecraft", "infested_cobblestone", 232)), 
		233 => Some(Item::new("minecraft", "infested_stone_bricks", 233)), 
		234 => Some(Item::new("minecraft", "infested_mossy_stone_bricks", 234)), 
		235 => Some(Item::new("minecraft", "infested_cracked_stone_bricks", 235)), 
		236 => Some(Item::new("minecraft", "infested_chiseled_stone_bricks", 236)), 
		237 => Some(Item::new("minecraft", "stone_bricks", 237)), 
		238 => Some(Item::new("minecraft", "mossy_stone_bricks", 238)), 
		239 => Some(Item::new("minecraft", "cracked_stone_bricks", 239)), 
		240 => Some(Item::new("minecraft", "chiseled_stone_bricks", 240)), 
		241 => Some(Item::new("minecraft", "brown_mushroom_block", 241)), 
		242 => Some(Item::new("minecraft", "red_mushroom_block", 242)), 
		243 => Some(Item::new("minecraft", "mushroom_stem", 243)), 
		244 => Some(Item::new("minecraft", "iron_bars", 244)), 
		245 => Some(Item::new("minecraft", "glass_pane", 245)), 
		246 => Some(Item::new("minecraft", "melon", 246)), 
		247 => Some(Item::new("minecraft", "vine", 247)), 
		248 => Some(Item::new("minecraft", "oak_fence_gate", 248)), 
		249 => Some(Item::new("minecraft", "spruce_fence_gate", 249)), 
		250 => Some(Item::new("minecraft", "birch_fence_gate", 250)), 
		251 => Some(Item::new("minecraft", "jungle_fence_gate", 251)), 
		252 => Some(Item::new("minecraft", "acacia_fence_gate", 252)), 
		253 => Some(Item::new("minecraft", "dark_oak_fence_gate", 253)), 
		254 => Some(Item::new("minecraft", "crimson_fence_gate", 254)), 
		255 => Some(Item::new("minecraft", "warped_fence_gate", 255)), 
		256 => Some(Item::new("minecraft", "brick_stairs", 256)), 
		257 => Some(Item::new("minecraft", "stone_brick_stairs", 257)), 
		258 => Some(Item::new("minecraft", "mycelium", 258)), 
		259 => Some(Item::new("minecraft", "lily_pad", 259)), 
		260 => Some(Item::new("minecraft", "nether_bricks", 260)), 
		261 => Some(Item::new("minecraft", "nether_brick_fence", 261)), 
		262 => Some(Item::new("minecraft", "nether_brick_stairs", 262)), 
		263 => Some(Item::new("minecraft", "enchanting_table", 263)), 
		264 => Some(Item::new("minecraft", "end_portal_frame", 264)), 
		265 => Some(Item::new("minecraft", "end_stone", 265)), 
		266 => Some(Item::new("minecraft", "end_stone_bricks", 266)), 
		267 => Some(Item::new("minecraft", "dragon_egg", 267)), 
		268 => Some(Item::new("minecraft", "redstone_lamp", 268)), 
		269 => Some(Item::new("minecraft", "sandstone_stairs", 269)), 
		270 => Some(Item::new("minecraft", "emerald_ore", 270)), 
		271 => Some(Item::new("minecraft", "ender_chest", 271)), 
		272 => Some(Item::new("minecraft", "tripwire_hook", 272)), 
		273 => Some(Item::new("minecraft", "emerald_block", 273)), 
		274 => Some(Item::new("minecraft", "spruce_stairs", 274)), 
		275 => Some(Item::new("minecraft", "birch_stairs", 275)), 
		276 => Some(Item::new("minecraft", "jungle_stairs", 276)), 
		277 => Some(Item::new("minecraft", "crimson_stairs", 277)), 
		278 => Some(Item::new("minecraft", "warped_stairs", 278)), 
		279 => Some(Item::new("minecraft", "command_block", 279)), 
		280 => Some(Item::new("minecraft", "beacon", 280)), 
		281 => Some(Item::new("minecraft", "cobblestone_wall", 281)), 
		282 => Some(Item::new("minecraft", "mossy_cobblestone_wall", 282)), 
		283 => Some(Item::new("minecraft", "brick_wall", 283)), 
		284 => Some(Item::new("minecraft", "prismarine_wall", 284)), 
		285 => Some(Item::new("minecraft", "red_sandstone_wall", 285)), 
		286 => Some(Item::new("minecraft", "mossy_stone_brick_wall", 286)), 
		287 => Some(Item::new("minecraft", "granite_wall", 287)), 
		288 => Some(Item::new("minecraft", "stone_brick_wall", 288)), 
		289 => Some(Item::new("minecraft", "nether_brick_wall", 289)), 
		290 => Some(Item::new("minecraft", "andesite_wall", 290)), 
		291 => Some(Item::new("minecraft", "red_nether_brick_wall", 291)), 
		292 => Some(Item::new("minecraft", "sandstone_wall", 292)), 
		293 => Some(Item::new("minecraft", "end_stone_brick_wall", 293)), 
		294 => Some(Item::new("minecraft", "diorite_wall", 294)), 
		295 => Some(Item::new("minecraft", "oak_button", 295)), 
		296 => Some(Item::new("minecraft", "spruce_button", 296)), 
		297 => Some(Item::new("minecraft", "birch_button", 297)), 
		298 => Some(Item::new("minecraft", "jungle_button", 298)), 
		299 => Some(Item::new("minecraft", "acacia_button", 299)), 
		300 => Some(Item::new("minecraft", "dark_oak_button", 300)), 
		301 => Some(Item::new("minecraft", "crimson_button", 301)), 
		302 => Some(Item::new("minecraft", "warped_button", 302)), 
		303 => Some(Item::new("minecraft", "anvil", 303)), 
		304 => Some(Item::new("minecraft", "chipped_anvil", 304)), 
		305 => Some(Item::new("minecraft", "damaged_anvil", 305)), 
		306 => Some(Item::new("minecraft", "trapped_chest", 306)), 
		307 => Some(Item::new("minecraft", "light_weighted_pressure_plate", 307)), 
		308 => Some(Item::new("minecraft", "heavy_weighted_pressure_plate", 308)), 
		309 => Some(Item::new("minecraft", "daylight_detector", 309)), 
		310 => Some(Item::new("minecraft", "redstone_block", 310)), 
		311 => Some(Item::new("minecraft", "nether_quartz_ore", 311)), 
		312 => Some(Item::new("minecraft", "hopper", 312)), 
		313 => Some(Item::new("minecraft", "chiseled_quartz_block", 313)), 
		314 => Some(Item::new("minecraft", "quartz_block", 314)), 
		315 => Some(Item::new("minecraft", "quartz_pillar", 315)), 
		316 => Some(Item::new("minecraft", "quartz_stairs", 316)), 
		317 => Some(Item::new("minecraft", "activator_rail", 317)), 
		318 => Some(Item::new("minecraft", "dropper", 318)), 
		319 => Some(Item::new("minecraft", "white_terracotta", 319)), 
		320 => Some(Item::new("minecraft", "orange_terracotta", 320)), 
		321 => Some(Item::new("minecraft", "magenta_terracotta", 321)), 
		322 => Some(Item::new("minecraft", "light_blue_terracotta", 322)), 
		323 => Some(Item::new("minecraft", "yellow_terracotta", 323)), 
		324 => Some(Item::new("minecraft", "lime_terracotta", 324)), 
		325 => Some(Item::new("minecraft", "pink_terracotta", 325)), 
		326 => Some(Item::new("minecraft", "gray_terracotta", 326)), 
		327 => Some(Item::new("minecraft", "light_gray_terracotta", 327)), 
		328 => Some(Item::new("minecraft", "cyan_terracotta", 328)), 
		329 => Some(Item::new("minecraft", "purple_terracotta", 329)), 
		330 => Some(Item::new("minecraft", "blue_terracotta", 330)), 
		331 => Some(Item::new("minecraft", "brown_terracotta", 331)), 
		332 => Some(Item::new("minecraft", "green_terracotta", 332)), 
		333 => Some(Item::new("minecraft", "red_terracotta", 333)), 
		334 => Some(Item::new("minecraft", "black_terracotta", 334)), 
		335 => Some(Item::new("minecraft", "barrier", 335)), 
		336 => Some(Item::new("minecraft", "iron_trapdoor", 336)), 
		337 => Some(Item::new("minecraft", "hay_block", 337)), 
		338 => Some(Item::new("minecraft", "white_carpet", 338)), 
		339 => Some(Item::new("minecraft", "orange_carpet", 339)), 
		340 => Some(Item::new("minecraft", "magenta_carpet", 340)), 
		341 => Some(Item::new("minecraft", "light_blue_carpet", 341)), 
		342 => Some(Item::new("minecraft", "yellow_carpet", 342)), 
		343 => Some(Item::new("minecraft", "lime_carpet", 343)), 
		344 => Some(Item::new("minecraft", "pink_carpet", 344)), 
		345 => Some(Item::new("minecraft", "gray_carpet", 345)), 
		346 => Some(Item::new("minecraft", "light_gray_carpet", 346)), 
		347 => Some(Item::new("minecraft", "cyan_carpet", 347)), 
		348 => Some(Item::new("minecraft", "purple_carpet", 348)), 
		349 => Some(Item::new("minecraft", "blue_carpet", 349)), 
		350 => Some(Item::new("minecraft", "brown_carpet", 350)), 
		351 => Some(Item::new("minecraft", "green_carpet", 351)), 
		352 => Some(Item::new("minecraft", "red_carpet", 352)), 
		353 => Some(Item::new("minecraft", "black_carpet", 353)), 
		354 => Some(Item::new("minecraft", "terracotta", 354)), 
		355 => Some(Item::new("minecraft", "coal_block", 355)), 
		356 => Some(Item::new("minecraft", "packed_ice", 356)), 
		357 => Some(Item::new("minecraft", "acacia_stairs", 357)), 
		358 => Some(Item::new("minecraft", "dark_oak_stairs", 358)), 
		359 => Some(Item::new("minecraft", "slime_block", 359)), 
		360 => Some(Item::new("minecraft", "grass_path", 360)), 
		361 => Some(Item::new("minecraft", "sunflower", 361)), 
		362 => Some(Item::new("minecraft", "lilac", 362)), 
		363 => Some(Item::new("minecraft", "rose_bush", 363)), 
		364 => Some(Item::new("minecraft", "peony", 364)), 
		365 => Some(Item::new("minecraft", "tall_grass", 365)), 
		366 => Some(Item::new("minecraft", "large_fern", 366)), 
		367 => Some(Item::new("minecraft", "white_stained_glass", 367)), 
		368 => Some(Item::new("minecraft", "orange_stained_glass", 368)), 
		369 => Some(Item::new("minecraft", "magenta_stained_glass", 369)), 
		370 => Some(Item::new("minecraft", "light_blue_stained_glass", 370)), 
		371 => Some(Item::new("minecraft", "yellow_stained_glass", 371)), 
		372 => Some(Item::new("minecraft", "lime_stained_glass", 372)), 
		373 => Some(Item::new("minecraft", "pink_stained_glass", 373)), 
		374 => Some(Item::new("minecraft", "gray_stained_glass", 374)), 
		375 => Some(Item::new("minecraft", "light_gray_stained_glass", 375)), 
		376 => Some(Item::new("minecraft", "cyan_stained_glass", 376)), 
		377 => Some(Item::new("minecraft", "purple_stained_glass", 377)), 
		378 => Some(Item::new("minecraft", "blue_stained_glass", 378)), 
		379 => Some(Item::new("minecraft", "brown_stained_glass", 379)), 
		380 => Some(Item::new("minecraft", "green_stained_glass", 380)), 
		381 => Some(Item::new("minecraft", "red_stained_glass", 381)), 
		382 => Some(Item::new("minecraft", "black_stained_glass", 382)), 
		383 => Some(Item::new("minecraft", "white_stained_glass_pane", 383)), 
		384 => Some(Item::new("minecraft", "orange_stained_glass_pane", 384)), 
		385 => Some(Item::new("minecraft", "magenta_stained_glass_pane", 385)), 
		386 => Some(Item::new("minecraft", "light_blue_stained_glass_pane", 386)), 
		387 => Some(Item::new("minecraft", "yellow_stained_glass_pane", 387)), 
		388 => Some(Item::new("minecraft", "lime_stained_glass_pane", 388)), 
		389 => Some(Item::new("minecraft", "pink_stained_glass_pane", 389)), 
		390 => Some(Item::new("minecraft", "gray_stained_glass_pane", 390)), 
		391 => Some(Item::new("minecraft", "light_gray_stained_glass_pane", 391)), 
		392 => Some(Item::new("minecraft", "cyan_stained_glass_pane", 392)), 
		393 => Some(Item::new("minecraft", "purple_stained_glass_pane", 393)), 
		394 => Some(Item::new("minecraft", "blue_stained_glass_pane", 394)), 
		395 => Some(Item::new("minecraft", "brown_stained_glass_pane", 395)), 
		396 => Some(Item::new("minecraft", "green_stained_glass_pane", 396)), 
		397 => Some(Item::new("minecraft", "red_stained_glass_pane", 397)), 
		398 => Some(Item::new("minecraft", "black_stained_glass_pane", 398)), 
		399 => Some(Item::new("minecraft", "prismarine", 399)), 
		400 => Some(Item::new("minecraft", "prismarine_bricks", 400)), 
		401 => Some(Item::new("minecraft", "dark_prismarine", 401)), 
		402 => Some(Item::new("minecraft", "prismarine_stairs", 402)), 
		403 => Some(Item::new("minecraft", "prismarine_brick_stairs", 403)), 
		404 => Some(Item::new("minecraft", "dark_prismarine_stairs", 404)), 
		405 => Some(Item::new("minecraft", "sea_lantern", 405)), 
		406 => Some(Item::new("minecraft", "red_sandstone", 406)), 
		407 => Some(Item::new("minecraft", "chiseled_red_sandstone", 407)), 
		408 => Some(Item::new("minecraft", "cut_red_sandstone", 408)), 
		409 => Some(Item::new("minecraft", "red_sandstone_stairs", 409)), 
		410 => Some(Item::new("minecraft", "repeating_command_block", 410)), 
		411 => Some(Item::new("minecraft", "chain_command_block", 411)), 
		412 => Some(Item::new("minecraft", "magma_block", 412)), 
		413 => Some(Item::new("minecraft", "nether_wart_block", 413)), 
		414 => Some(Item::new("minecraft", "warped_wart_block", 414)), 
		415 => Some(Item::new("minecraft", "red_nether_bricks", 415)), 
		416 => Some(Item::new("minecraft", "bone_block", 416)), 
		417 => Some(Item::new("minecraft", "structure_void", 417)), 
		418 => Some(Item::new("minecraft", "observer", 418)), 
		419 => Some(Item::new("minecraft", "shulker_box", 419)), 
		420 => Some(Item::new("minecraft", "white_shulker_box", 420)), 
		421 => Some(Item::new("minecraft", "orange_shulker_box", 421)), 
		422 => Some(Item::new("minecraft", "magenta_shulker_box", 422)), 
		423 => Some(Item::new("minecraft", "light_blue_shulker_box", 423)), 
		424 => Some(Item::new("minecraft", "yellow_shulker_box", 424)), 
		425 => Some(Item::new("minecraft", "lime_shulker_box", 425)), 
		426 => Some(Item::new("minecraft", "pink_shulker_box", 426)), 
		427 => Some(Item::new("minecraft", "gray_shulker_box", 427)), 
		428 => Some(Item::new("minecraft", "light_gray_shulker_box", 428)), 
		429 => Some(Item::new("minecraft", "cyan_shulker_box", 429)), 
		430 => Some(Item::new("minecraft", "purple_shulker_box", 430)), 
		431 => Some(Item::new("minecraft", "blue_shulker_box", 431)), 
		432 => Some(Item::new("minecraft", "brown_shulker_box", 432)), 
		433 => Some(Item::new("minecraft", "green_shulker_box", 433)), 
		434 => Some(Item::new("minecraft", "red_shulker_box", 434)), 
		435 => Some(Item::new("minecraft", "black_shulker_box", 435)), 
		436 => Some(Item::new("minecraft", "white_glazed_terracotta", 436)), 
		437 => Some(Item::new("minecraft", "orange_glazed_terracotta", 437)), 
		438 => Some(Item::new("minecraft", "magenta_glazed_terracotta", 438)), 
		439 => Some(Item::new("minecraft", "light_blue_glazed_terracotta", 439)), 
		440 => Some(Item::new("minecraft", "yellow_glazed_terracotta", 440)), 
		441 => Some(Item::new("minecraft", "lime_glazed_terracotta", 441)), 
		442 => Some(Item::new("minecraft", "pink_glazed_terracotta", 442)), 
		443 => Some(Item::new("minecraft", "gray_glazed_terracotta", 443)), 
		444 => Some(Item::new("minecraft", "light_gray_glazed_terracotta", 444)), 
		445 => Some(Item::new("minecraft", "cyan_glazed_terracotta", 445)), 
		446 => Some(Item::new("minecraft", "purple_glazed_terracotta", 446)), 
		447 => Some(Item::new("minecraft", "blue_glazed_terracotta", 447)), 
		448 => Some(Item::new("minecraft", "brown_glazed_terracotta", 448)), 
		449 => Some(Item::new("minecraft", "green_glazed_terracotta", 449)), 
		450 => Some(Item::new("minecraft", "red_glazed_terracotta", 450)), 
		451 => Some(Item::new("minecraft", "black_glazed_terracotta", 451)), 
		452 => Some(Item::new("minecraft", "white_concrete", 452)), 
		453 => Some(Item::new("minecraft", "orange_concrete", 453)), 
		454 => Some(Item::new("minecraft", "magenta_concrete", 454)), 
		455 => Some(Item::new("minecraft", "light_blue_concrete", 455)), 
		456 => Some(Item::new("minecraft", "yellow_concrete", 456)), 
		457 => Some(Item::new("minecraft", "lime_concrete", 457)), 
		458 => Some(Item::new("minecraft", "pink_concrete", 458)), 
		459 => Some(Item::new("minecraft", "gray_concrete", 459)), 
		460 => Some(Item::new("minecraft", "light_gray_concrete", 460)), 
		461 => Some(Item::new("minecraft", "cyan_concrete", 461)), 
		462 => Some(Item::new("minecraft", "purple_concrete", 462)), 
		463 => Some(Item::new("minecraft", "blue_concrete", 463)), 
		464 => Some(Item::new("minecraft", "brown_concrete", 464)), 
		465 => Some(Item::new("minecraft", "green_concrete", 465)), 
		466 => Some(Item::new("minecraft", "red_concrete", 466)), 
		467 => Some(Item::new("minecraft", "black_concrete", 467)), 
		468 => Some(Item::new("minecraft", "white_concrete_powder", 468)), 
		469 => Some(Item::new("minecraft", "orange_concrete_powder", 469)), 
		470 => Some(Item::new("minecraft", "magenta_concrete_powder", 470)), 
		471 => Some(Item::new("minecraft", "light_blue_concrete_powder", 471)), 
		472 => Some(Item::new("minecraft", "yellow_concrete_powder", 472)), 
		473 => Some(Item::new("minecraft", "lime_concrete_powder", 473)), 
		474 => Some(Item::new("minecraft", "pink_concrete_powder", 474)), 
		475 => Some(Item::new("minecraft", "gray_concrete_powder", 475)), 
		476 => Some(Item::new("minecraft", "light_gray_concrete_powder", 476)), 
		477 => Some(Item::new("minecraft", "cyan_concrete_powder", 477)), 
		478 => Some(Item::new("minecraft", "purple_concrete_powder", 478)), 
		479 => Some(Item::new("minecraft", "blue_concrete_powder", 479)), 
		480 => Some(Item::new("minecraft", "brown_concrete_powder", 480)), 
		481 => Some(Item::new("minecraft", "green_concrete_powder", 481)), 
		482 => Some(Item::new("minecraft", "red_concrete_powder", 482)), 
		483 => Some(Item::new("minecraft", "black_concrete_powder", 483)), 
		484 => Some(Item::new("minecraft", "turtle_egg", 484)), 
		485 => Some(Item::new("minecraft", "dead_tube_coral_block", 485)), 
		486 => Some(Item::new("minecraft", "dead_brain_coral_block", 486)), 
		487 => Some(Item::new("minecraft", "dead_bubble_coral_block", 487)), 
		488 => Some(Item::new("minecraft", "dead_fire_coral_block", 488)), 
		489 => Some(Item::new("minecraft", "dead_horn_coral_block", 489)), 
		490 => Some(Item::new("minecraft", "tube_coral_block", 490)), 
		491 => Some(Item::new("minecraft", "brain_coral_block", 491)), 
		492 => Some(Item::new("minecraft", "bubble_coral_block", 492)), 
		493 => Some(Item::new("minecraft", "fire_coral_block", 493)), 
		494 => Some(Item::new("minecraft", "horn_coral_block", 494)), 
		495 => Some(Item::new("minecraft", "tube_coral", 495)), 
		496 => Some(Item::new("minecraft", "brain_coral", 496)), 
		497 => Some(Item::new("minecraft", "bubble_coral", 497)), 
		498 => Some(Item::new("minecraft", "fire_coral", 498)), 
		499 => Some(Item::new("minecraft", "horn_coral", 499)), 
		500 => Some(Item::new("minecraft", "dead_brain_coral", 500)), 
		501 => Some(Item::new("minecraft", "dead_bubble_coral", 501)), 
		502 => Some(Item::new("minecraft", "dead_fire_coral", 502)), 
		503 => Some(Item::new("minecraft", "dead_horn_coral", 503)), 
		504 => Some(Item::new("minecraft", "dead_tube_coral", 504)), 
		505 => Some(Item::new("minecraft", "tube_coral_fan", 505)), 
		506 => Some(Item::new("minecraft", "brain_coral_fan", 506)), 
		507 => Some(Item::new("minecraft", "bubble_coral_fan", 507)), 
		508 => Some(Item::new("minecraft", "fire_coral_fan", 508)), 
		509 => Some(Item::new("minecraft", "horn_coral_fan", 509)), 
		510 => Some(Item::new("minecraft", "dead_tube_coral_fan", 510)), 
		511 => Some(Item::new("minecraft", "dead_brain_coral_fan", 511)), 
		512 => Some(Item::new("minecraft", "dead_bubble_coral_fan", 512)), 
		513 => Some(Item::new("minecraft", "dead_fire_coral_fan", 513)), 
		514 => Some(Item::new("minecraft", "dead_horn_coral_fan", 514)), 
		515 => Some(Item::new("minecraft", "blue_ice", 515)), 
		516 => Some(Item::new("minecraft", "conduit", 516)), 
		517 => Some(Item::new("minecraft", "polished_granite_stairs", 517)), 
		518 => Some(Item::new("minecraft", "smooth_red_sandstone_stairs", 518)), 
		519 => Some(Item::new("minecraft", "mossy_stone_brick_stairs", 519)), 
		520 => Some(Item::new("minecraft", "polished_diorite_stairs", 520)), 
		521 => Some(Item::new("minecraft", "mossy_cobblestone_stairs", 521)), 
		522 => Some(Item::new("minecraft", "end_stone_brick_stairs", 522)), 
		523 => Some(Item::new("minecraft", "stone_stairs", 523)), 
		524 => Some(Item::new("minecraft", "smooth_sandstone_stairs", 524)), 
		525 => Some(Item::new("minecraft", "smooth_quartz_stairs", 525)), 
		526 => Some(Item::new("minecraft", "granite_stairs", 526)), 
		527 => Some(Item::new("minecraft", "andesite_stairs", 527)), 
		528 => Some(Item::new("minecraft", "red_nether_brick_stairs", 528)), 
		529 => Some(Item::new("minecraft", "polished_andesite_stairs", 529)), 
		530 => Some(Item::new("minecraft", "diorite_stairs", 530)), 
		531 => Some(Item::new("minecraft", "polished_granite_slab", 531)), 
		532 => Some(Item::new("minecraft", "smooth_red_sandstone_slab", 532)), 
		533 => Some(Item::new("minecraft", "mossy_stone_brick_slab", 533)), 
		534 => Some(Item::new("minecraft", "polished_diorite_slab", 534)), 
		535 => Some(Item::new("minecraft", "mossy_cobblestone_slab", 535)), 
		536 => Some(Item::new("minecraft", "end_stone_brick_slab", 536)), 
		537 => Some(Item::new("minecraft", "smooth_sandstone_slab", 537)), 
		538 => Some(Item::new("minecraft", "smooth_quartz_slab", 538)), 
		539 => Some(Item::new("minecraft", "granite_slab", 539)), 
		540 => Some(Item::new("minecraft", "andesite_slab", 540)), 
		541 => Some(Item::new("minecraft", "red_nether_brick_slab", 541)), 
		542 => Some(Item::new("minecraft", "polished_andesite_slab", 542)), 
		543 => Some(Item::new("minecraft", "diorite_slab", 543)), 
		544 => Some(Item::new("minecraft", "scaffolding", 544)), 
		545 => Some(Item::new("minecraft", "iron_door", 545)), 
		546 => Some(Item::new("minecraft", "oak_door", 546)), 
		547 => Some(Item::new("minecraft", "spruce_door", 547)), 
		548 => Some(Item::new("minecraft", "birch_door", 548)), 
		549 => Some(Item::new("minecraft", "jungle_door", 549)), 
		550 => Some(Item::new("minecraft", "acacia_door", 550)), 
		551 => Some(Item::new("minecraft", "dark_oak_door", 551)), 
		552 => Some(Item::new("minecraft", "crimson_door", 552)), 
		553 => Some(Item::new("minecraft", "warped_door", 553)), 
		554 => Some(Item::new("minecraft", "repeater", 554)), 
		555 => Some(Item::new("minecraft", "comparator", 555)), 
		556 => Some(Item::new("minecraft", "structure_block", 556)), 
		557 => Some(Item::new("minecraft", "jigsaw", 557)), 
		558 => Some(Item::new("minecraft", "turtle_helmet", 558)), 
		559 => Some(Item::new("minecraft", "scute", 559)), 
		560 => Some(Item::new("minecraft", "iron_shovel", 560)), 
		561 => Some(Item::new("minecraft", "iron_pickaxe", 561)), 
		562 => Some(Item::new("minecraft", "iron_axe", 562)), 
		563 => Some(Item::new("minecraft", "flint_and_steel", 563)), 
		564 => Some(Item::new("minecraft", "apple", 564)), 
		565 => Some(Item::new("minecraft", "bow", 565)), 
		566 => Some(Item::new("minecraft", "arrow", 566)), 
		567 => Some(Item::new("minecraft", "coal", 567)), 
		568 => Some(Item::new("minecraft", "charcoal", 568)), 
		569 => Some(Item::new("minecraft", "diamond", 569)), 
		570 => Some(Item::new("minecraft", "iron_ingot", 570)), 
		571 => Some(Item::new("minecraft", "gold_ingot", 571)), 
		572 => Some(Item::new("minecraft", "iron_sword", 572)), 
		573 => Some(Item::new("minecraft", "wooden_sword", 573)), 
		574 => Some(Item::new("minecraft", "wooden_shovel", 574)), 
		575 => Some(Item::new("minecraft", "wooden_pickaxe", 575)), 
		576 => Some(Item::new("minecraft", "wooden_axe", 576)), 
		577 => Some(Item::new("minecraft", "stone_sword", 577)), 
		578 => Some(Item::new("minecraft", "stone_shovel", 578)), 
		579 => Some(Item::new("minecraft", "stone_pickaxe", 579)), 
		580 => Some(Item::new("minecraft", "stone_axe", 580)), 
		581 => Some(Item::new("minecraft", "diamond_sword", 581)), 
		582 => Some(Item::new("minecraft", "diamond_shovel", 582)), 
		583 => Some(Item::new("minecraft", "diamond_pickaxe", 583)), 
		584 => Some(Item::new("minecraft", "diamond_axe", 584)), 
		585 => Some(Item::new("minecraft", "stick", 585)), 
		586 => Some(Item::new("minecraft", "bowl", 586)), 
		587 => Some(Item::new("minecraft", "mushroom_stew", 587)), 
		588 => Some(Item::new("minecraft", "golden_sword", 588)), 
		589 => Some(Item::new("minecraft", "golden_shovel", 589)), 
		590 => Some(Item::new("minecraft", "golden_pickaxe", 590)), 
		591 => Some(Item::new("minecraft", "golden_axe", 591)), 
		592 => Some(Item::new("minecraft", "netherite_sword", 592)), 
		593 => Some(Item::new("minecraft", "netherite_shovel", 593)), 
		594 => Some(Item::new("minecraft", "netherite_pickaxe", 594)), 
		595 => Some(Item::new("minecraft", "netherite_axe", 595)), 
		596 => Some(Item::new("minecraft", "string", 596)), 
		597 => Some(Item::new("minecraft", "feather", 597)), 
		598 => Some(Item::new("minecraft", "gunpowder", 598)), 
		599 => Some(Item::new("minecraft", "wooden_hoe", 599)), 
		600 => Some(Item::new("minecraft", "stone_hoe", 600)), 
		601 => Some(Item::new("minecraft", "iron_hoe", 601)), 
		602 => Some(Item::new("minecraft", "diamond_hoe", 602)), 
		603 => Some(Item::new("minecraft", "golden_hoe", 603)), 
		604 => Some(Item::new("minecraft", "netherite_hoe", 604)), 
		605 => Some(Item::new("minecraft", "wheat_seeds", 605)), 
		606 => Some(Item::new("minecraft", "wheat", 606)), 
		607 => Some(Item::new("minecraft", "bread", 607)), 
		608 => Some(Item::new("minecraft", "leather_helmet", 608)), 
		609 => Some(Item::new("minecraft", "leather_chestplate", 609)), 
		610 => Some(Item::new("minecraft", "leather_leggings", 610)), 
		611 => Some(Item::new("minecraft", "leather_boots", 611)), 
		612 => Some(Item::new("minecraft", "chainmail_helmet", 612)), 
		613 => Some(Item::new("minecraft", "chainmail_chestplate", 613)), 
		614 => Some(Item::new("minecraft", "chainmail_leggings", 614)), 
		615 => Some(Item::new("minecraft", "chainmail_boots", 615)), 
		616 => Some(Item::new("minecraft", "iron_helmet", 616)), 
		617 => Some(Item::new("minecraft", "iron_chestplate", 617)), 
		618 => Some(Item::new("minecraft", "iron_leggings", 618)), 
		619 => Some(Item::new("minecraft", "iron_boots", 619)), 
		620 => Some(Item::new("minecraft", "diamond_helmet", 620)), 
		621 => Some(Item::new("minecraft", "diamond_chestplate", 621)), 
		622 => Some(Item::new("minecraft", "diamond_leggings", 622)), 
		623 => Some(Item::new("minecraft", "diamond_boots", 623)), 
		624 => Some(Item::new("minecraft", "golden_helmet", 624)), 
		625 => Some(Item::new("minecraft", "golden_chestplate", 625)), 
		626 => Some(Item::new("minecraft", "golden_leggings", 626)), 
		627 => Some(Item::new("minecraft", "golden_boots", 627)), 
		628 => Some(Item::new("minecraft", "netherite_helmet", 628)), 
		629 => Some(Item::new("minecraft", "netherite_chestplate", 629)), 
		630 => Some(Item::new("minecraft", "netherite_leggings", 630)), 
		631 => Some(Item::new("minecraft", "netherite_boots", 631)), 
		632 => Some(Item::new("minecraft", "flint", 632)), 
		633 => Some(Item::new("minecraft", "porkchop", 633)), 
		634 => Some(Item::new("minecraft", "cooked_porkchop", 634)), 
		635 => Some(Item::new("minecraft", "painting", 635)), 
		636 => Some(Item::new("minecraft", "golden_apple", 636)), 
		637 => Some(Item::new("minecraft", "enchanted_golden_apple", 637)), 
		638 => Some(Item::new("minecraft", "oak_sign", 638)), 
		639 => Some(Item::new("minecraft", "spruce_sign", 639)), 
		640 => Some(Item::new("minecraft", "birch_sign", 640)), 
		641 => Some(Item::new("minecraft", "jungle_sign", 641)), 
		642 => Some(Item::new("minecraft", "acacia_sign", 642)), 
		643 => Some(Item::new("minecraft", "dark_oak_sign", 643)), 
		644 => Some(Item::new("minecraft", "crimson_sign", 644)), 
		645 => Some(Item::new("minecraft", "warped_sign", 645)), 
		646 => Some(Item::new("minecraft", "bucket", 646)), 
		647 => Some(Item::new("minecraft", "water_bucket", 647)), 
		648 => Some(Item::new("minecraft", "lava_bucket", 648)), 
		649 => Some(Item::new("minecraft", "minecart", 649)), 
		650 => Some(Item::new("minecraft", "saddle", 650)), 
		651 => Some(Item::new("minecraft", "redstone", 651)), 
		652 => Some(Item::new("minecraft", "snowball", 652)), 
		653 => Some(Item::new("minecraft", "oak_boat", 653)), 
		654 => Some(Item::new("minecraft", "leather", 654)), 
		655 => Some(Item::new("minecraft", "milk_bucket", 655)), 
		656 => Some(Item::new("minecraft", "pufferfish_bucket", 656)), 
		657 => Some(Item::new("minecraft", "salmon_bucket", 657)), 
		658 => Some(Item::new("minecraft", "cod_bucket", 658)), 
		659 => Some(Item::new("minecraft", "tropical_fish_bucket", 659)), 
		660 => Some(Item::new("minecraft", "brick", 660)), 
		661 => Some(Item::new("minecraft", "clay_ball", 661)), 
		662 => Some(Item::new("minecraft", "sugar_cane", 662)), 
		663 => Some(Item::new("minecraft", "kelp", 663)), 
		664 => Some(Item::new("minecraft", "dried_kelp_block", 664)), 
		665 => Some(Item::new("minecraft", "bamboo", 665)), 
		666 => Some(Item::new("minecraft", "paper", 666)), 
		667 => Some(Item::new("minecraft", "book", 667)), 
		668 => Some(Item::new("minecraft", "slime_ball", 668)), 
		669 => Some(Item::new("minecraft", "chest_minecart", 669)), 
		670 => Some(Item::new("minecraft", "furnace_minecart", 670)), 
		671 => Some(Item::new("minecraft", "egg", 671)), 
		672 => Some(Item::new("minecraft", "compass", 672)), 
		673 => Some(Item::new("minecraft", "fishing_rod", 673)), 
		674 => Some(Item::new("minecraft", "clock", 674)), 
		675 => Some(Item::new("minecraft", "glowstone_dust", 675)), 
		676 => Some(Item::new("minecraft", "cod", 676)), 
		677 => Some(Item::new("minecraft", "salmon", 677)), 
		678 => Some(Item::new("minecraft", "tropical_fish", 678)), 
		679 => Some(Item::new("minecraft", "pufferfish", 679)), 
		680 => Some(Item::new("minecraft", "cooked_cod", 680)), 
		681 => Some(Item::new("minecraft", "cooked_salmon", 681)), 
		682 => Some(Item::new("minecraft", "ink_sac", 682)), 
		683 => Some(Item::new("minecraft", "red_dye", 683)), 
		684 => Some(Item::new("minecraft", "green_dye", 684)), 
		685 => Some(Item::new("minecraft", "cocoa_beans", 685)), 
		686 => Some(Item::new("minecraft", "lapis_lazuli", 686)), 
		687 => Some(Item::new("minecraft", "purple_dye", 687)), 
		688 => Some(Item::new("minecraft", "cyan_dye", 688)), 
		689 => Some(Item::new("minecraft", "light_gray_dye", 689)), 
		690 => Some(Item::new("minecraft", "gray_dye", 690)), 
		691 => Some(Item::new("minecraft", "pink_dye", 691)), 
		692 => Some(Item::new("minecraft", "lime_dye", 692)), 
		693 => Some(Item::new("minecraft", "yellow_dye", 693)), 
		694 => Some(Item::new("minecraft", "light_blue_dye", 694)), 
		695 => Some(Item::new("minecraft", "magenta_dye", 695)), 
		696 => Some(Item::new("minecraft", "orange_dye", 696)), 
		697 => Some(Item::new("minecraft", "bone_meal", 697)), 
		698 => Some(Item::new("minecraft", "blue_dye", 698)), 
		699 => Some(Item::new("minecraft", "brown_dye", 699)), 
		700 => Some(Item::new("minecraft", "black_dye", 700)), 
		701 => Some(Item::new("minecraft", "white_dye", 701)), 
		702 => Some(Item::new("minecraft", "bone", 702)), 
		703 => Some(Item::new("minecraft", "sugar", 703)), 
		704 => Some(Item::new("minecraft", "cake", 704)), 
		705 => Some(Item::new("minecraft", "white_bed", 705)), 
		706 => Some(Item::new("minecraft", "orange_bed", 706)), 
		707 => Some(Item::new("minecraft", "magenta_bed", 707)), 
		708 => Some(Item::new("minecraft", "light_blue_bed", 708)), 
		709 => Some(Item::new("minecraft", "yellow_bed", 709)), 
		710 => Some(Item::new("minecraft", "lime_bed", 710)), 
		711 => Some(Item::new("minecraft", "pink_bed", 711)), 
		712 => Some(Item::new("minecraft", "gray_bed", 712)), 
		713 => Some(Item::new("minecraft", "light_gray_bed", 713)), 
		714 => Some(Item::new("minecraft", "cyan_bed", 714)), 
		715 => Some(Item::new("minecraft", "purple_bed", 715)), 
		716 => Some(Item::new("minecraft", "blue_bed", 716)), 
		717 => Some(Item::new("minecraft", "brown_bed", 717)), 
		718 => Some(Item::new("minecraft", "green_bed", 718)), 
		719 => Some(Item::new("minecraft", "red_bed", 719)), 
		720 => Some(Item::new("minecraft", "black_bed", 720)), 
		721 => Some(Item::new("minecraft", "cookie", 721)), 
		722 => Some(Item::new("minecraft", "filled_map", 722)), 
		723 => Some(Item::new("minecraft", "shears", 723)), 
		724 => Some(Item::new("minecraft", "melon_slice", 724)), 
		725 => Some(Item::new("minecraft", "dried_kelp", 725)), 
		726 => Some(Item::new("minecraft", "pumpkin_seeds", 726)), 
		727 => Some(Item::new("minecraft", "melon_seeds", 727)), 
		728 => Some(Item::new("minecraft", "beef", 728)), 
		729 => Some(Item::new("minecraft", "cooked_beef", 729)), 
		730 => Some(Item::new("minecraft", "chicken", 730)), 
		731 => Some(Item::new("minecraft", "cooked_chicken", 731)), 
		732 => Some(Item::new("minecraft", "rotten_flesh", 732)), 
		733 => Some(Item::new("minecraft", "ender_pearl", 733)), 
		734 => Some(Item::new("minecraft", "blaze_rod", 734)), 
		735 => Some(Item::new("minecraft", "ghast_tear", 735)), 
		736 => Some(Item::new("minecraft", "gold_nugget", 736)), 
		737 => Some(Item::new("minecraft", "nether_wart", 737)), 
		738 => Some(Item::new("minecraft", "potion", 738)), 
		739 => Some(Item::new("minecraft", "glass_bottle", 739)), 
		740 => Some(Item::new("minecraft", "spider_eye", 740)), 
		741 => Some(Item::new("minecraft", "fermented_spider_eye", 741)), 
		742 => Some(Item::new("minecraft", "blaze_powder", 742)), 
		743 => Some(Item::new("minecraft", "magma_cream", 743)), 
		744 => Some(Item::new("minecraft", "brewing_stand", 744)), 
		745 => Some(Item::new("minecraft", "cauldron", 745)), 
		746 => Some(Item::new("minecraft", "ender_eye", 746)), 
		747 => Some(Item::new("minecraft", "glistering_melon_slice", 747)), 
		748 => Some(Item::new("minecraft", "bat_spawn_egg", 748)), 
		749 => Some(Item::new("minecraft", "bee_spawn_egg", 749)), 
		750 => Some(Item::new("minecraft", "blaze_spawn_egg", 750)), 
		751 => Some(Item::new("minecraft", "cat_spawn_egg", 751)), 
		752 => Some(Item::new("minecraft", "cave_spider_spawn_egg", 752)), 
		753 => Some(Item::new("minecraft", "chicken_spawn_egg", 753)), 
		754 => Some(Item::new("minecraft", "cod_spawn_egg", 754)), 
		755 => Some(Item::new("minecraft", "cow_spawn_egg", 755)), 
		756 => Some(Item::new("minecraft", "creeper_spawn_egg", 756)), 
		757 => Some(Item::new("minecraft", "dolphin_spawn_egg", 757)), 
		758 => Some(Item::new("minecraft", "donkey_spawn_egg", 758)), 
		759 => Some(Item::new("minecraft", "drowned_spawn_egg", 759)), 
		760 => Some(Item::new("minecraft", "elder_guardian_spawn_egg", 760)), 
		761 => Some(Item::new("minecraft", "enderman_spawn_egg", 761)), 
		762 => Some(Item::new("minecraft", "endermite_spawn_egg", 762)), 
		763 => Some(Item::new("minecraft", "evoker_spawn_egg", 763)), 
		764 => Some(Item::new("minecraft", "fox_spawn_egg", 764)), 
		765 => Some(Item::new("minecraft", "ghast_spawn_egg", 765)), 
		766 => Some(Item::new("minecraft", "guardian_spawn_egg", 766)), 
		767 => Some(Item::new("minecraft", "hoglin_spawn_egg", 767)), 
		768 => Some(Item::new("minecraft", "horse_spawn_egg", 768)), 
		769 => Some(Item::new("minecraft", "husk_spawn_egg", 769)), 
		770 => Some(Item::new("minecraft", "llama_spawn_egg", 770)), 
		771 => Some(Item::new("minecraft", "magma_cube_spawn_egg", 771)), 
		772 => Some(Item::new("minecraft", "mooshroom_spawn_egg", 772)), 
		773 => Some(Item::new("minecraft", "mule_spawn_egg", 773)), 
		774 => Some(Item::new("minecraft", "ocelot_spawn_egg", 774)), 
		775 => Some(Item::new("minecraft", "panda_spawn_egg", 775)), 
		776 => Some(Item::new("minecraft", "parrot_spawn_egg", 776)), 
		777 => Some(Item::new("minecraft", "phantom_spawn_egg", 777)), 
		778 => Some(Item::new("minecraft", "pig_spawn_egg", 778)), 
		779 => Some(Item::new("minecraft", "piglin_spawn_egg", 779)), 
		780 => Some(Item::new("minecraft", "pillager_spawn_egg", 780)), 
		781 => Some(Item::new("minecraft", "polar_bear_spawn_egg", 781)), 
		782 => Some(Item::new("minecraft", "pufferfish_spawn_egg", 782)), 
		783 => Some(Item::new("minecraft", "rabbit_spawn_egg", 783)), 
		784 => Some(Item::new("minecraft", "ravager_spawn_egg", 784)), 
		785 => Some(Item::new("minecraft", "salmon_spawn_egg", 785)), 
		786 => Some(Item::new("minecraft", "sheep_spawn_egg", 786)), 
		787 => Some(Item::new("minecraft", "shulker_spawn_egg", 787)), 
		788 => Some(Item::new("minecraft", "silverfish_spawn_egg", 788)), 
		789 => Some(Item::new("minecraft", "skeleton_spawn_egg", 789)), 
		790 => Some(Item::new("minecraft", "skeleton_horse_spawn_egg", 790)), 
		791 => Some(Item::new("minecraft", "slime_spawn_egg", 791)), 
		792 => Some(Item::new("minecraft", "spider_spawn_egg", 792)), 
		793 => Some(Item::new("minecraft", "squid_spawn_egg", 793)), 
		794 => Some(Item::new("minecraft", "stray_spawn_egg", 794)), 
		795 => Some(Item::new("minecraft", "trader_llama_spawn_egg", 795)), 
		796 => Some(Item::new("minecraft", "tropical_fish_spawn_egg", 796)), 
		797 => Some(Item::new("minecraft", "turtle_spawn_egg", 797)), 
		798 => Some(Item::new("minecraft", "vex_spawn_egg", 798)), 
		799 => Some(Item::new("minecraft", "villager_spawn_egg", 799)), 
		800 => Some(Item::new("minecraft", "vindicator_spawn_egg", 800)), 
		801 => Some(Item::new("minecraft", "wandering_trader_spawn_egg", 801)), 
		802 => Some(Item::new("minecraft", "witch_spawn_egg", 802)), 
		803 => Some(Item::new("minecraft", "wither_skeleton_spawn_egg", 803)), 
		804 => Some(Item::new("minecraft", "wolf_spawn_egg", 804)), 
		805 => Some(Item::new("minecraft", "zombie_spawn_egg", 805)), 
		806 => Some(Item::new("minecraft", "zombie_horse_spawn_egg", 806)), 
		807 => Some(Item::new("minecraft", "zombified_piglin_spawn_egg", 807)), 
		808 => Some(Item::new("minecraft", "zombie_villager_spawn_egg", 808)), 
		809 => Some(Item::new("minecraft", "experience_bottle", 809)), 
		810 => Some(Item::new("minecraft", "fire_charge", 810)), 
		811 => Some(Item::new("minecraft", "writable_book", 811)), 
		812 => Some(Item::new("minecraft", "written_book", 812)), 
		813 => Some(Item::new("minecraft", "emerald", 813)), 
		814 => Some(Item::new("minecraft", "item_frame", 814)), 
		815 => Some(Item::new("minecraft", "flower_pot", 815)), 
		816 => Some(Item::new("minecraft", "carrot", 816)), 
		817 => Some(Item::new("minecraft", "potato", 817)), 
		818 => Some(Item::new("minecraft", "baked_potato", 818)), 
		819 => Some(Item::new("minecraft", "poisonous_potato", 819)), 
		820 => Some(Item::new("minecraft", "map", 820)), 
		821 => Some(Item::new("minecraft", "golden_carrot", 821)), 
		822 => Some(Item::new("minecraft", "skeleton_skull", 822)), 
		823 => Some(Item::new("minecraft", "wither_skeleton_skull", 823)), 
		824 => Some(Item::new("minecraft", "player_head", 824)), 
		825 => Some(Item::new("minecraft", "zombie_head", 825)), 
		826 => Some(Item::new("minecraft", "creeper_head", 826)), 
		827 => Some(Item::new("minecraft", "dragon_head", 827)), 
		828 => Some(Item::new("minecraft", "carrot_on_a_stick", 828)), 
		829 => Some(Item::new("minecraft", "nether_star", 829)), 
		830 => Some(Item::new("minecraft", "pumpkin_pie", 830)), 
		831 => Some(Item::new("minecraft", "firework_rocket", 831)), 
		832 => Some(Item::new("minecraft", "firework_star", 832)), 
		833 => Some(Item::new("minecraft", "enchanted_book", 833)), 
		834 => Some(Item::new("minecraft", "nether_brick", 834)), 
		835 => Some(Item::new("minecraft", "quartz", 835)), 
		836 => Some(Item::new("minecraft", "tnt_minecart", 836)), 
		837 => Some(Item::new("minecraft", "hopper_minecart", 837)), 
		838 => Some(Item::new("minecraft", "prismarine_shard", 838)), 
		839 => Some(Item::new("minecraft", "prismarine_crystals", 839)), 
		840 => Some(Item::new("minecraft", "rabbit", 840)), 
		841 => Some(Item::new("minecraft", "cooked_rabbit", 841)), 
		842 => Some(Item::new("minecraft", "rabbit_stew", 842)), 
		843 => Some(Item::new("minecraft", "rabbit_foot", 843)), 
		844 => Some(Item::new("minecraft", "rabbit_hide", 844)), 
		845 => Some(Item::new("minecraft", "armor_stand", 845)), 
		846 => Some(Item::new("minecraft", "iron_horse_armor", 846)), 
		847 => Some(Item::new("minecraft", "golden_horse_armor", 847)), 
		848 => Some(Item::new("minecraft", "diamond_horse_armor", 848)), 
		849 => Some(Item::new("minecraft", "leather_horse_armor", 849)), 
		850 => Some(Item::new("minecraft", "lead", 850)), 
		851 => Some(Item::new("minecraft", "name_tag", 851)), 
		852 => Some(Item::new("minecraft", "command_block_minecart", 852)), 
		853 => Some(Item::new("minecraft", "mutton", 853)), 
		854 => Some(Item::new("minecraft", "cooked_mutton", 854)), 
		855 => Some(Item::new("minecraft", "white_banner", 855)), 
		856 => Some(Item::new("minecraft", "orange_banner", 856)), 
		857 => Some(Item::new("minecraft", "magenta_banner", 857)), 
		858 => Some(Item::new("minecraft", "light_blue_banner", 858)), 
		859 => Some(Item::new("minecraft", "yellow_banner", 859)), 
		860 => Some(Item::new("minecraft", "lime_banner", 860)), 
		861 => Some(Item::new("minecraft", "pink_banner", 861)), 
		862 => Some(Item::new("minecraft", "gray_banner", 862)), 
		863 => Some(Item::new("minecraft", "light_gray_banner", 863)), 
		864 => Some(Item::new("minecraft", "cyan_banner", 864)), 
		865 => Some(Item::new("minecraft", "purple_banner", 865)), 
		866 => Some(Item::new("minecraft", "blue_banner", 866)), 
		867 => Some(Item::new("minecraft", "brown_banner", 867)), 
		868 => Some(Item::new("minecraft", "green_banner", 868)), 
		869 => Some(Item::new("minecraft", "red_banner", 869)), 
		870 => Some(Item::new("minecraft", "black_banner", 870)), 
		871 => Some(Item::new("minecraft", "end_crystal", 871)), 
		872 => Some(Item::new("minecraft", "chorus_fruit", 872)), 
		873 => Some(Item::new("minecraft", "popped_chorus_fruit", 873)), 
		874 => Some(Item::new("minecraft", "beetroot", 874)), 
		875 => Some(Item::new("minecraft", "beetroot_seeds", 875)), 
		876 => Some(Item::new("minecraft", "beetroot_soup", 876)), 
		877 => Some(Item::new("minecraft", "dragon_breath", 877)), 
		878 => Some(Item::new("minecraft", "splash_potion", 878)), 
		879 => Some(Item::new("minecraft", "spectral_arrow", 879)), 
		880 => Some(Item::new("minecraft", "tipped_arrow", 880)), 
		881 => Some(Item::new("minecraft", "lingering_potion", 881)), 
		882 => Some(Item::new("minecraft", "shield", 882)), 
		883 => Some(Item::new("minecraft", "elytra", 883)), 
		884 => Some(Item::new("minecraft", "spruce_boat", 884)), 
		885 => Some(Item::new("minecraft", "birch_boat", 885)), 
		886 => Some(Item::new("minecraft", "jungle_boat", 886)), 
		887 => Some(Item::new("minecraft", "acacia_boat", 887)), 
		888 => Some(Item::new("minecraft", "dark_oak_boat", 888)), 
		889 => Some(Item::new("minecraft", "totem_of_undying", 889)), 
		890 => Some(Item::new("minecraft", "shulker_shell", 890)), 
		891 => Some(Item::new("minecraft", "iron_nugget", 891)), 
		892 => Some(Item::new("minecraft", "knowledge_book", 892)), 
		893 => Some(Item::new("minecraft", "debug_stick", 893)), 
		894 => Some(Item::new("minecraft", "music_disc_13", 894)), 
		895 => Some(Item::new("minecraft", "music_disc_cat", 895)), 
		896 => Some(Item::new("minecraft", "music_disc_blocks", 896)), 
		897 => Some(Item::new("minecraft", "music_disc_chirp", 897)), 
		898 => Some(Item::new("minecraft", "music_disc_far", 898)), 
		899 => Some(Item::new("minecraft", "music_disc_mall", 899)), 
		900 => Some(Item::new("minecraft", "music_disc_mellohi", 900)), 
		901 => Some(Item::new("minecraft", "music_disc_stal", 901)), 
		902 => Some(Item::new("minecraft", "music_disc_strad", 902)), 
		903 => Some(Item::new("minecraft", "music_disc_ward", 903)), 
		904 => Some(Item::new("minecraft", "music_disc_11", 904)), 
		905 => Some(Item::new("minecraft", "music_disc_wait", 905)), 
		906 => Some(Item::new("minecraft", "trident", 906)), 
		907 => Some(Item::new("minecraft", "phantom_membrane", 907)), 
		908 => Some(Item::new("minecraft", "nautilus_shell", 908)), 
		909 => Some(Item::new("minecraft", "heart_of_the_sea", 909)), 
		910 => Some(Item::new("minecraft", "crossbow", 910)), 
		911 => Some(Item::new("minecraft", "suspicious_stew", 911)), 
		912 => Some(Item::new("minecraft", "loom", 912)), 
		913 => Some(Item::new("minecraft", "flower_banner_pattern", 913)), 
		914 => Some(Item::new("minecraft", "creeper_banner_pattern", 914)), 
		915 => Some(Item::new("minecraft", "skull_banner_pattern", 915)), 
		916 => Some(Item::new("minecraft", "mojang_banner_pattern", 916)), 
		917 => Some(Item::new("minecraft", "globe_banner_pattern", 917)), 
		918 => Some(Item::new("minecraft", "composter", 918)), 
		919 => Some(Item::new("minecraft", "barrel", 919)), 
		920 => Some(Item::new("minecraft", "smoker", 920)), 
		921 => Some(Item::new("minecraft", "blast_furnace", 921)), 
		922 => Some(Item::new("minecraft", "cartography_table", 922)), 
		923 => Some(Item::new("minecraft", "fletching_table", 923)), 
		924 => Some(Item::new("minecraft", "grindstone", 924)), 
		925 => Some(Item::new("minecraft", "lectern", 925)), 
		926 => Some(Item::new("minecraft", "smithing_table", 926)), 
		927 => Some(Item::new("minecraft", "stonecutter", 927)), 
		928 => Some(Item::new("minecraft", "bell", 928)), 
		929 => Some(Item::new("minecraft", "lantern", 929)), 
		930 => Some(Item::new("minecraft", "soul_fire_lantern", 930)), 
		931 => Some(Item::new("minecraft", "sweet_berries", 931)), 
		932 => Some(Item::new("minecraft", "campfire", 932)), 
		933 => Some(Item::new("minecraft", "shroomlight", 933)), 
		934 => Some(Item::new("minecraft", "honeycomb", 934)), 
		935 => Some(Item::new("minecraft", "bee_nest", 935)), 
		936 => Some(Item::new("minecraft", "beehive", 936)), 
		937 => Some(Item::new("minecraft", "honey_bottle", 937)), 
		938 => Some(Item::new("minecraft", "honey_block", 938)), 
		939 => Some(Item::new("minecraft", "honeycomb_block", 939)), 
		940 => Some(Item::new("minecraft", "netherite_block", 940)), 
		941 => Some(Item::new("minecraft", "ancient_debris", 941)), 
		942 => Some(Item::new("minecraft", "netherite_ingot", 942)), 
		943 => Some(Item::new("minecraft", "netherite_scrap", 943)), 
		944 => Some(Item::new("minecraft", "target", 944)), 
		945 => Some(Item::new("minecraft", "crying_obsidian", 945)), 
		946 => Some(Item::new("minecraft", "respawn_anchor", 946)),
		_ => None
	}
}

pub fn by_identifier(id: Identifier) -> Option<Item> {
	for item in get_items().iter() {
		if item.get_name() == id.get_name() && item.get_namespace() == id.get_namespace() {
			return Some(item.clone())
		}
	}
	return None
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