use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision as BevyCollision},
};

use crate::components::{
    animation::AnimationState,
    collision::Collision,
    player::{InteractDirection, Player, PLAYER_SPEED},
};

#[derive(Debug)]
pub struct PlayerInput {
    pub x: f32,
    pub y: f32,
    pub interact: bool,
    pub back: bool,
}

fn key_to_analog(key: &Res<Input<KeyCode>>, codes: &[KeyCode], value: f32) -> f32 {
    let pressed = codes.iter().any(|&code| key.pressed(code));
    if pressed {
        value
    } else {
        0.0
    }
}

impl PlayerInput {
    pub fn from_keys(key: Res<Input<KeyCode>>) -> Self {
        let key_left = key_to_analog(&key, &[KeyCode::A, KeyCode::Left], -1.0);
        let key_right = key_to_analog(&key, &[KeyCode::D, KeyCode::Right], 1.0);
        let key_up = key_to_analog(&key, &[KeyCode::W, KeyCode::Up], 1.0);
        let key_down = key_to_analog(&key, &[KeyCode::S, KeyCode::Down], -1.0);
        Self {
            x: key_right + key_left,
            y: key_up + key_down,
            interact: key.pressed(KeyCode::Space),
            back: key.just_pressed(KeyCode::Escape),
        }
    }
    pub fn from_gamepad(
        gamepad: Gamepad,
        axis: &Res<Axis<GamepadAxis>>,
        button: &Res<Input<GamepadButton>>,
    ) -> Self {
        let axis_lx = GamepadAxis(gamepad, GamepadAxisType::LeftStickX);
        let axis_ly = GamepadAxis(gamepad, GamepadAxisType::LeftStickY);
        let interact = GamepadButton(gamepad, GamepadButtonType::South);
        let back = GamepadButton(gamepad, GamepadButtonType::East);
        Self {
            x: axis.get(axis_lx).unwrap_or(0.0),
            y: axis.get(axis_ly).unwrap_or(0.0),
            interact: button.pressed(interact),
            back: button.just_pressed(back),
        }
    }
    pub fn merge(&mut self, inputs: impl Iterator<Item = PlayerInput>) {
        for input in inputs {
            self.x += input.x;
            self.y += input.y;
            self.interact |= input.interact;
            self.back |= input.back;
        }
        self.x = self.x.clamp(-1.0, 1.0);
        self.y = self.y.clamp(-1.0, 1.0);
    }
}

pub fn player_input(
    key: Res<Input<KeyCode>>,
    gamepads: Res<Gamepads>,
    gamepad_axis: Res<Axis<GamepadAxis>>,
    gamepad_button: Res<Input<GamepadButton>>,
    mut query: Query<(
        &mut Transform,
        &mut Player,
        &mut AnimationState,
        &mut Collision,
    )>,
    time: Res<Time>,
    collision_query: Query<(&Collision, Without<Player>)>,
) {
    let mut input = PlayerInput::from_keys(key);
    input.merge(
        gamepads
            .iter()
            .map(|gamepad| PlayerInput::from_gamepad(*gamepad, &gamepad_axis, &gamepad_button)),
    );
    let (mut transform, mut player, mut animation, mut player_collision) = query.single_mut();
    let delta = time.delta().as_secs_f32();

    animation.animation = "idle";

    if input.y > 0.0 {
        animation.animation = "walk_up";
    } else if input.y < 0.0 {
        animation.animation = "walk_down";
    }

    if input.x > 0.0 {
        animation.animation = "walk_right";
        player.interact_direction = InteractDirection::Right;
    } else if input.x < 0.0 {
        animation.animation = "walk_left";
        player.interact_direction = InteractDirection::Left;
    }

    if input.interact {
        animation.animation = match player.interact_direction {
            InteractDirection::Left => "interact_left",
            InteractDirection::Right => "interact_right",
        };
    } else {
        transform.translation.x += input.x * PLAYER_SPEED * delta;
        transform.translation.y += input.y * PLAYER_SPEED * delta;
        transform.translation.z = player_collision.update_position(transform.translation);

        // now make sure we're not colliding with anything
        for (entity_collision, _) in collision_query.iter() {
            if let Some(collision) = collide(
                player_collision.pos,
                player_collision.size,
                entity_collision.pos,
                entity_collision.size,
            ) {
                match collision {
                    BevyCollision::Left => {
                        transform.translation.x = entity_collision.pos.x
                            - entity_collision.size.x / 2.0
                            - player_collision.size.x / 2.0
                            - player_collision.origin.x;
                    }
                    BevyCollision::Right => {
                        transform.translation.x = entity_collision.pos.x
                            + entity_collision.size.x / 2.0
                            + player_collision.size.x / 2.0
                            - player_collision.origin.x;
                    }
                    BevyCollision::Top => {
                        transform.translation.y = entity_collision.pos.y
                            + entity_collision.size.y / 2.0
                            + player_collision.size.y / 2.0
                            - player_collision.origin.y;
                    }
                    BevyCollision::Bottom => {
                        transform.translation.y = entity_collision.pos.y
                            - entity_collision.size.y / 2.0
                            - player_collision.size.y / 2.0
                            - player_collision.origin.y;
                    }
                }
            }
        }
    }
}
