use bevy::prelude::{AssetServer, Res, ResMut};
use bevy_kira_audio::{Audio, AudioChannel};

use crate::resources::{audio::AudioChannels, config::Config};

pub fn music_system(
    mut channels: ResMut<AudioChannels>,
    asset_server: Res<AssetServer>,
    config: Res<Config>,
    audio: Res<Audio>,
) {
    channels.music = AudioChannel::new("music".to_owned());
    audio.set_volume_in_channel(config.audio.music_volume, &channels.music);

    audio.play_looped_in_channel(asset_server.load("music/base.mp3"), &channels.music);
}
