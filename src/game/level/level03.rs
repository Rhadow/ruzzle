use crate::game::constants::TOTAL_TILES;
use crate::game::Position;

const TERRAINS: [&str; TOTAL_TILES] = [
    "G ","G ","G ","G ","G ","G ","G ","G ","G ","G ","G ","G ",
    "G ","G ","G ","G ","G ","G ","H ","G ","G ","G ","G ","G ",
    "G ","G ","G ","G ","G ","G ","G ","G ","G ","G ","G ","G ",
    "G ","G ","G ","G ","G ","G ","G ","G ","G ","G ","G ","G ",
    "G ","G ","G ","G ","G ","G ","G ","G ","G ","G ","G ","G ",
    "G ","G ","G ","G ","G ","G ","G ","G ","G ","G ","G ","G ",
    "G ","G ","G ","G ","G ","G ","G ","G ","G ","G ","G ","G ",
    "G ","G ","G ","G ","G ","G ","G ","G ","G ","G ","G ","G ",
];

const OBJECTS: [&str; TOTAL_TILES] = [
    "  ","  ","T ","T ","  ","T ","  ","  ","  ","  ","  ","T ",
    "  ","  ","  ","  ","  ","  ","  ","  ","  ","  ","T ","T ",
    "  ","RS","BF","R ","NF","T ","  ","R ","  ","T ","T ","  ",
    "  ","  ","  ","  ","  ","  ","  ","  ","BW","T ","C ","  ",
    "  ","  ","NF","  ","  ","  ","  ","  ","W ","T ","  ","T ",
    "  ","  ","  ","  ","  ","  ","  ","  ","W ","T ","R ","T ",
    "  ","T ","T ","  ","  ","  ","DF","  ","  ","  ","T ","T ",
    "  ","  ","  ","  ","  ","  ","  ","  ","  ","  ","  ","T ",
];

const PLAYER_POSITION: Position = Position(1f64, 11f64);

pub const LEVEL03: ([&str; TOTAL_TILES], [&str; TOTAL_TILES], Position) = (TERRAINS, OBJECTS, PLAYER_POSITION);
