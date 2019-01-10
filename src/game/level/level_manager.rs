/*
    Environment:
    "G": GrassLand,
    "WP": WoodenPath,

    Objects:
    "T": Tree
*/

use crate::game::{
    Cell,
    Coordinate
};
use crate::game::terrain::{
    Terrain,
    GrassLand,
    WoodenPath,
};
use crate::game::object::{
    Object,
    Tree
};
use crate::game::constants::{
    WORLD_WIDTH_IN_CELLS,
    TOTAL_CELLS,
};
use super::level00::LEVEL00;

type Level = ([&'static str; TOTAL_CELLS], [&'static str; TOTAL_CELLS], Coordinate);
const LEVELS: [Level; 1] = [
    LEVEL00,
];

pub struct LevelManager {
    player_coordinate: Option<Coordinate>
}

impl LevelManager {
    pub fn new() -> LevelManager {
        LevelManager {
            player_coordinate: None
        }
    }

    fn index_to_coordinate(&self, idx: usize) -> Coordinate {
        let row = (idx as i32 / WORLD_WIDTH_IN_CELLS as i32) as usize;
        let col = (idx as i32 % WORLD_WIDTH_IN_CELLS as i32) as usize;
        Coordinate(row, col)
    }

    pub fn get_player_coordinate(&self) -> &Option<Coordinate> {
        &self.player_coordinate
    }

    pub fn set_player_coordinate(&mut self, new_coordinate: Option<Coordinate>) {
        self.player_coordinate = new_coordinate;
    }

    pub fn construct_level(&mut self, level: usize) -> Vec<Cell> {
        let mut cells = vec![];
        let (terrains, objects, player_coordinate) = LEVELS[level];
        self.set_player_coordinate(Some(player_coordinate));
        for i in 0..terrains.len() {
            let terrain = terrains[i];
            let object = objects[i];
            let coordinate = self.index_to_coordinate(i);

            let terrain: Option<Box<dyn Terrain>> = match String::from(terrain.trim()).to_uppercase().as_str() {
                "G" => Some(Box::new(GrassLand::new())),
                "WP" => Some(Box::new(WoodenPath::new())),
                _ => None
            };

            let object: Option<Box<dyn Object>> = match String::from(object.trim()).to_uppercase().as_str() {
                "T" => Some(Box::new(Tree::new(coordinate))),
                _ => None
            };

            cells.push(Cell::new(terrain, object));
        }
        cells
    }
}