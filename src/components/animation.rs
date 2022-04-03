use std::time::Duration;

use bevy::{prelude::Component, utils::HashMap};

#[derive(Component, Debug)]
pub struct Animation {
    pub frames: HashMap<String, Vec<(usize, Duration)>>,
}

#[derive(Component, Debug)]
pub struct AnimationState {
    pub animation: &'static str,
    pub index: usize,
}
