use crate::game::constants::TOTAL_TILES;
use crate::game::Position;

const TERRAINS: [&str; TOTAL_TILES] = [
    "G4","G4","G4","G4","G1","G4","G3","G4","G3","G4","G4","G4",
    "G3","G1","G4","G4","G4","G1","G3","G4","G4","G1","G4","G4",
    "G4","G1","G3","G3","G4","G4","G3","G4","G1","G4","G3","G4",
    "G4","G3","G4","G3","G3","G2","G4","G3","G3","G3","G1","G3",
    "G1","G3","G4","G3","G4","G3","G3","G4","G4","G4","G4","G2",
    "G3","G4","G4","G3","G4","G3","G4","G3","G1","G3","G3","G3",
    "G4","G4","G3","G3","G4","G4","G4","G4","G2","G4","G4","G1",
    "G3","G1","G4","G4","G2","G3","G3","G3","G3","G3","G4","G4"
];

const OBJECTS: [&str; TOTAL_TILES] = [
    "  ","  ","  ","T ","T ","T ","  ","  ","  ","  ","T ","T ",
    "  ","DS","  ","T ","  ","  ","W ","BW","W ","  ","  ","T ",
    "  ","  ","  ","T ","  ","R ","  ","R ","  ","W ","  ","T ",
    "  ","  ","  ","T ","T ","  ","T ","  ","T ","T ","  ","  ",
    "  ","  ","  ","W ","  ","  ","  ","  ","  ","T ","W ","  ",
    "  ","R ","  ","W ","  ","T ","R ","R ","R ","T ","  ","  ",
    "W ","BW","W ","W ","T ","  ","  ","  ","  ","T ","  ","T ",
    "  ","  ","  ","  ","  ","  ","  ","US","  ","T ","C ","  ",
];

const PLAYER_POSITION: Position = Position(3f64, 2f64);

pub const LEVEL02: ([&str; TOTAL_TILES], [&str; TOTAL_TILES], Position) = (TERRAINS, OBJECTS, PLAYER_POSITION);
