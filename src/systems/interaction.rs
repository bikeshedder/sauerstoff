use bevy::prelude::{Query, Transform};

use crate::components::{interaction::Interaction, player::Player};

pub fn detect_interaction(
    player_query: Query<(&Player, &Transform)>,
    interaction_query: Query<&Interaction>,
) {
    let (player, player_transform) = player_query.single();
    let interactions = interaction_query.iter().filter(|interaction| {
        ((player_transform.translation + player.center).distance(interaction.center))
            <= f32::from(interaction.max_distance)
    });
    for interaction in interactions {
        // FIXME continue here
        println!("Available interaction: {}", interaction.name);
    }
}
