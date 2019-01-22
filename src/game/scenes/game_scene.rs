// use web_sys::console::log_1;
use super::{SceneType, Scene};
use crate::renderer::Renderer;
use crate::game::World;
use crate::controller::Controller;
use crate::game::constants::{
    ARROW_DOWN,
    ARROW_UP,
    ARROW_RIGHT,
    ARROW_LEFT,
    WORLD_WIDTH_IN_TILES,
    WORLD_HEIGHT_IN_TILES,
    ACTION_KEY,
};
use crate::audio::AudioPlayer;

pub struct GameScene {
    scene_type: SceneType,
    next_scene_type: Option<SceneType>,
    action_timestamp: f64,
}

impl Scene for GameScene {
    fn scene_type(&self) -> &SceneType {
        &self.scene_type
    }
    fn render(&self, renderer: &Renderer, world: &World, _completed_levels: &Vec<bool>) {
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
    fn update(&mut self, world: &mut World, controller: &mut Controller, audio: &mut Box<dyn AudioPlayer>, completed_levels: &mut Vec<bool>, now: f64) {
        if world.is_completed {
            if let Some(level_number) = world.level_number {
                if !completed_levels[level_number] {
                    completed_levels[level_number] = true;
                }
            }
            self.set_next_scene_type(SceneType::LevelSelection);
        } else {
            self.check_direction_event(controller, world);
            self.check_action_event(controller, world);
            world.update(now, audio);
        }
    }
    fn next_scene_type(&self) -> &Option<SceneType> {
        &self.next_scene_type
    }
    fn set_next_scene_type(&mut self, scene_type: SceneType) {
        self.next_scene_type = Some(scene_type);
    }
}

impl GameScene {
    pub fn new() -> GameScene {
        GameScene {
            scene_type: SceneType::Game,
            next_scene_type: None,
            action_timestamp: 0f64,
        }
    }
    fn check_direction_event(&mut self, controller: &mut Controller, world: &mut World) {
        let key_map = &controller.key_map;
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
    fn check_action_event(&mut self, controller: &mut Controller, world: &mut World) {
        if let Some(value) = controller.key_map.get(ACTION_KEY) {
            if let Some(timestamp) = value {
                if *timestamp != self.action_timestamp {
                    world.handle_action_event(ACTION_KEY);
                }
                self.action_timestamp = *timestamp;
            }
        }
    }
}