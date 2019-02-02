use crate::game::constants::TOTAL_TILES;
use crate::game::Position;

const TERRAINS: [&str; TOTAL_TILES] = [
    "G1","G1","G1","G1","G1","G1","G1","G1","G1","G1","G1","H ",
    "G1","G1","G1","G1","G1","G1","G1","G1","G1","G1","G1","G1",
    "G1","G1","G1","G1","G1","G1","G1","G1","G1","G1","G1","G1",
    "G1","G1","G1","G1","G1","G1","G1","G1","G1","G1","G1","G1",
    "G1","G1","G1","G1","G1","G1","G1","G1","G1","G1","G1","G1",
    "G1","G1","G1","G1","H ","G1","G1","G1","G1","G1","G1","G1",
    "G1","G1","G1","G1","G1","G1","G1","G1","G1","G1","G1","G1",
    "G1","G1","G1","G1","G1","G1","G1","G1","G1","G1","G1","G1",
];

const OBJECTS: [&str; TOTAL_TILES] = [
    "  ","SP","T ","T ","T ","T ","T ","T ","  ","T ","C ","  ",
    "  ","  ","  ","T ","T ","T ","T ","  ","  ","T ","T ","  ",
    "T ","R ","R ","  ","T ","  ","R ","  ","  ","T ","  ","  ",
    "  ","  ","  ","  ","T ","  ","W ","W ","  ","T ","  ","  ",
    "T ","T ","T ","  ","T ","  ","W ","  ","  ","  ","  ","  ",
    "  ","  ","  ","  ","  ","  ","  ","  ","T ","T ","  ","  ",
    "  ","R ","  ","  ","T ","T ","T ","T ","T ","T ","  ","  ",
    "  ","  ","  ","  ","T ","T ","T ","T ","T ","T ","T ","T ",
];

const PLAYER_POSITION: Position = Position(0f64, 1f64);

pub const LEVEL01: ([&str; TOTAL_TILES], [&str; TOTAL_TILES], Position) = (TERRAINS, OBJECTS, PLAYER_POSITION);
