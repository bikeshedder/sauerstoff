use bevy::prelude::Handle;
use bevy_kira_audio::AudioInstance;

pub struct AudioInstances {
    pub music1: Handle<AudioInstance>,
    pub music2: Handle<AudioInstance>,
    pub sigh: Handle<AudioInstance>,
    pub footsteps: Handle<AudioInstance>,
}
