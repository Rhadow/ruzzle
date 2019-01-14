use wasm_bindgen::JsCast;
use web_sys::HtmlAudioElement;
use super::BGM;

pub struct WebAudioPlayer {
    now_playing: Option<BGM>,
    world_0: HtmlAudioElement,
    world_1: HtmlAudioElement,
}

impl WebAudioPlayer {
    pub fn new(
        world_0_id: &str,
        world_1_id: &str,
    ) -> WebAudioPlayer {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let world_0: HtmlAudioElement = document.get_element_by_id(world_0_id).unwrap().dyn_into().unwrap();
        let world_1: HtmlAudioElement = document.get_element_by_id(world_1_id).unwrap().dyn_into().unwrap();
        WebAudioPlayer {
            now_playing: None,
            world_0: world_0,
            world_1: world_1,
        }
    }

    pub fn play_bgm(&mut self, bgm: BGM) {
        if let Some(ref playing) = self.now_playing {
            match playing {
                BGM::World0 => self.world_0.pause(),
                BGM::World1 => self.world_1.pause(),
            }.unwrap();
        }
        match bgm {
            BGM::World0 => self.now_playing = Some(BGM::World0),
            BGM::World1 => self.now_playing = Some(BGM::World1),
        }
        if let Some(ref playing) = self.now_playing {
            match playing {
                BGM::World0 => {
                    self.world_0.fast_seek(0f64);
                    self.world_0.play();
                },
                BGM::World1 => {
                    self.world_1.fast_seek(0f64);
                    self.world_1.play();
                },
            };
        }
    }
}