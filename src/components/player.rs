use bevy::prelude::Component;

pub const PLAYER_SPEED: f32 = 300.0;

#[derive(Debug)]
pub enum InteractDirection {
    Left,
    Right,
}

#[derive(Component, Debug)]
pub struct Player {
    pub interact_direction: InteractDirection,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            interact_direction: InteractDirection::Right,
        }
    }
}
