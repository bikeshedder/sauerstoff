use bevy::{
    core::{Time, Timer},
    prelude::{Query, Res},
    sprite::TextureAtlasSprite,
};

use crate::components::animation::{Animation, AnimationState};

pub fn animation_system(
    time: Res<Time>,
    mut query: Query<(
        &mut Timer,
        &mut TextureAtlasSprite,
        &Animation,
        &mut AnimationState,
    )>,
) {
    for (mut timer, mut sprite, animation, mut state) in query.iter_mut() {
        let update = if state.restart {
            state.restart = false;
            true
        } else {
            timer.tick(time.delta());
            timer.finished()
        };
        if update {
            let frames = &animation.frames[state.animation];
            state.index = (state.index + 1) % frames.len();
            let (atlas_index, duration) = frames[state.index];
            sprite.index = atlas_index;
            timer.set_duration(duration);
        }
    }
}
