use bevy::{
    math::vec3,
    prelude::{Query, Res, Transform, Without},
    time::Time,
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

    if let Some(direction) = player.primary_direction() {
        player.direction = direction;
        match direction {
            PlayerDirection::Left => {
                player.state = PlayerState::Walk;
                player.interact_direction = InteractDirection::Left;
            }
            PlayerDirection::Right => {
                player.state = PlayerState::Walk;
                player.interact_direction = InteractDirection::Right;
            }
            PlayerDirection::Up => {
                player.state = PlayerState::Walk;
            }
            PlayerDirection::Down => {
                player.state = PlayerState::Walk;
            }
        }
    } else {
        player.state = PlayerState::Idle;
    }

    if player.input.interact {
        player.state = PlayerState::Interact;
        // FIXME once there are animations for interact_up and
        // interact_down the interact_direction is rendered obsolete
        // and this hack can go away.
        player.direction = match player.interact_direction {
            InteractDirection::Right => PlayerDirection::Right,
            InteractDirection::Left => PlayerDirection::Left,
        }
    }

    if player.state == PlayerState::Walk {
        // move player to new position
        let mut new_translation = vec3(
            transform.translation.x + player.input.x * PLAYER_SPEED * delta,
            transform.translation.y + player.input.y * PLAYER_SPEED * delta,
            0.0,
        );

        // now make sure we're not colliding with anything
        for (entity_collision, _) in collision_query.iter() {
            if let Some(trans) = player_collision.collide(new_translation, entity_collision) {
                new_translation = trans;
            }
        }

        // check collision with the world
        if let Some(trans) = map_collision.collide(transform.translation, new_translation) {
            new_translation = trans;
        }

        // update player position
        transform.translation = vec3(
            new_translation.x,
            new_translation.y,
            player_collision.update_position(new_translation),
        );
    }

    animation.start(match (player.state, player.direction) {
        (PlayerState::Idle, _) => "idle",
        (PlayerState::Walk, PlayerDirection::Right) => "walk_right",
        (PlayerState::Walk, PlayerDirection::Left) => "walk_left",
        (PlayerState::Walk, PlayerDirection::Up) => "walk_up",
        (PlayerState::Walk, PlayerDirection::Down) => "walk_down",
        (PlayerState::Interact, PlayerDirection::Left) => "interact_left",
        (PlayerState::Interact, PlayerDirection::Right) => "interact_right",
        (PlayerState::Interact, PlayerDirection::Up) => "interact_up",
        (PlayerState::Interact, PlayerDirection::Down) => "interact_down",
    });
}
