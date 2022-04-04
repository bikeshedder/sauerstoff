use bevy::{
    prelude::{AssetServer, Query, Res, ResMut, Transform},
    transform,
};
use bevy_kira_audio::{Audio, AudioChannel};

use crate::{
    components::player::{Player, PlayerState},
    resources::{audio::AudioChannels, config::Config},
};

pub fn music_system(
    mut channels: ResMut<AudioChannels>,
    asset_server: Res<AssetServer>,
    config: Res<Config>,
    audio: Res<Audio>,
) {
    channels.music_1 = AudioChannel::new("music_1".to_owned());
    channels.music_2 = AudioChannel::new("music_2".to_owned());
    channels.footsteps = AudioChannel::new("footsteps".to_owned());
    channels.sigh = AudioChannel::new("sigh".to_owned());

    audio.set_volume_in_channel(config.audio.music_volume, &channels.music_1);
    audio.set_volume_in_channel(config.audio.music_volume, &channels.music_2);

    audio.play_looped_in_channel(asset_server.load("music/base.mp3"), &channels.music_1);
    audio.play_looped_in_channel(asset_server.load("music/crystally.mp3"), &channels.music_2);
    audio.play_looped_in_channel(
        asset_server.load("sounds/Running-on-Gravel-www.fesliyanstudios.com.mp3"),
        &channels.footsteps,
    );
    audio.play_looped_in_channel(
        asset_server.load("sounds/Sigh-A3-www.fesliyanstudios.com.mp3"),
        &channels.sigh,
    );
}

pub fn music_scene(
    channels: ResMut<AudioChannels>,
    config: Res<Config>,
    audio: Res<Audio>,
    query: Query<(&Player, &Transform)>,
) {
    // cross fade between the two tracks depending on the current player
    // x translation
    let (player, transform) = query.single();
    let mix = ((300.0 - transform.translation.x) / 600.0).clamp(0.0, 1.0);
    audio.set_volume_in_channel(mix * config.audio.music_volume, &channels.music_1);
    audio.set_volume_in_channel((1.0 - mix) * config.audio.music_volume, &channels.music_2);
    if player.state == PlayerState::Walk {
        audio.set_volume_in_channel(config.audio.effects_volume, &channels.footsteps);
    } else {
        audio.set_volume_in_channel(0.0, &channels.footsteps);
    }
    if player.state == PlayerState::Interact {
        audio.set_volume_in_channel(config.audio.effects_volume, &channels.sigh);
    } else {
        audio.set_volume_in_channel(0.0, &channels.sigh);
    }
}
