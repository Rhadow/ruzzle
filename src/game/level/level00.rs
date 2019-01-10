use crate::game::constants::TOTAL_CELLS;
use crate::game::Coordinate;

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

const PLAYER_COORDINATE: Coordinate = Coordinate(7, 0);

pub const LEVEL00: ([&str; TOTAL_CELLS], [&str; TOTAL_CELLS], Coordinate) = (TERRAINS, OBJECTS, PLAYER_COORDINATE);
