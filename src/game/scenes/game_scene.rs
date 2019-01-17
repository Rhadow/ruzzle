use super::{SceneType, Scene};
use crate::renderer::Renderer;
use crate::game::World;
use crate::canvas::CanvasInputMap;
use crate::game::constants::{
    ARROW_DOWN,
    ARROW_UP,
    ARROW_RIGHT,
    ARROW_LEFT,
    WORLD_WIDTH_IN_TILES,
    WORLD_HEIGHT_IN_TILES,
};
use crate::audio::AudioPlayer;

pub struct GameScene {
    scene_type: SceneType,
}

impl Scene for GameScene {
    fn scene_type(&self) -> &SceneType {
        &self.scene_type
    }
    fn render(&self, renderer: &Renderer, world: &World) {
        let tile_map = world.tile_map();
        let characters = world.get_characters();
        let objects = world.get_objects();
        renderer.clear_screen();

        for row in 0..WORLD_HEIGHT_IN_TILES {
            for col in 0..WORLD_WIDTH_IN_TILES {
                let idx = world.get_index(row, col);
                let tile = &tile_map[idx];
                if let Some(terrain) = tile.borrow().terrain() {
                    renderer.draw_terrain(terrain);
                }
            }
        }
        renderer.draw_objects(objects);
        renderer.draw_characters(characters);
    }
    fn update(&mut self, world: &mut World, input_map: &mut CanvasInputMap, audio: &mut AudioPlayer, now: f64) {
        self.check_direction_event(input_map, world);
        world.update(now, audio);
    }
}

impl GameScene {
    pub fn new() -> GameScene {
        GameScene {
            scene_type: SceneType::Game
        }
    }
    fn check_direction_event(&mut self, input_map: &mut CanvasInputMap, world: &mut World) {
        let key_map = &((*input_map).borrow().key_map);
        let mut direction_key = None;
        let mut most_recent_timestamp = 0f64;
        for (key, &value) in key_map {
            if key == ARROW_DOWN || key == ARROW_UP || key == ARROW_RIGHT || key == ARROW_LEFT {
                if let Some(timestamp) = value {
                    if timestamp > most_recent_timestamp {
                        most_recent_timestamp = timestamp;
                        direction_key = Some(key);
                    }
                }
            }
        }
        if let Some(direction_key) = direction_key {
            world.handle_direction_event(direction_key);
        }
    }
}