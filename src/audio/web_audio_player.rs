use wasm_bindgen::JsCast;
use web_sys::HtmlAudioElement;
use super::BGM;

pub struct WebAudioPlayer {
    now_playing_bgm: Option<BGM>,
    bgm_0: HtmlAudioElement,
    bgm_1: HtmlAudioElement,
}

impl WebAudioPlayer {
    pub fn new(
        bgm_0: &str,
        bgm_1: &str,
    ) -> WebAudioPlayer {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let bgm_0: HtmlAudioElement = document.get_element_by_id(bgm_0).unwrap().dyn_into().unwrap();
        let bgm_1: HtmlAudioElement = document.get_element_by_id(bgm_1).unwrap().dyn_into().unwrap();
        WebAudioPlayer {
            now_playing_bgm: None,
            bgm_0: bgm_0,
            bgm_1: bgm_1,
        }
    }

    pub fn play_bgm(&mut self, bgm: BGM) {
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
        match bgm {
            BGM::World0 => {
                if !is_bgm_already_playing {
                    self.bgm_0.set_current_time(0f64);
                    self.bgm_0.play().unwrap();
                    self.now_playing_bgm = Some(BGM::World0);
                }
            },
            BGM::World1 => {
                if !is_bgm_already_playing {
                    self.bgm_1.set_current_time(0f64);
                    self.bgm_1.play().unwrap();
                    self.now_playing_bgm = Some(BGM::World1);
                }
            },
        }
    }
}