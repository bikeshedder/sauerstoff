use std::cmp::Ordering;

use bevy::{
    input::{Axis, Input},
    math::Vec3,
    prelude::{
        Component, Gamepad, GamepadAxis, GamepadAxisType, GamepadButton, GamepadButtonType,
        KeyCode, Res,
    },
};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PlayerState {
    Idle,
    Walk,
    Interact,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum InteractDirection {
    Left,
    Right,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PlayerDirection {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, Default)]
pub struct PlayerInput {
    pub x: f32,
    pub y: f32,
    pub interact: bool,
    pub back: bool,
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
        let axis_dx = GamepadAxis(gamepad, GamepadAxisType::DPadX);
        let axis_dy = GamepadAxis(gamepad, GamepadAxisType::DPadY);
        let interact = GamepadButton(gamepad, GamepadButtonType::South);
        let back = GamepadButton(gamepad, GamepadButtonType::East);
        Self {
            x: (axis.get(axis_lx).unwrap_or(0.0) + axis.get(axis_dx).unwrap_or(0.0))
                .clamp(-1.0, 1.0),
            y: (axis.get(axis_ly).unwrap_or(0.0) + axis.get(axis_dy).unwrap_or(0.0))
                .clamp(-1.0, 1.0),
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

fn key_to_analog(key: &Res<Input<KeyCode>>, codes: &[KeyCode], value: f32) -> f32 {
    let pressed = codes.iter().any(|&code| key.pressed(code));
    if pressed {
        value
    } else {
        0.0
    }
}

#[derive(Component, Debug)]
pub struct Player {
    pub input: PlayerInput,
    pub state: PlayerState,
    pub interact_direction: InteractDirection,
    pub direction: PlayerDirection,
    pub center: Vec3,
}

impl Player {
    pub fn primary_direction(&self) -> Option<PlayerDirection> {
        self.input
            .x
            .abs()
            .partial_cmp(&self.input.y.abs())
            .and_then(|ord| match ord {
                Ordering::Less => self.input.y.partial_cmp(&0.0).and_then(|ord| match ord {
                    Ordering::Less => Some(PlayerDirection::Down),
                    Ordering::Greater => Some(PlayerDirection::Up),
                    _ => None,
                }),
                _ => self.input.x.partial_cmp(&0.0).and_then(|ord| match ord {
                    Ordering::Less => Some(PlayerDirection::Left),
                    Ordering::Greater => Some(PlayerDirection::Right),
                    _ => None,
                }),
            })
    }
}

impl Default for Player {
    fn default() -> Self {
        Self {
            input: PlayerInput::default(),
            state: PlayerState::Idle,
            direction: PlayerDirection::Right,
            interact_direction: InteractDirection::Right,
            center: Vec3::new(0.0, -40.0, 0.0),
        }
    }
}

#[test]
fn test_primary_direction() {
    for (x, y, dir) in &[
        (0.0, 0.0, None),
        (1.0, 0.0, Some(PlayerDirection::Right)),
        (1.0, 0.5, Some(PlayerDirection::Right)),
        (-1.0, 0.0, Some(PlayerDirection::Left)),
        (-1.0, 0.5, Some(PlayerDirection::Left)),
        (0.0, 1.0, Some(PlayerDirection::Up)),
        (0.5, 1.0, Some(PlayerDirection::Up)),
        (0.0, -1.0, Some(PlayerDirection::Down)),
        (0.5, -1.0, Some(PlayerDirection::Down)),
    ] {
        assert_eq!(
            Player {
                input: PlayerInput {
                    x: *x,
                    y: *y,
                    ..Default::default()
                },
                ..Default::default()
            }
            .primary_direction(),
            *dir,
        )
    }
}
