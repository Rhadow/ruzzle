use std::cell::RefCell;
use super::terrain::Terrain;
use super::object::Object;

pub struct Tile {
    terrain: Option<Box<dyn Terrain>>,
    object: Option<RefCell<Box<dyn Object>>>,
}

impl Tile {
    pub fn new(terrain: Option<Box<dyn Terrain>>, object: Option<Box<dyn Object>>) -> Tile {
        let new_object = match object {
            Some(o) => Some(RefCell::new(o)),
            None => None
        };
        Tile {
            terrain,
            object: new_object,
        }
    }

    pub fn get_terrain(&self) -> &Option<Box<dyn Terrain>> {
        &self.terrain
    }

    pub fn get_object(&self) -> &Option<RefCell<Box<dyn Object>>> {
        &self.object
    }
}