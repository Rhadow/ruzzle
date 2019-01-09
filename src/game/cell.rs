use super::terrain::Terrain;
use super::object::Object;

pub struct Cell {
    terrain: Option<Box<dyn Terrain>>,
    object: Option<Box<dyn Object>>,
}

impl Cell {
    pub fn new(terrain: Option<Box<dyn Terrain>>, object: Option<Box<dyn Object>>) -> Cell {
        Cell {
            terrain,
            object,
        }
    }

    pub fn get_terrain(&self) -> &Option<Box<dyn Terrain>> {
        &self.terrain
    }

    pub fn get_object(&self) -> &Option<Box<dyn Object>> {
        &self.object
    }
}