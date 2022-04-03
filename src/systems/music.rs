use bevy::prelude::{AssetServer, Res};
use bevy_kira_audio::Audio;

pub fn music_system(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    audio.play_looped(asset_server.load("music/base.mp3"));
}
