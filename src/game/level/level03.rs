use crate::game::constants::TOTAL_TILES;
use crate::game::Position;

const TERRAINS: [&str; TOTAL_TILES] = [
    "G1","G1","G1","G1","G1","G1","G1","G1","G1","G1","G1","G1",
    "G1","G1","G1","G1","G1","G1","H ","G1","G1","G1","G1","G1",
    "G1","G1","G1","G1","G1","G1","G1","G1","G1","G1","G1","G1",
    "G1","G1","G1","G1","G1","G1","G1","G1","G1","G1","G1","G1",
    "G1","G1","G1","G1","G1","G1","G1","G1","G1","G1","G1","G1",
    "G1","G1","G1","G1","G1","G1","G1","G1","G1","G1","G1","G1",
    "G1","G1","G1","G1","G1","G1","G1","G1","G1","G1","G1","G1",
    "G1","G1","G1","G1","G1","G1","G1","G1","G1","G1","G1","G1",
];

const OBJECTS: [&str; TOTAL_TILES] = [
    "  ","  ","T ","T ","  ","T ","  ","  ","  ","  ","  ","T ",
    "  ","  ","  ","  ","  ","  ","  ","  ","  ","  ","T ","T ",
    "  ","RS","BF","R ","NF","T ","  ","R ","  ","T ","T ","  ",
    "  ","  ","  ","  ","  ","  ","  ","  ","BW","T ","C ","  ",
    "  ","  ","NF","  ","  ","  ","  ","  ","W ","T ","  ","T ",
    "  ","  ","  ","  ","  ","  ","  ","  ","W ","T ","R ","T ",
    "  ","T ","T ","  ","  ","  ","DF","  ","  ","  ","T ","T ",
    "SP","  ","  ","  ","  ","  ","  ","  ","  ","  ","  ","T ",
];

const PLAYER_POSITION: Position = Position(7f64, 0f64);

pub const LEVEL03: ([&str; TOTAL_TILES], [&str; TOTAL_TILES], Position) = (TERRAINS, OBJECTS, PLAYER_POSITION);
