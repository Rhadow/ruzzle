use crate::game::constants::TOTAL_TILES;
use crate::game::Position;

const TERRAINS: [&str; TOTAL_TILES] = [
    "G ","G ","G ","G ","G ","G ","G ","G ","G ","G ","G ","G ",
    "G ","G ","G ","G ","G ","G ","G ","G ","G ","G ","G ","G ",
    "G ","G ","G ","G ","G ","G ","G ","G ","G ","G ","G ","G ",
    "G ","G ","G ","G ","G ","G ","G ","G ","G ","G ","G ","G ",
    "G ","G ","G ","G ","G ","G ","G ","G ","G ","G ","G ","G ",
    "G ","G ","G ","G ","G ","G ","G ","G ","G ","G ","G ","G ",
    "G ","G ","G ","G ","G ","G ","G ","G ","G ","G ","G ","G ",
    "G ","G ","G ","G ","G ","G ","G ","G ","G ","G ","G ","G ",
];

const OBJECTS: [&str; TOTAL_TILES] = [
    "W ","DS","T ","T ","T ","T ","T ","T ","  ","T ","  ","  ",
    "W ","  ","T ","T ","T ","R ","  ","R ","  ","  ","  ","  ",
    "W ","  ","T ","T ","  ","R ","R ","  ","R ","  ","  ","  ",
    "W ","  ","  ","  ","  ","  ","R ","  ","R ","  ","W ","W ",
    "W ","  ","T ","T ","  ","R ","R ","  ","R ","  ","W ","T ",
    "W ","  ","  ","T ","T ","R ","  ","R ","T ","T ","T ","C ",
    "W ","  ","T ","T ","  ","T ","T ","T ","  ","T ","T ","  ",
    "RF","  ","  ","  ","  ","  ","  ","  ","  ","  ","  ","  ",
];

const PLAYER_POSITION: Position = Position(1f64, 11f64);

pub const LEVEL02: ([&str; TOTAL_TILES], [&str; TOTAL_TILES], Position) = (TERRAINS, OBJECTS, PLAYER_POSITION);
