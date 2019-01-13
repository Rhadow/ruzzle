use std::cell::RefCell;
use crate::game::World;
use super::terrain::Terrain;

pub struct Tile {
    terrain: Option<RefCell<Box<dyn Terrain>>>,
}

impl Tile {
    pub fn new(terrain: Option<RefCell<Box<dyn Terrain>>>) -> Tile {
        Tile {
            terrain,
        }
    }

    pub fn terrain(&self) -> &Option<RefCell<Box<dyn Terrain>>> {
        &self.terrain
    }

    pub fn update(&mut self, now: f64, world: &World) {
        if let Some(ref mut terrain) = self.terrain {
            terrain.borrow_mut().update(now, world);
        }
    }
}