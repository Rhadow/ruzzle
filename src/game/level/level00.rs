use crate::game::constants::TOTAL_TILES;
use crate::game::Position;

const TERRAINS: [&str; TOTAL_TILES] = [
    "G ","G ","G ","G ","G ","G ","G ","G ","G ","G ","G ","G ",
    "G ","H ","G ","G ","G ","G ","G ","G ","G ","G ","G ","G ",
    "G ","G ","G ","G ","G ","G ","G ","G ","G ","G ","G ","G ",
    "G ","WP","G ","WP","WP","WP","WP","WP","G ","G ","G ","G ",
    "G ","G ","G ","G ","G ","G ","G ","WP","WP","G ","G ","G ",
    "H ","WP","G ","G ","G ","G ","G ","G ","WP","G ","G ","G ",
    "G ","G ","G ","G ","G ","G ","G ","G ","WP","WP","WP","WP",
    "G ","G ","G ","G ","G ","G ","G ","G ","G ","G ","G ","G ",
];

const OBJECTS: [&str; TOTAL_TILES] = [
    "BW","DF","T ","T ","  ","  ","  ","  ","  ","  ","  ","  ",
    "  ","  ","R ","  ","T ","  ","  ","R ","  ","W ","  ","  ",
    "  ","  ","  ","  ","  ","  ","  ","  ","T ","W ","T ","  ",
    "  ","  ","RF","  ","  ","  ","  ","  ","T ","W ","  ","  ",
    "  ","  ","  ","  ","  ","BF","  ","  ","T ","BW","  ","  ",
    "  ","  ","  ","  ","  ","  ","  ","NF","T ","  ","  ","  ",
    "  ","  ","  ","  ","  ","  ","NF","  ","  ","T ","LS","C ",
    "T ","T ","  ","US","  ","  ","  ","  ","  ","  ","  ","  ",
];

const PLAYER_POSITION: Position = Position(6f64, 0f64);

pub const LEVEL00: ([&str; TOTAL_TILES], [&str; TOTAL_TILES], Position) = (TERRAINS, OBJECTS, PLAYER_POSITION);
