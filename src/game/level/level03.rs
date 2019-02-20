use crate::game::constants::TOTAL_TILES;
use crate::game::Position;

const TERRAINS: [&str; TOTAL_TILES] = [
    "G4","G4","G4","G4","G4","G4","G3","G4","G3","G4","G4","G4",
    "G3","G1","G4","G4","G4","G3","G3","G4","G4","G1","G4","G4",
    "G4","G1","G3","G2","H ","H ","G1","G4","G1","G4","G3","G4",
    "G4","G3","G4","G3","G4","G3","G4","G3","G3","G3","G1","G3",
    "G1","G3","G4","G3","G4","G4","G3","G4","G4","G4","G4","G2",
    "G3","G4","G4","G3","G4","G3","G4","G3","G1","G3","G3","G3",
    "G4","G4","G3","G3","G3","G4","G4","G4","G2","G4","G4","G1",
    "G3","G1","G4","G4","G2","G3","G3","G3","G3","G3","G4","G4"
];

const OBJECTS: [&str; TOTAL_TILES] = [
    "W ","  ","  ","T ","  ","  ","  ","W ","W ","T ","DF","W ",
    "RS","  ","  ","  ","  ","R ","  ","  ","T ","W ","  ","  ",
    "W ","  ","R ","  ","  ","  ","W ","W ","T ","  ","  ","  ",
    "  ","  ","  ","W ","SP","  ","W ","  ","T ","NF","  ","  ",
    "  ","W ","W ","  ","  ","  ","W ","  ","  ","  ","  ","  ",
    "  ","  ","W ","  ","NF","  ","BF","W ","  ","T ","T ","T ",
    "T ","  ","  ","W ","T ","T ","W ","  ","T ","  ","W ","  ",
    "T ","T ","  ","  ","  ","  ","  ","W ","T ","  ","C ","  ",
];

const PLAYER_POSITION: Position = Position(3f64, 4f64);

pub const LEVEL03: ([&str; TOTAL_TILES], [&str; TOTAL_TILES], Position) = (TERRAINS, OBJECTS, PLAYER_POSITION);
