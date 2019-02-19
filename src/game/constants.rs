// Entry Scene
pub const START_BUTTON_X_OFFSET: f64 = 8f64;
pub const START_BUTTON_Y_OFFSET: f64 = 2f64;
pub const START_BUTTON_WIDTH: f64 = 124f64;
pub const START_BUTTON_HEIGHT: f64 = 32f64;
pub const START_BUTTON_FLASH_FREQUENCY: f64 = 1500f64;

pub const TITLE_X_OFFSET: f64 = 0f64;
pub const TITLE_Y_OFFSET: f64 = 2f64;
pub const TITLE_WIDTH: f64 = 64f64;
pub const TITLE_HEIGHT: f64 = 32f64;

// Level Selection Scene
pub const LEVEL_BUTTON_X_OFFSET: f64 = 4f64;
pub const LEVEL_BUTTON_Y_OFFSET: f64 = 2f64;
pub const LEVEL_BUTTON_WIDTH: f64 = 40f64;
pub const LEVEL_BUTTON_HEIGHT: f64 = 40f64;
pub const LEVEL_BUTTON_SPRITE_WIDTH: f64 = 32f64;
pub const LEVEL_BUTTON_SPRITE_HEIGHT: f64 = 32f64;
pub const LEVEL_BUTTON_MARGIN: f64 = 15f64;
pub const LEVELS_PER_PAGE: usize = 8;
pub const ROW_PER_PAGE: usize = 2;
pub const PAGE_BUTTON_WIDTH: f64 = 32f64;
pub const PAGE_BUTTON_HEIGHT: f64 = 32f64;
pub const DIGIT_WIDTH: f64 = 16f64;
pub const DIGIT_HEIGHT: f64 = 16f64;
pub const ZERO_X_OFFSET: f64 = 8f64;
pub const ZERO_Y_OFFSET: f64 = 0f64;
pub const ONE_X_OFFSET: f64 = 9f64;
pub const ONE_Y_OFFSET: f64 = 0f64;
pub const TWO_X_OFFSET: f64 = 10f64;
pub const TWO_Y_OFFSET: f64 = 0f64;
pub const THREE_X_OFFSET: f64 = 11f64;
pub const THREE_Y_OFFSET: f64 = 0f64;
pub const FOUR_X_OFFSET: f64 = 12f64;
pub const FOUR_Y_OFFSET: f64 = 0f64;
pub const FIVE_X_OFFSET: f64 = 8f64;
pub const FIVE_Y_OFFSET: f64 = 1f64;
pub const SIX_X_OFFSET: f64 = 9f64;
pub const SIX_Y_OFFSET: f64 = 1f64;
pub const SEVEN_X_OFFSET: f64 = 10f64;
pub const SEVEN_Y_OFFSET: f64 = 1f64;
pub const EIGHT_X_OFFSET: f64 = 11f64;
pub const EIGHT_Y_OFFSET: f64 = 1f64;
pub const NINE_X_OFFSET: f64 = 12f64;
pub const NINE_Y_OFFSET: f64 = 1f64;
pub const LEFT_ARROW_X_OFFSET: f64 = 13f64;
pub const LEFT_ARROW_Y_OFFSET: f64 = 0f64;
pub const RIGHT_ARROW_X_OFFSET: f64 = 14f64;
pub const RIGHT_ARROW_Y_OFFSET: f64 = 0f64;
pub const ARROW_BUTTON_SPRITE_SIZE: f64 = 16f64;

// Game Scene
pub const WINDOW_WIDTH_IN_TILES: usize = 13;
pub const WINDOW_HEIGHT_IN_TILES: usize = 8;
pub const WORLD_WIDTH_IN_TILES: usize = 12;
pub const WORLD_HEIGHT_IN_TILES: usize = 8;
pub const TOTAL_TILES: usize = WORLD_WIDTH_IN_TILES * WORLD_HEIGHT_IN_TILES;
pub const MAX_BURNING_LEVEL: isize = 3;

pub const TILE_SIZE: f64 = 32f64;
pub const ASSET_SIZE: f64 = 16f64;
pub const FALLING_BUFFER_TIME: f64 = 32f64;
pub const BACK_BUTTON_X_OFFSET: f64 = 0f64;
pub const BACK_BUTTON_Y_OFFSET: f64 = 0f64;
pub const BACK_BUTTON_SIZE: f64 = 32f64;
pub const RESET_BUTTON_X_OFFSET: f64 = 4f64;
pub const RESET_BUTTON_Y_OFFSET: f64 = 0f64;
pub const RESET_BUTTON_SIZE: f64 = 32f64;

// Keyboard
pub const ARROW_UP: &str = "ArrowUp";
pub const ARROW_DOWN: &str = "ArrowDown";
pub const ARROW_LEFT: &str = "ArrowLeft";
pub const ARROW_RIGHT: &str = "ArrowRight";
pub const ACTION_KEY: &str = "z";

// Player
pub const PLAYER_DOWN_X_OFFSET: f64 = 0f64;
pub const PLAYER_DOWN_Y_OFFSET: f64 = 0f64;
pub const PLAYER_UP_X_OFFSET: f64 = 0f64;
pub const PLAYER_UP_Y_OFFSET: f64 = 4f64;
pub const PLAYER_LEFT_X_OFFSET: f64 = 0f64;
pub const PLAYER_LEFT_Y_OFFSET: f64 = 6f64;
pub const PLAYER_RIGHT_X_OFFSET: f64 = 0f64;
pub const PLAYER_RIGHT_Y_OFFSET: f64 = 2f64;
pub const PLAYER_WIDTH: f64 = 32f64;
pub const PLAYER_HEIGHT: f64 = 32f64;
pub const PLAYER_MOVE_TIME: f64 = 280f64;
pub const PLAYER_WALKING_ANIMATION_TIME: f64 = 280f64;
pub const PLAYER_WALKING_ANIMATION_SPRITE_LENGTH: isize = 4;
pub const PLAYER_PUSHING_X_OFFSET: f64 = 12f64;
pub const PLAYER_IDLE_X_OFFSET: f64 = 8f64;
pub const PLAYER_IDLE_ANIMATION_TIME: f64 = 1_000f64;
pub const PLAYER_IDLE_ANIMATION_SPRITE_LENGTH: isize = 2;
pub const PLAYER_DEAD_X_OFFSET: f64 = 0f64;
pub const PLAYER_DEAD_Y_OFFSET: f64 = 8f64;
pub const PLAYER_DEAD_ANIMATION_TIME: f64 = 500f64;
pub const PLAYER_DEAD_ANIMATION_SPRITE_LENGTH: isize = 1;
pub const PLAYER_FALL_DEAD_X_OFFSET: f64 = 2f64;
pub const PLAYER_FALL_DEAD_Y_OFFSET: f64 = 8f64;
pub const PLAYER_FALL_DEAD_ANIMATION_SPRITE_LENGTH: isize = 4;
pub const PLAYER_RESPAWNING_ANIMATION_TIME: f64 = 500f64;
pub const PLAYER_RESPAWNING_ANIMATION_SPRITE_LENGTH: isize = 4;
pub const PLAYER_EXITING_X_OFFSET: f64 = 10f64;
pub const PLAYER_EXITING_Y_OFFSET: f64 = 8f64;
pub const PLAYER_EXITING_ANIMATION_TIME: f64 = 1_000f64;
pub const PLAYER_EXITING_ANIMATION_SPRITE_LENGTH: isize = 4;
pub const PLAYER_ROTATION_LAG: f64 = 55f64;

// Terrains
pub const GRASS_LAND_ONE_X_OFFSET: f64 = 0f64;
pub const GRASS_LAND_ONE_Y_OFFSET: f64 = 2f64;
pub const GRASS_LAND_TWO_X_OFFSET: f64 = 2f64;
pub const GRASS_LAND_TWO_Y_OFFSET: f64 = 2f64;
pub const GRASS_LAND_THREE_X_OFFSET: f64 = 4f64;
pub const GRASS_LAND_THREE_Y_OFFSET: f64 = 2f64;
pub const GRASS_LAND_FOUR_X_OFFSET: f64 = 6f64;
pub const GRASS_LAND_FOUR_Y_OFFSET: f64 = 2f64;
pub const GRASS_LAND_SIZE: f64 = 32f64;

// Objects
pub const TREE_X_OFFSET: f64 = 0f64;
pub const TREE_Y_OFFSET: f64 = 0f64;
pub const TREE_BURNING_X_OFFSET: f64 = 2f64;
pub const TREE_BURNING_Y_OFFSET: f64 = 0f64;
pub const TREE_BURNING_ANIMATION_TIME: f64 = 350f64;
pub const TREE_BURNING_ANIMATION_SPRITE_LENGTH: isize = 2;
pub const TREE_BURNING_END_X_OFFSET: f64 = 6f64;
pub const TREE_BURNING_END_Y_OFFSET: f64 = 0f64;
pub const TREE_BURN_DOWN_TIME: f64 = 9_900f64;
pub const TREE_IGNITE_FRAMES: f64 = 200f64;
pub const TREE_SIZE: f64 = 32f64;

pub const ROCK_X_OFFSET: f64 = 0f64;
pub const ROCK_Y_OFFSET: f64 = 2f64;
pub const ROCK_SIZE: f64 = 32f64;
pub const ROCK_MOVE_TIME: f64 = 280f64;

pub const HOLE_X_OFFSET: f64 = 0f64;
pub const HOLE_FILLED_X_OFFSET: f64 = 2f64;
pub const HOLE_Y_OFFSET: f64 = 0f64;
pub const HOLE_SIZE: f64 = 32f64;
pub const HOLE_FALL_THRESHOLD: f64 = 0.9f64;

pub const CHEST_X_OFFSET: f64 = 8f64;
pub const CHEST_Y_OFFSET: f64 = 2f64;
pub const CHEST_SIZE: f64 = 32f64;

pub const CANNON_UP_X_OFFSET: f64 = 0f64;
pub const CANNON_UP_Y_OFFSET: f64 = 6f64;
pub const CANNON_RIGHT_X_OFFSET: f64 = 4f64;
pub const CANNON_RIGHT_Y_OFFSET: f64 = 6f64;
pub const CANNON_DOWN_X_OFFSET: f64 = 8f64;
pub const CANNON_DOWN_Y_OFFSET: f64 = 6f64;
pub const CANNON_LEFT_X_OFFSET: f64 = 12f64;
pub const CANNON_LEFT_Y_OFFSET: f64 = 6f64;
pub const CANNON_WIDTH: f64 = 32f64;
pub const CANNON_HEIGHT: f64 = 32f64;
pub const CANNON_MOVE_TIME: f64 = 280f64;
pub const CANNON_ROTATION_ANIMATION_TIME: f64 = 50f64;
pub const CANNON_ROTATION_ANIMATION_SPRITE_LENGTH: isize = 8;
pub const SLOW_CANNON_PROJECT_CYCLE: f64 = 3000f64;
pub const FAST_CANNON_PROJECT_CYCLE: f64 = 1500f64;

pub const PROJECTILE_X_OFFSET: f64 = 0f64;
pub const PROJECTILE_Y_OFFSET: f64 = 4f64;
pub const PROJECTILE_SIZE: f64 = 32f64;
pub const PROJECTILE_MOVE_TIME: f64 = 150f64;
pub const PROJECTILE_BURNING_X_OFFSET: f64 = 2f64;
pub const PROJECTILE_BURNING_Y_OFFSET: f64 = 4f64;

pub const SMOKE_BASE_X_OFFSET: f64 = 0f64;
pub const SMOKE_BASE_Y_OFFSET: f64 = 10f64;
pub const SMOKE_ANIMATION_TIME: f64 = 420f64;
pub const SMOKE_ANIMATION_SPRITE_LENGTH: isize = 8;
pub const SMOKE_SIZE: f64 = 32f64;

pub const FIRE_SOURCE_OFF_X_OFFSET: f64 = 0f64;
pub const FIRE_SOURCE_OFF_Y_OFFSET: f64 = 8f64;
pub const FIRE_SOURCE_ON_X_OFFSET: f64 = 2f64;
pub const FIRE_SOURCE_ON_Y_OFFSET: f64 = 8f64;
pub const FIRE_SOURCE_SIZE: f64 = 32f64;
pub const FIRE_SOURCE_MOVE_TIME: f64 = 280f64;
pub const FIRE_SOURCE_IGNITE_FRAMES: f64 = 60f64;
pub const FIRE_ANIMATION_TIME: f64 = 350f64;
pub const FIRE_ANIMATION_SPRITE_LENGTH: isize = 2;

pub const WALL_X_OFFSET: f64 = 2f64;
pub const WALL_Y_OFFSET: f64 = 2f64;
pub const WALL_WIDTH: f64 = 32f64;
pub const WALL_HEIGHT: f64 = 32f64;

pub const BREAKABLE_WALL_X_OFFSET: f64 = 4f64;
pub const BREAKABLE_WALL_Y_OFFSET: f64 = 2f64;
pub const BREAKABLE_WALL_WIDTH: f64 = 32f64;
pub const BREAKABLE_WALL_HEIGHT: f64 = 32f64;

pub const SPAWNING_POINT_X_OFFSET: f64 = 6f64;
pub const SPAWNING_POINT_Y_OFFSET: f64 = 2f64;
pub const SPAWNING_POINT_WIDTH: f64 = 32f64;
pub const SPAWNING_POINT_HEIGHT: f64 = 32f64;
