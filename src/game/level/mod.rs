mod level00;

use super::Cell;
use super::terrain::{
    Terrain,
    GrassLand,
    WoodenPath,
};
use super::object::{Object, Tree};
use super::constants::TOTAL_CELLS;
use self::level00::LEVEL00;

type Level = ([&'static str; TOTAL_CELLS], [&'static str; TOTAL_CELLS]);
const LEVELS: [Level; 1] = [
    LEVEL00,
];

pub struct LevelManager {}

impl LevelManager {
    pub fn new() -> LevelManager {
        LevelManager {}
    }

    pub fn construct_level(&self, level: usize) -> Vec<Cell> {
        let mut cells = vec![];
        let (terrains, objects) = LEVELS[level];
        for i in 0..terrains.len() {
            let terrain = terrains[i];
            let object = objects[i];

            let terrain: Option<Box<dyn Terrain>> = match String::from(terrain.trim()).to_uppercase().as_str() {
                "G" => Some(Box::new(GrassLand::new())),
                "WP" => Some(Box::new(WoodenPath::new())),
                _ => None
            };

            let object: Option<Box<dyn Object>> = match String::from(object.trim()).to_uppercase().as_str() {
                "T" => Some(Box::new(Tree::new())),
                _ => None
            };

            cells.push(Cell::new(terrain, object));
        }
        cells
    }
}