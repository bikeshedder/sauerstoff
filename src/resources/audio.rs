use bevy::prelude::{Handle, Resource};
use bevy_kira_audio::AudioInstance;

#[derive(Resource)]
pub struct AudioInstances {
    pub music1: Handle<AudioInstance>,
    pub music2: Handle<AudioInstance>,
    pub sigh: Handle<AudioInstance>,
    pub footsteps: Handle<AudioInstance>,
}
