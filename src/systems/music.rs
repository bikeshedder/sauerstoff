use bevy::prelude::{AssetServer, Assets, Commands, Query, Res, ResMut, Transform};
use bevy_kira_audio::{Audio, AudioControl, AudioInstance, AudioTween};

use crate::{
    components::player::{Player, PlayerState},
    resources::{audio::AudioInstances, config::Config},
};

pub fn music_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    config: Res<Config>,
    audio: Res<Audio>,
) {
    let music1 = audio
        .play(asset_server.load("music/base.ogg"))
        .with_volume(config.audio.music_volume as f64)
        .looped()
        .handle();

    let music2 = audio
        .play(asset_server.load("music/crystally.ogg"))
        .with_volume(config.audio.music_volume as f64)
        .looped()
        .handle();

    let sigh = audio
        .play(asset_server.load("sounds/Running-on-Gravel-www.fesliyanstudios.com.ogg"))
        .with_volume(config.audio.effects_volume as f64)
        .looped()
        .handle();

    let footsteps = audio
        .play(asset_server.load("sounds/Sigh-A3-www.fesliyanstudios.com.ogg"))
        .with_volume(config.audio.effects_volume as f64)
        .looped()
        .handle();

    let music = AudioInstances {
        music1,
        music2,
        sigh,
        footsteps,
    };

    commands.insert_resource(music);
}

pub fn music_scene(
    config: Res<Config>,
    query: Query<(&Player, &Transform)>,
    handle: Res<AudioInstances>,
    mut audio_instances: ResMut<Assets<AudioInstance>>,
) {
    // cross fade between the two tracks depending on the current player
    // x translation
    let (player, transform) = query.single();
    let mix = ((300.0 - transform.translation.x) / 600.0).clamp(0.0, 1.0);

    if let Some(instance) = audio_instances.get_mut(&handle.music1) {
        instance.set_volume(
            (mix * config.audio.music_volume) as f64,
            AudioTween::default(),
        );
    }

    if let Some(instance) = audio_instances.get_mut(&handle.music2) {
        instance.set_volume(
            ((1.0 - mix) * config.audio.music_volume) as f64,
            AudioTween::default(),
        );
    }

    if let Some(instance) = audio_instances.get_mut(&handle.sigh) {
        if player.state == PlayerState::Walk {
            instance.set_volume(config.audio.effects_volume as f64, AudioTween::default());
        } else {
            instance.set_volume(0.0, AudioTween::default());
        }
    }
    if let Some(instance) = audio_instances.get_mut(&handle.footsteps) {
        if player.state == PlayerState::Interact {
            instance.set_volume(config.audio.effects_volume as f64, AudioTween::default());
        } else {
            instance.set_volume(0.0, AudioTween::default());
        }
    }
}
