
use super::terrain::Terrain;

pub struct Tile {
    terrain: Option<Box<dyn Terrain>>,
}

impl Tile {
    pub fn new(terrain: Option<Box<dyn Terrain>>) -> Tile {
        Tile {
            terrain,
        }
    }

    pub fn get_terrain(&self) -> &Option<Box<dyn Terrain>> {
        &self.terrain
    }
}