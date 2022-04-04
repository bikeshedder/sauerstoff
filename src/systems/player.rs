use std::cmp::Ordering;

use bevy::{
    core::Time,
    prelude::{Query, Res, Transform, Without},
};

use crate::{
    components::{
        animation::AnimationState,
        collision::Collision,
        player::{InteractDirection, Player, PlayerDirection, PlayerState},
    },
    resources::map::Map,
};

pub const PLAYER_SPEED: f32 = 600.0;

pub fn player_system(
    time: Res<Time>,
    mut query: Query<(
        &mut Player,
        &mut Transform,
        &mut AnimationState,
        &mut Collision,
    )>,
    collision_query: Query<(&Collision, Without<Player>)>,
    map_collision: Res<Map>,
) {
    let (mut player, mut transform, mut animation, mut player_collision) = query.single_mut();
    let delta = time.delta().as_secs_f32();

    let mut new_animation = match player.primary_direction() {
        Some(PlayerDirection::Left) => {
            player.interact_direction = InteractDirection::Left;
            "walk_left"
        }
        Some(PlayerDirection::Right) => {
            player.interact_direction = InteractDirection::Right;
            "walk_right"
        }
        Some(PlayerDirection::Up) => "walk_up",
        Some(PlayerDirection::Down) => "walk_down",
        _ => "idle",
    };

    let old_translation = transform.translation;

    if player.input.interact {
        new_animation = match player.interact_direction {
            InteractDirection::Left => "interact_left",
            InteractDirection::Right => "interact_right",
        };
        animation.start(new_animation);
        return;
    }

    animation.start(new_animation);

    if player.input.x == 0.0 && player.input.y == 0.0 {
        return;
    }

    transform.translation.x += player.input.x * PLAYER_SPEED * delta;
    transform.translation.y += player.input.y * PLAYER_SPEED * delta;
    transform.translation.z = player_collision.update_position(transform.translation);

    // now make sure we're not colliding with anything
    for (entity_collision, _) in collision_query.iter() {
        if let Some(trans) = player_collision.collide(transform.translation, entity_collision) {
            transform.translation = trans;
        }
    }

    // check collision with the world
    if let Some(translation) = map_collision.collide(old_translation, transform.translation) {
        transform.translation = translation;
        transform.translation.z = player_collision.update_position(transform.translation);
    }
}
