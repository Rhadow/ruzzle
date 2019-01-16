mod web_audio_player;
pub use self::web_audio_player::WebAudioPlayer;

#[derive(PartialEq, Copy, Clone, Eq, Hash)]
pub enum BGM {
    World0,
    World1,
}

#[derive(PartialEq, Debug, Copy, Clone, Eq, Hash)]
pub enum SFX {
    RockFall,
    RockMove,
    Dead,
}

pub trait AudioPlayer {
    fn play_bgm(&mut self, bgm: BGM);
    fn play_sfx(&mut self, sfx: SFX);
}