use crate::game::constants::TOTAL_TILES;
use crate::game::Position;

const TERRAINS: [&str; TOTAL_TILES] = [
    "G4","G4","G4","G3","G1","G4","G4","G4","G4","G3","G1","G3",
    "G4","G4","G3","G3","G4","G4","G3","G2","G3","G3","G3","G4",
    "G4","G4","G3","G2","G2","G3","G3","H ","G3","G4","G2","G2",
    "G3","G2","G4","G4","G1","G1","G3","G3","G3","G1","G3","G3",
    "G4","G3","H ","G4","G1","G3","G4","G3","G4","G3","G3","G2",
    "G3","G3","G1","G1","G3","G2","G4","G4","G4","G4","G3","G3",
    "G3","G3","G2","G2","G3","G3","G3","G3","G4","G3","G1","G3",
    "G3","G4","G3","G4","G4","G3","G1","G1","G2","G4","G3","G4"
];

const OBJECTS: [&str; TOTAL_TILES] = [
    "BW","DF","T ","T ","  ","  ","  ","  ","  ","  ","  ","  ",
    "  ","  ","R ","  ","T ","  ","  ","R ","  ","W ","  ","  ",
    "  ","  ","  ","  ","  ","  ","  ","  ","T ","W ","T ","  ",
    "  ","  ","RF","  ","  ","  ","  ","  ","T ","W ","  ","  ",
    "  ","  ","  ","  ","  ","BF","  ","  ","T ","BW","  ","  ",
    "  ","  ","  ","  ","  ","  ","  ","NF","T ","  ","  ","  ",
    "SP","  ","  ","  ","  ","  ","NF","  ","  ","T ","LS","C ",
    "T ","T ","  ","US","  ","  ","  ","  ","  ","  ","  ","  ",
];

const PLAYER_POSITION: Position = Position(6f64, 0f64);

pub const LEVEL00: ([&str; TOTAL_TILES], [&str; TOTAL_TILES], Position) = (TERRAINS, OBJECTS, PLAYER_POSITION);
