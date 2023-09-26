use bevy::{ecs::system::Resource, utils::HashMap};
use serde::Deserialize;

use super::common::Position;

#[derive(Resource)]
pub struct Map {
    pub entities: MapEntities,
}

pub type MapEntities = HashMap<String, MapEntity>;

#[derive(Deserialize, Debug)]
pub struct MapEntity {
    #[serde(rename = "type")]
    pub entity_type: String,
    #[serde(flatten)]
    pub position: Position,
}

pub fn load_map() -> Result<Map, anyhow::Error> {
    let file_name = "assets/map/entities.yaml";
    let file = std::fs::File::open(file_name)
        .unwrap_or_else(|e| panic!("Reading {:?} failed: {:?}", file_name, e));
    let entities: MapEntities = serde_yaml::from_reader(file)
        .unwrap_or_else(|e| panic!("Parsing {:?} failed: {:?}", file_name, e));
    Ok(Map { entities })
}
