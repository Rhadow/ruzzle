use crate::game::constants::TOTAL_TILES;
use crate::game::Position;

const TERRAINS: [&str; TOTAL_TILES] = [
    "G ","H ","G ","G ","G ","G ","G ","G ","G ","G ","G ","G ",
    "G ","H ","G ","G ","G ","G ","G ","G ","G ","G ","G ","G ",
    "G ","H ","G ","G ","G ","G ","G ","G ","G ","G ","G ","G ",
    "G ","WP","H ","WP","WP","WP","WP","WP","G ","G ","G ","G ",
    "G ","H ","G ","G ","G ","G ","G ","WP","WP","G ","G ","G ",
    "H ","WP","G ","G ","G ","G ","G ","G ","WP","G ","G ","G ",
    "G ","H ","G ","G ","G ","G ","G ","G ","WP","WP","WP","WP",
    "G ","G ","H ","G ","G ","G ","G ","G ","G ","G ","G ","G ",
];

const OBJECTS: [&str; TOTAL_TILES] = [
    "  ","  ","R ","  ","  ","  ","  ","  ","  ","  ","  ","  ",
    "  ","  ","  ","  ","T ","  ","  ","R ","  ","  ","  ","  ",
    "  ","  ","  ","R ","  ","  ","  ","  ","  ","  ","T ","  ",
    "  ","  ","  ","  ","R ","  ","  ","  ","  ","  ","  ","  ",
    "  ","  ","  ","R ","  ","  ","  ","  ","  ","  ","  ","  ",
    "  ","  ","  ","  ","  ","  ","DN","  ","  ","  ","  ","  ",
    "R ","  ","T ","  ","  ","R ","  ","  ","  ","  ","  ","C ",
    "  ","  ","  ","  ","  ","  ","R ","  ","  ","  ","  ","  ",
];

const PLAYER_POSITION: Position = Position(7f64, 0f64);

pub const LEVEL00: ([&str; TOTAL_TILES], [&str; TOTAL_TILES], Position) = (TERRAINS, OBJECTS, PLAYER_POSITION);
