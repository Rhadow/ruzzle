mod web_audio_player;
pub use self::web_audio_player::WebAudioPlayer;

#[derive(PartialEq)]
pub enum BGM {
    World0,
    World1,
}

#[derive(PartialEq)]
pub enum SFX {
}