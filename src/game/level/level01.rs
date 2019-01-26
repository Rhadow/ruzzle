use crate::game::constants::TOTAL_TILES;
use crate::game::Position;

const TERRAINS: [&str; TOTAL_TILES] = [
    "G ","G ","G ","G ","G ","G ","G ","G ","G ","G ","G ","G ",
    "G ","G ","G ","G ","G ","G ","G ","G ","G ","G ","G ","G ",
    "G ","G ","G ","H ","G ","G ","G ","G ","G ","G ","G ","G ",
    "G ","WP","G ","WP","WP","WP","WP","WP","G ","G ","G ","G ",
    "G ","G ","G ","G ","G ","G ","G ","WP","WP","G ","G ","G ",
    "G ","WP","G ","G ","G ","G ","G ","G ","WP","G ","G ","G ",
    "G ","G ","G ","G ","G ","G ","G ","G ","WP","WP","WP","WP",
    "G ","G ","H ","G ","G ","G ","G ","G ","G ","G ","G ","G ",
];

const OBJECTS: [&str; TOTAL_TILES] = [
    "  ","DN","T ","T ","  ","  ","  ","  ","  ","  ","  ","  ",
    "  ","  ","  ","  ","T ","  ","  ","R ","  ","  ","  ","  ",
    "  ","  ","  ","  ","  ","  ","  ","  ","  ","  ","T ","  ",
    "  ","  ","RN","  ","  ","  ","  ","  ","  ","  ","  ","  ",
    "  ","T ","  ","  ","  ","FS","  ","  ","  ","  ","LN","  ",
    "  ","  ","  ","  ","  ","  ","  ","  ","  ","  ","  ","  ",
    "  ","  ","  ","  ","  ","  ","  ","  ","  ","  ","  ","C ",
    "  ","  ","  ","UN","  ","  ","R ","  ","  ","  ","  ","  ",
];

const PLAYER_POSITION: Position = Position(5f64, 4f64);

pub const LEVEL01: ([&str; TOTAL_TILES], [&str; TOTAL_TILES], Position) = (TERRAINS, OBJECTS, PLAYER_POSITION);
