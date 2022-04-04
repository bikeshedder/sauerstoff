use bevy::prelude::{Camera, Query, Transform, Without};

use crate::components::{followcam::FollowCam, player::Player};

pub fn camera_system(
    mut camera_query: Query<(&FollowCam, &mut Transform, Without<Player>)>,
    player_query: Query<(&Player, &Transform, Without<Camera>)>,
) {
    let (_, player_transform, _) = player_query.single();
    if let Ok((_, mut transform, _)) = camera_query.get_single_mut() {
        transform.translation.x = player_transform.translation.x.clamp(-2880.0, 2880.0);
        transform.translation.y = player_transform.translation.y.clamp(-1620.0, 1620.0);
    }
}
