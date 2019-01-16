// use web_sys::console::log_1;
use crate::web_client::WebAssets;
use std::collections::HashMap;
use wasm_bindgen::JsCast;
use web_sys::HtmlAudioElement;
use super::{BGM, SFX, AudioPlayer};

pub struct WebAudioPlayer {
    now_playing_bgm: Option<BGM>,
    bgm_map: HashMap<BGM, HtmlAudioElement>,
    sfx_map: HashMap<SFX, HtmlAudioElement>,
}

impl WebAudioPlayer {
    pub fn new(asset: &WebAssets) -> WebAudioPlayer {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let id_bgm_map = WebAudioPlayer::init_id_bgm_map();
        let id_sfx_map = WebAudioPlayer::init_id_sfx_map();
        let mut bgm_map: HashMap<BGM, HtmlAudioElement> = HashMap::new();
        let mut sfx_map: HashMap<SFX, HtmlAudioElement> = HashMap::new();

        for bgm_id in &asset.bgm {
            let bgm = id_bgm_map.get(bgm_id).unwrap();
            let bgm_element: HtmlAudioElement = document.get_element_by_id(bgm_id).unwrap().dyn_into().unwrap();
            bgm_map.insert(*bgm, bgm_element);
        }

        for sfx_id in &asset.sfx {
            let sfx = id_sfx_map.get(sfx_id).unwrap();
            let sfx_element: HtmlAudioElement = document.get_element_by_id(sfx_id).unwrap().dyn_into().unwrap();
            sfx_map.insert(*sfx, sfx_element);
        }

        WebAudioPlayer {
            now_playing_bgm: None,
            bgm_map,
            sfx_map,
        }
    }

    fn init_id_bgm_map() -> HashMap<String, BGM> {
        let mut bgm_map: HashMap<String, BGM> = HashMap::new();
        bgm_map.insert(String::from("bgm_0"), BGM::World0);
        bgm_map.insert(String::from("bgm_1"), BGM::World1);
        bgm_map
    }

    fn init_id_sfx_map() -> HashMap<String, SFX> {
        let mut sfx_map: HashMap<String, SFX> = HashMap::new();
        sfx_map.insert(String::from("sfx_rock_fall"), SFX::RockFall);
        sfx_map.insert(String::from("sfx_rock_move"), SFX::RockMove);
        sfx_map.insert(String::from("sfx_dead"), SFX::Dead);
        sfx_map
    }
}

impl AudioPlayer for WebAudioPlayer {
    fn play_bgm(&mut self, bgm: BGM) {
        let mut is_bgm_already_playing = false;
        if let Some(ref playing) = self.now_playing_bgm {
            if *playing != bgm {
                let bgm_to_pause = self.bgm_map.get(&playing).unwrap();
                bgm_to_pause.pause().unwrap();
            } else {
                is_bgm_already_playing = true;
            }
        }
        if !is_bgm_already_playing {
            let bgm_to_play = self.bgm_map.get(&bgm).unwrap();
            bgm_to_play.set_current_time(0f64);
            bgm_to_play.play().unwrap();
            self.now_playing_bgm = Some(bgm);
        }
    }

    fn play_sfx(&mut self, sfx: SFX) {
        let target_sfx = self.sfx_map.get(&sfx).unwrap();
        if target_sfx.current_time() == 0f64 || target_sfx.ended() {
            target_sfx.set_current_time(0f64);
            target_sfx.play().unwrap();
        }
    }
}