// Game
pub const WORLD_WIDTH_IN_TILES: usize = 12;
pub const WORLD_HEIGHT_IN_TILES: usize = 8;
pub const TOTAL_TILES: usize = WORLD_WIDTH_IN_TILES * WORLD_HEIGHT_IN_TILES;

pub const TILE_SIZE: f64 = 64f64;
pub const ASSET_SIZE: f64 = 16f64;
pub const FALLING_BUFFER_TIME: f64 = 32f64;

// Keyboard
pub const ARROW_UP: &str = "ArrowUp";
pub const ARROW_DOWN: &str = "ArrowDown";
pub const ARROW_LEFT: &str = "ArrowLeft";
pub const ARROW_RIGHT: &str = "ArrowRight";

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
pub const PLAYER_FALLING_X_OFFSET: f64 = 6f64;
pub const PLAYER_FALLING_ANIMATION_TIME: f64 = 1_000f64;
pub const PLAYER_FALLING_ANIMATION_SPRITE_LENGTH: isize = 1;

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
pub const TREE_SIZE: f64 = 32f64;

pub const ROCK_X_OFFSET: f64 = 35f64;
pub const ROCK_Y_OFFSET: f64 = 8f64;
pub const ROCK_SIZE: f64 = 32f64;
pub const ROCK_MOVE_TIME: f64 = 280f64;

pub const HOLE_X_OFFSET: f64 = 35f64;
pub const HOLE_FILLED_X_OFFSET: f64 = 33f64;
pub const HOLE_Y_OFFSET: f64 = 3f64;
pub const HOLE_SIZE: f64 = 32f64;

