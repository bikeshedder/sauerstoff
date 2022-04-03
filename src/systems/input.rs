use bevy::prelude::*;

#[derive(Debug)]
pub struct PlayerInput {
    pub x: f32,
    pub y: f32,
    pub interact: bool,
    pub back: bool
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
        button: &Res<Input<GamepadButton>>
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
    pub fn merge(
        &mut self,
        inputs: impl Iterator<Item = PlayerInput>
    ) {
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
