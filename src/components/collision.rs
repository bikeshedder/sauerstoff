use bevy::{
    math::{Vec2, Vec3},
    prelude::Component,
    sprite::collide_aabb::{collide, Collision as BevyCollision},
};

use crate::{
    data::common::{Rect, Size},
    helpers::z_index,
};

#[derive(Component, Debug)]
pub struct Collision {
    pub origin: Vec3,
    pub pos: Vec3,
    pub size: Vec2,
}

impl Collision {
    pub fn from_data(entity_size: Size, collision_box: Rect) -> Self {
        // The collision box in the data uses a right handed coordinate
        // system with 0:0 in the lower left corner. Bevy uses a left
        // handed coordinate system with 0:0 in the center. The origin
        // can be added to the entity translation to calculate the bevy
        // position of the bounding box.
        let origin = Vec3::new(
            -f32::from(entity_size.width) / 2.0
                + f32::from(collision_box.size.width) / 2.0
                + f32::from(collision_box.position.x),
            // FIXME this expression can be simplified
            f32::from(entity_size.height) / 2.0
                - f32::from(collision_box.size.height) / 2.0
                - f32::from(collision_box.position.y),
            0.0,
        );
        Self {
            origin,
            pos: origin,
            size: Vec2::new(
                collision_box.size.width.into(),
                collision_box.size.height.into(),
            ),
        }
    }
    pub fn update_position(&mut self, translation: Vec3) -> f32 {
        self.pos = translation + self.origin;
        z_index(self.pos.y)
    }
    /// Check collision with annother `Collision` object
    /// and return the new entity transformation if a collision
    /// was detected.
    pub fn collide(&mut self, translation: Vec3, other: &Self) -> Option<Vec3> {
        collide(translation + self.origin, self.size, other.pos, other.size).map(|collision| {
            match collision {
                BevyCollision::Left => Vec3::new(
                    other.pos.x - other.size.x / 2.0 - self.size.x / 2.0 - self.origin.x,
                    translation.y,
                    translation.z,
                ),
                BevyCollision::Right => Vec3::new(
                    other.pos.x + other.size.x / 2.0 + self.size.x / 2.0 - self.origin.x,
                    translation.y,
                    translation.z,
                ),
                BevyCollision::Top => Vec3::new(
                    translation.x,
                    other.pos.y + other.size.y / 2.0 + self.size.y / 2.0 - self.origin.y,
                    translation.z,
                ),
                BevyCollision::Bottom => Vec3::new(
                    translation.x,
                    other.pos.y - other.size.y / 2.0 - self.size.y / 2.0 - self.origin.y,
                    translation.z,
                ),
            }
        })
    }
}
