use bevy::{ecs::system::EntityCommands, prelude::*};
use bevy_kira_audio::AudioPlugin;

use components::{
    animation::{Animation, AnimationState},
    collision::Collision,
    followcam::FollowCam,
    interaction::Interaction,
    player::Player,
};
use data::{
    entity_types::{load_entity_types, EntityType, EntityTypes, Loaded},
    map::load_map,
};
use helpers::z_index;
use systems::{
    animation::animation_system,
    camera::camera_system,
    input::player_input,
    interaction::detect_interaction,
    item::{item_bobbing, spawn_item},
    map::initialize_map,
    music::music_system,
    player::player_system,
    textures::{check_textures, load_textures},
};

mod components;
mod data;
mod helpers;
mod systems;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppState {
    Setup,
    Finished,
}

#[derive(Default)]
pub struct ImageHandles {
    handles: Vec<Handle<Image>>,
}

impl ImageHandles {
    pub fn add(&mut self, handle: Handle<Image>) -> usize {
        let index = self.handles.len();
        self.handles.push(handle);
        index
    }
}

fn spawn_entity(
    commands: &mut Commands,
    entity_type: &EntityType,
    mut translation: Vec3,
    animation_name: Option<&'static str>,
    f: fn(cmd: &mut EntityCommands),
) {
    let collision = entity_type.collision.map(|collision| {
        let mut collision = Collision::from_data(entity_type.size, collision);
        translation.z = collision.update_position(translation);
        collision
    });
    let mut entity_cmds = match entity_type.loaded.as_ref().unwrap() {
        Loaded::Static(handle) => commands.spawn_bundle(SpriteBundle {
            texture: handle.clone(),
            transform: Transform {
                translation,
                ..Default::default()
            },
            ..Default::default()
        }),
        Loaded::Animations(animations) => {
            let mut cmd = commands.spawn_bundle(SpriteSheetBundle {
                texture_atlas: animations.atlas.clone(),
                // FIXME pass optional initial frame
                sprite: TextureAtlasSprite {
                    index: animation_name
                        .map(|name| animations.frames[name][0].0)
                        .unwrap(),
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, z_index(0.0)),
                    ..Default::default()
                },
                ..Default::default()
            });
            cmd.insert(Animation {
                frames: animations.frames.clone(),
            });
            cmd.insert(AnimationState {
                animation: animation_name.unwrap(),
                restart: true,
                index: 0,
            });
            cmd
        }
        _ => unimplemented!(),
    };
    if let Some(collision) = collision {
        entity_cmds.insert(collision);
    }
    if let Some(interaction) = &entity_type.interaction {
        entity_cmds.insert(Interaction {
            name: interaction.name.clone(),
            center: Vec3::new(
                translation.x - f32::from(entity_type.size.width) / 2.0
                    + f32::from(interaction.position.x),
                translation.y + f32::from(entity_type.size.height) / 2.0
                    - f32::from(interaction.position.y),
                0.0,
            ),
            max_distance: interaction.max_distance,
        });
    }
    f(&mut entity_cmds);
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, entity_types: Res<EntityTypes>) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(FollowCam {});
    commands.spawn_bundle(UiCameraBundle::default());
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("map/map.jpg"),
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            ..Default::default()
        },
        ..Default::default()
    });

    spawn_entity(
        &mut commands,
        entity_types.get("wolfgang").unwrap(),
        Vec3::new(0.0, 0.0, 0.0),
        Some("idle"),
        |cmd| {
            cmd.insert(Player::default())
                // XXX initial timer value?
                .insert(Timer::from_seconds(0.1, true));
        },
    );

    commands.spawn_bundle(TextBundle {
        style: Style {
            margin: Rect::all(Val::Px(5.0)),
            ..Default::default()
        },
        text: Text::with_section(
            "Text Example",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 30.0,
                color: Color::WHITE,
            },
            Default::default(),
        ),
        ..Default::default()
    });

    spawn_item(&mut commands, asset_server);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let map = load_map()?;
    let entity_types = load_entity_types()?;

    App::new()
        .init_resource::<ImageHandles>()
        .insert_resource(map)
        .insert_resource(entity_types)
        .add_plugins(DefaultPlugins)
        .add_plugin(AudioPlugin)
        .add_state(AppState::Setup)
        //.add_startup_system(music_system)
        .add_system_set(SystemSet::on_enter(AppState::Setup).with_system(load_textures))
        .add_system_set(SystemSet::on_update(AppState::Setup).with_system(check_textures))
        .add_system_set(
            SystemSet::on_enter(AppState::Finished)
                .with_system(initialize_map)
                .with_system(setup),
        )
        .add_system_set(
            SystemSet::on_update(AppState::Finished)
                .with_system(player_input)
                .with_system(player_system)
                .with_system(animation_system)
                .with_system(detect_interaction)
                .with_system(camera_system)
                .with_system(item_bobbing),
        )
        .run();

    Ok(())
}
