use crate::game::constants::TOTAL_TILES;
use crate::game::Position;

const TERRAINS: [&str; TOTAL_TILES] = [
    "G ","G ","G ","G ","G ","G ","G ","G ","G ","G ","G ","G ",
    "G ","G ","G ","G ","G ","G ","G ","G ","G ","G ","G ","G ",
    "G ","G ","G ","G ","G ","G ","G ","G ","G ","G ","G ","G ",
    "G ","WP","H ","WP","WP","WP","WP","WP","G ","G ","G ","G ",
    "G ","G ","G ","G ","G ","G ","G ","WP","WP","G ","G ","G ",
    "G ","WP","G ","G ","G ","G ","G ","G ","WP","G ","G ","G ",
    "G ","G ","G ","G ","G ","G ","G ","G ","WP","WP","WP","WP",
    "G ","G ","H ","G ","G ","G ","G ","G ","G ","G ","G ","G ",
];

const OBJECTS: [&str; TOTAL_TILES] = [
    "  ","  ","T ","  ","  ","  ","  ","  ","  ","  ","  ","  ",
    "  ","  ","  ","  ","T ","  ","  ","R ","  ","  ","  ","  ",
    "P ","  ","  ","  ","  ","  ","  ","  ","UN","  ","T ","  ",
    "P ","  ","  ","  ","  ","  ","  ","  ","RN","  ","  ","  ",
    "P ","  ","  ","  ","  ","  ","  ","  ","LN","  ","  ","  ",
    "P ","  ","  ","  ","  ","  ","  ","  ","DN","  ","  ","  ",
    "P ","  ","  ","  ","  ","  ","  ","  ","  ","  ","  ","C ",
    "P ","  ","  ","  ","  ","  ","R ","  ","  ","  ","  ","  ",
];

const PLAYER_POSITION: Position = Position(5f64, 3f64);

pub const LEVEL01: ([&str; TOTAL_TILES], [&str; TOTAL_TILES], Position) = (TERRAINS, OBJECTS, PLAYER_POSITION);
