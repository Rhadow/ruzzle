use crate::game::constants::TOTAL_TILES;
use crate::game::Position;

const TERRAINS: [&str; TOTAL_TILES] = [
    "G4","G4","G4","G4","G4","G4","G3","G4","G3","G4","G4","G4",
    "G3","G1","G4","G4","G4","G3","G3","G4","G4","G1","G4","G4",
    "G4","G1","G3","G3","G3","G4","G3","G4","G1","G4","G3","G4",
    "G4","G3","G4","G3","G4","G3","G4","G3","G3","G3","G1","G3",
    "G1","G3","G4","G3","G4","G4","G3","G4","G4","G4","G4","G2",
    "G3","G4","G4","G3","G4","G3","G4","G3","G1","G3","G3","G3",
    "G4","G4","G3","G3","G3","G4","G4","G4","G2","G4","G4","G1",
    "G3","G1","G4","G4","G2","G3","G3","G3","G3","G3","G4","G4"
];

const OBJECTS: [&str; TOTAL_TILES] = [
    "  ","  ","DF","  ","  ","  ","DF ","  ","  ","  ","  ","T ",
    "  ","R ","  ","  ","  ","  ","  ","  ","  ","  ","T ","T ",
    "  ","  ","  ","  ","  ","  ","  ","  ","  ","  ","T ","  ",
    "  ","  ","  ","  ","  ","R ","  ","  ","LS","  ","  ","C ",
    "  ","  ","  ","  ","  ","R ","  ","  ","LS","  ","  ","  ",
    "  ","  ","  ","  ","  ","  ","  ","  ","  ","  ","T ","  ",
    "  ","  ","R ","  ","  ","  ","  ","  ","  ","  ","  ","T ",
    "SP","  ","  ","UF","  ","  ","UF","  ","  ","  ","  ","T ",
];

const PLAYER_POSITION: Position = Position(4f64, 0f64);

pub const LEVEL04: ([&str; TOTAL_TILES], [&str; TOTAL_TILES], Position) = (TERRAINS, OBJECTS, PLAYER_POSITION);
