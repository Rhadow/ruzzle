use crate::game::constants::TOTAL_TILES;
use crate::game::Position;

const TERRAINS: [&str; TOTAL_TILES] = [
    "G3","G4","G3","G4","G4","G4","G3","G4","G3","G4","G3","G3",
    "G2","G4","G3","G3","G4","G2","G3","G4","G4","G2","G2","G3",
    "G3","G3","G3","G3","G3","G4","G3","G4","G3","G4","G3","G1",
    "G2","G4","G3","G1","G3","G4","G3","G3","G3","G3","G1","G4",
    "G4","G3","G3","G3","G1","G4","G4","G4","G3","G4","G2","G4",
    "G4","G2","G4","G1","G1","G2","G1","G2","G3","G4","G3","G3",
    "G3","G4","G4","G4","G1","G1","G1","G3","G3","G4","G1","G4",
    "G4","G4","G2","G3","G4","G2","G4","G4","G4","G4","G4","G4"
];

const OBJECTS: [&str; TOTAL_TILES] = [
    "W ","DS","T ","T ","T ","T ","T ","T ","  ","T ","  ","  ",
    "W ","  ","T ","T ","T ","R ","  ","R ","  ","  ","  ","SP",
    "W ","  ","T ","T ","  ","R ","R ","  ","R ","  ","  ","  ",
    "W ","  ","  ","  ","  ","  ","R ","  ","R ","  ","W ","W ",
    "W ","  ","T ","T ","  ","R ","R ","  ","R ","  ","W ","T ",
    "W ","  ","  ","T ","T ","R ","  ","R ","T ","T ","T ","C ",
    "W ","  ","T ","T ","  ","T ","T ","T ","  ","T ","T ","  ",
    "RF","  ","  ","  ","  ","  ","  ","  ","  ","  ","  ","  ",
];

const PLAYER_POSITION: Position = Position(1f64, 11f64);

pub const LEVEL06: ([&str; TOTAL_TILES], [&str; TOTAL_TILES], Position) = (TERRAINS, OBJECTS, PLAYER_POSITION);
