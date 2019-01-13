use crate::game::World;
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

    pub fn update(&mut self, now: f64, world: &World) {
        if let Some(ref mut terrain) = self.terrain {
            terrain.update(now, world);
        }
    }
}