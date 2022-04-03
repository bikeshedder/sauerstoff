use bevy::{
    math::Vec3,
    prelude::{Commands, Res},
};

use crate::{
    data::{entity_types::EntityTypes, map::Map},
    spawn_entity,
};

pub fn initialize_map(mut commands: Commands, map: Res<Map>, entity_types: Res<EntityTypes>) {
    for (name, entity) in map.entities.iter() {
        let entity_type = entity_types.get(&entity.entity_type).unwrap_or_else(|| {
            panic!(
                "Entity {:?} references non existant entity type: {}",
                name, entity.entity_type
            )
        });
        let position = Vec3::new(entity.position.x.into(), entity.position.y.into(), 0.0);
        spawn_entity(&mut commands, entity_type, position, None, |_| {});
    }
}
