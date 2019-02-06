use crate::game::constants::TOTAL_TILES;
use crate::game::Position;

const TERRAINS: [&str; TOTAL_TILES] = [
    "G4","G4","G4","G4","H ","H ","G3","G4","G3","G4","G4","G4",
    "G3","G1","G4","G4","H ","H ","G3","G4","G4","G1","G4","G4",
    "G4","G1","G3","G3","H ","H ","G3","G4","G1","G4","G3","G4",
    "G4","G3","G4","G3","H ","H ","G4","G3","G3","G3","G1","G3",
    "G1","G3","G4","G3","H ","H ","G3","G4","G4","G4","G4","G2",
    "G3","G4","G4","G3","H ","H ","G4","G3","G1","G3","G3","G3",
    "G4","G4","G3","G3","G4","G4","G4","G4","G2","G4","G4","G1",
    "G3","G1","G4","G4","G2","G3","G3","G3","G3","G3","G4","G4"
];

const OBJECTS: [&str; TOTAL_TILES] = [
    "  ","  ","  ","  ","  ","  ","  ","  ","W ","T ","C ","T ",
    "  ","R ","  ","  ","  ","  ","BF","  ","W ","  ","T ","T ",
    "  ","  ","  ","  ","  ","  ","  ","  ","W ","T ","T ","  ",
    "  ","  ","  ","  ","  ","  ","  ","  ","W ","T ","W ","  ",
    "SP","  ","  ","  ","  ","  ","  ","  ","W ","T ","W ","  ",
    "  ","R ","  ","  ","  ","  ","  ","  ","W ","T ","T ","T ",
    "  ","  ","R ","  ","  ","  ","W ","W ","W ","T ","T ","T ",
    "RS","  ","  ","  ","  ","  ","  ","  ","  ","T ","W ","T ",
];

const PLAYER_POSITION: Position = Position(4f64, 0f64);

pub const LEVEL03: ([&str; TOTAL_TILES], [&str; TOTAL_TILES], Position) = (TERRAINS, OBJECTS, PLAYER_POSITION);
