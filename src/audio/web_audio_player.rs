// use web_sys::console::log_1;
use wasm_bindgen::JsCast;
use web_sys::HtmlAudioElement;
use super::{BGM, SFX, AudioPlayer};

pub struct WebAudioPlayer {
    now_playing_bgm: Option<BGM>,
    bgm_0: HtmlAudioElement,
    bgm_1: HtmlAudioElement,
    sfx_rock_fall: HtmlAudioElement,
    sfx_rock_move: HtmlAudioElement,
    sfx_dead: HtmlAudioElement,
}

impl WebAudioPlayer {
    pub fn new(
        bgm_0: &str,
        bgm_1: &str,
        sfx_rock_fall: &str,
        sfx_rock_move: &str,
        sfx_dead: &str,
    ) -> WebAudioPlayer {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let bgm_0: HtmlAudioElement = document.get_element_by_id(bgm_0).unwrap().dyn_into().unwrap();
        let bgm_1: HtmlAudioElement = document.get_element_by_id(bgm_1).unwrap().dyn_into().unwrap();
        let sfx_rock_fall: HtmlAudioElement = document.get_element_by_id(sfx_rock_fall).unwrap().dyn_into().unwrap();
        let sfx_rock_move: HtmlAudioElement = document.get_element_by_id(sfx_rock_move).unwrap().dyn_into().unwrap();
        let sfx_dead: HtmlAudioElement = document.get_element_by_id(sfx_dead).unwrap().dyn_into().unwrap();
        WebAudioPlayer {
            now_playing_bgm: None,
            bgm_0,
            bgm_1,
            sfx_rock_fall,
            sfx_rock_move,
            sfx_dead,
        }
    }
}

impl AudioPlayer for WebAudioPlayer {
    fn play_bgm(&mut self, bgm: BGM) {
        let mut is_bgm_already_playing = false;
        if let Some(ref playing) = self.now_playing_bgm {
            if *playing != bgm {
                match playing {
                    BGM::World0 => self.bgm_0.pause(),
                    BGM::World1 => self.bgm_1.pause(),
                }.unwrap();
            } else {
                is_bgm_already_playing = true;
            }
        }
        if !is_bgm_already_playing {
            let bgm_to_play = match bgm {
                BGM::World0 => &self.bgm_0,
                BGM::World1 => &self.bgm_1,
            };
            bgm_to_play.set_current_time(0f64);
            bgm_to_play.play().unwrap();
            self.now_playing_bgm = Some(bgm);
        }
    }

    fn play_sfx(&mut self, sfx: SFX) {
        let target_sfx = match sfx {
            SFX::RockFall => &self.sfx_rock_fall,
            SFX::RockMove => &self.sfx_rock_move,
            SFX::Dead => &self.sfx_dead,
        };

        if target_sfx.current_time() == 0f64 || target_sfx.ended() {
            target_sfx.set_current_time(0f64);
            target_sfx.play().unwrap();
        }
    }
}