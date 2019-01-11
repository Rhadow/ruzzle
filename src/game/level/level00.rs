use crate::game::constants::TOTAL_CELLS;
use crate::game::Position;

const TERRAINS: [&str; TOTAL_CELLS] = [
    "G ","WP","G ","G ","G ","G ","G ","G ","G ","G ","G ","G ",
    "G ","WP","G ","G ","G ","G ","G ","G ","G ","G ","G ","G ",
    "G ","WP","G ","G ","G ","G ","G ","G ","G ","G ","G ","G ",
    "G ","WP","WP","WP","WP","WP","WP","WP","G ","G ","G ","G ",
    "G ","WP","G ","G ","G ","G ","G ","WP","WP","G ","G ","G ",
    "G ","WP","G ","G ","G ","G ","G ","G ","WP","G ","G ","G ",
    "G ","G ","G ","G ","G ","G ","G ","G ","WP","WP","WP","WP",
    "G ","G ","G ","G ","G ","G ","G ","G ","G ","G ","G ","G ",
];

const OBJECTS: [&str; TOTAL_CELLS] = [
    "  ","  ","  ","  ","  ","  ","  ","  ","  ","  ","  ","  ",
    "  ","  ","  ","  ","T ","  ","  ","  ","  ","  ","  ","  ",
    "  ","  ","  ","  ","  ","  ","  ","  ","  ","  ","T ","  ",
    "  ","  ","  ","  ","  ","  ","  ","  ","  ","  ","  ","  ",
    "  ","  ","  ","  ","  ","  ","  ","  ","  ","  ","  ","  ",
    "  ","  ","  ","  ","  ","  ","  ","  ","  ","  ","  ","  ",
    "  ","  ","T ","  ","  ","  ","  ","  ","  ","  ","  ","  ",
    "  ","  ","  ","  ","  ","  ","  ","  ","  ","  ","  ","  ",
];

const PLAYER_POSITION: Position = Position(7f64, 0f64);

pub const LEVEL00: ([&str; TOTAL_CELLS], [&str; TOTAL_CELLS], Position) = (TERRAINS, OBJECTS, PLAYER_POSITION);
