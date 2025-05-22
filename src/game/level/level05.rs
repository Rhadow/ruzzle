use crate::game::constants::TOTAL_TILES;
use crate::game::Position;

const TERRAINS: [&str; TOTAL_TILES] = [
    "G1","G2","G1","G2","G1","G2","G1","G2","G1","G2","G1","G2",
    "G3","G4","G3","G4","G3","G4","G3","G4","G3","G4","G3","G4",
    "G1","G2","G1","G2","G1","G2","G1","G2","G1","G2","G1","G2",
    "G3","G4","G3","G4","G3","G4","G3","G4","G3","G4","G3","G4",
    "G1","G2","G1","G2","G1","G2","G1","G2","G1","G2","G1","G2",
    "G3","G4","G3","G4","G3","G4","G3","G4","G3","G4","G3","G4",
    "G1","G2","G1","G2","G1","G2","G1","G2","G1","G2","G1","G2",
    "G3","G4","G3","G4","G3","G4","G3","G4","G3","G4","G3","G4"
];

const OBJECTS: [&str; TOTAL_TILES] = [
    "W ","W ","W ","W ","W ","W ","W ","W ","W ","W ","W ","W ",
    "W ","  ","  ","R ","BW","BW","BW","R ","  ","  ","CL","W ",
    "W ","  ","H ","T ","BW","C ","BW","T ","CD","  ","  ","W ",
    "W ","R ","T ","  ","BW","BW","BW","  ","T ","R ","  ","W ",
    "W ","  ","  ","R ","  ","FS","  ","R ","  ","  ","CR","W ",
    "W ","T ","BW","BW","BW","BW","BW","BW","BW","T ","  ","W ",
    "W ","  ","R ","  ","  ","CU","  ","  ","R ","  ","RF","W ",
    "W ","W ","W ","W ","W ","W ","W ","W ","W ","W ","W ","W "
];

const PLAYER_POSITION: Position = Position(1f64, 1f64); // Corresponds to (row 1, col 1) -> OBJECTS[1*12+1]

pub const LEVEL05: ([&str; TOTAL_TILES], [&str; TOTAL_TILES], Position) = (TERRAINS, OBJECTS, PLAYER_POSITION);
