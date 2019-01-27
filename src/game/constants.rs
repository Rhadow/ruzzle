// Entry Scene
pub const START_BUTTON_WIDTH: f64 = 100f64;
pub const START_BUTTON_HEIGHT: f64 = 20f64;

// Level Selection Scene
pub const LEVEL_BUTTON_WIDTH: f64 = 80f64;
pub const LEVEL_BUTTON_HEIGHT: f64 = 80f64;
pub const LEVEL_BUTTON_MARGIN: f64 = 30f64;
pub const LEVELS_PER_PAGE: usize = 8;
pub const ROW_PER_PAGE: usize = 2;
pub const PAGE_BUTTON_WIDTH: f64 = 30f64;
pub const PAGE_BUTTON_HEIGHT: f64 = 30f64;

// Game Scene
pub const WINDOW_WIDTH_IN_TILES: usize = 13;
pub const WINDOW_HEIGHT_IN_TILES: usize = 8;
pub const WORLD_WIDTH_IN_TILES: usize = 12;
pub const WORLD_HEIGHT_IN_TILES: usize = 8;
pub const TOTAL_TILES: usize = WORLD_WIDTH_IN_TILES * WORLD_HEIGHT_IN_TILES;
pub const MAX_BURNING_LEVEL: isize = 3;

pub const TILE_SIZE: f64 = 64f64;
pub const ASSET_SIZE: f64 = 16f64;
pub const FALLING_BUFFER_TIME: f64 = 32f64;

pub const BACK_BUTTON_WIDTH: f64 = 64f64;
pub const BACK_BUTTON_HEIGHT: f64 = 64f64;
pub const RESET_BUTTON_WIDTH: f64 = 64f64;
pub const RESET_BUTTON_HEIGHT: f64 = 64f64;

// Keyboard
pub const ARROW_UP: &str = "ArrowUp";
pub const ARROW_DOWN: &str = "ArrowDown";
pub const ARROW_LEFT: &str = "ArrowLeft";
pub const ARROW_RIGHT: &str = "ArrowRight";
pub const ACTION_KEY: &str = "z";

// Player
pub const PLAYER_BASE_X_OFFSET: f64 = 0f64;
pub const PLAYER_BASE_Y_OFFSET: f64 = 0f64;
pub const PLAYER_WIDTH: f64 = 16f64;
pub const PLAYER_HEIGHT: f64 = 32f64;
pub const PLAYER_MOVE_TIME: f64 = 280f64;
pub const PLAYER_WALKING_ANIMATION_TIME: f64 = 280f64;
pub const PLAYER_WALKING_ANIMATION_SPRITE_LENGTH: isize = 4;
pub const PLAYER_PUSHING_X_OFFSET: f64 = 9f64;
pub const PLAYER_IDLE_X_OFFSET: f64 = 9f64;
pub const PLAYER_IDLE_ANIMATION_WAITING_TIME: f64 = 5_000f64;
pub const PLAYER_IDLE_ANIMATION_TIME: f64 = 2_000f64;
pub const PLAYER_IDLE_ANIMATION_SPRITE_LENGTH: isize = 4;
pub const PLAYER_DEAD_X_OFFSET: f64 = 6f64;
pub const PLAYER_DEAD_ANIMATION_TIME: f64 = 1_000f64;
pub const PLAYER_DEAD_ANIMATION_SPRITE_LENGTH: isize = 2;
pub const PLAYER_RESPAWNING_ANIMATION_TIME: f64 = 500f64;
pub const PLAYER_RESPAWNING_ANIMATION_SPRITE_LENGTH: isize = 4;
pub const PLAYER_EXITING_ANIMATION_TIME: f64 = 1_000f64;
pub const PLAYER_EXITING_ANIMATION_SPRITE_LENGTH: isize = 4;
pub const PLAYER_ROTATION_LAG: f64 = 55f64;

// Terrains
pub const GRASS_LAND_X_OFFSET: f64 = 7f64;
pub const GRASS_LAND_Y_OFFSET: f64 = 9f64;
pub const GRASS_LAND_SIZE: f64 = 32f64;

pub const WOODEN_PATH_X_OFFSET: f64 = 13f64;
pub const WOODEN_PATH_Y_OFFSET: f64 = 15f64;
pub const WOODEN_PATH_SIZE: f64 = 32f64;

// Objects
pub const TREE_X_OFFSET: f64 = 16f64;
pub const TREE_Y_OFFSET: f64 = 12f64;
pub const TREE_BURNING_LEVEL_1_X_OFFSET: f64 = 6f64;
pub const TREE_BURNING_LEVEL_1_Y_OFFSET: f64 = 12f64;
pub const TREE_BURNING_LEVEL_2_X_OFFSET: f64 = 0f64;
pub const TREE_BURNING_LEVEL_2_Y_OFFSET: f64 = 10f64;
pub const TREE_BURN_DOWN_TIME: f64 = 7200f64;
pub const TREE_SIZE: f64 = 32f64;

pub const ROCK_X_OFFSET: f64 = 35f64;
pub const ROCK_Y_OFFSET: f64 = 8f64;
pub const ROCK_SIZE: f64 = 32f64;
pub const ROCK_MOVE_TIME: f64 = 280f64;

pub const HOLE_X_OFFSET: f64 = 35f64;
pub const HOLE_FILLED_X_OFFSET: f64 = 33f64;
pub const HOLE_Y_OFFSET: f64 = 3f64;
pub const HOLE_SIZE: f64 = 32f64;

pub const CHEST_X_OFFSET: f64 = 0f64;
pub const CHEST_Y_OFFSET: f64 = 0f64;
pub const CHEST_SIZE: f64 = 16f64;

pub const CANNON_DOWN_X_OFFSET: f64 = 10f64;
pub const CANNON_DOWN_Y_OFFSET: f64 = 22f64;
pub const CANNON_UP_X_OFFSET: f64 = 10f64;
pub const CANNON_UP_Y_OFFSET: f64 = 24f64;
pub const CANNON_LEFT_X_OFFSET: f64 = 16f64;
pub const CANNON_LEFT_Y_OFFSET: f64 = 27f64;
pub const CANNON_RIGHT_X_OFFSET: f64 = 14f64;
pub const CANNON_RIGHT_Y_OFFSET: f64 = 27f64;
pub const CANNON_VERTICAL_WIDTH: f64 = 16f64;
pub const CANNON_VERTICAL_HEIGHT: f64 = 32f64;
pub const CANNON_HORIZONTAL_WIDTH: f64 = 32f64;
pub const CANNON_HORIZONTAL_HEIGHT: f64 = 32f64;
pub const CANNON_MOVE_TIME: f64 = 280f64;
pub const CANNON_PROJECT_TIME: f64 = 5000f64;

pub const PROJECTILE_X_OFFSET: f64 = 17f64;
pub const PROJECTILE_Y_OFFSET: f64 = 1f64;
pub const PROJECTILE_SIZE: f64 = 16f64;
pub const PROJECTILE_MOVE_TIME: f64 = 150f64;
pub const PROJECTILE_BURNING_X_OFFSET: f64 = 0f64;
pub const PROJECTILE_BURNING_Y_OFFSET: f64 = 4f64;

pub const SMOKE_BASE_X_OFFSET: f64 = 4f64;
pub const SMOKE_BASE_Y_OFFSET: f64 = 4f64;
pub const SMOKE_ANIMATION_TIME: f64 = 420f64;
pub const SMOKE_ANIMATION_SPRITE_LENGTH: isize = 7;
pub const SMOKE_SIZE: f64 = 32f64;

pub const FIRE_SOURCE_X_OFFSET: f64 = 4f64;
pub const FIRE_SOURCE_Y_OFFSET: f64 = 3f64;
pub const FIRE_SOURCE_SIZE: f64 = 16f64;
pub const FIRE_ANIMATION_TIME: f64 = 500f64;
pub const FIRE_ANIMATION_SPRITE_LENGTH: isize = 7;

pub const WALL_X_OFFSET: f64 = 8f64;
pub const WALL_Y_OFFSET: f64 = 31f64;
pub const WALL_WIDTH: f64 = 32f64;
pub const WALL_HEIGHT: f64 = 64f64;

pub const BREAKABLE_WALL_X_OFFSET: f64 = 33f64;
pub const BREAKABLE_WALL_Y_OFFSET: f64 = 0f64;
pub const BREAKABLE_WALL_WIDTH: f64 = 16f64;
pub const BREAKABLE_WALL_HEIGHT: f64 = 32f64;
