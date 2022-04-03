use std::cmp::Ordering;

use bevy::{
    core::Time,
    prelude::{Query, Res, Transform, Without},
};

use crate::components::{
    animation::AnimationState,
    collision::Collision,
    player::{InteractDirection, Player, PlayerDirection, PlayerState},
};

pub const PLAYER_SPEED: f32 = 300.0;

pub fn player_system(
    time: Res<Time>,
    mut query: Query<(
        &mut Player,
        &mut Transform,
        &mut AnimationState,
        &mut Collision,
    )>,
    collision_query: Query<(&Collision, Without<Player>)>,
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

    if player.input.interact {
        new_animation = match player.interact_direction {
            InteractDirection::Left => "interact_left",
            InteractDirection::Right => "interact_right",
        };
    } else {
        transform.translation.x += player.input.x * PLAYER_SPEED * delta;
        transform.translation.y += player.input.y * PLAYER_SPEED * delta;
        transform.translation.z = player_collision.update_position(transform.translation);

        // now make sure we're not colliding with anything
        for (entity_collision, _) in collision_query.iter() {
            if let Some(trans) = player_collision.collide(&transform.translation, entity_collision)
            {
                transform.translation = trans;
            }
        }
    }

    animation.start(new_animation);
}
