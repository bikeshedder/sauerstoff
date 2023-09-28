use bevy::{
    ecs::system::EntityCommands,
    prelude::*,
    render::camera::ScalingMode,
    window::{close_on_esc, WindowResolution},
};
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
use resources::{config::Config, map::Map};
use systems::{
    animation::{animation_system, AnimationTimer},
    camera::camera_system,
    input::player_input,
    interaction::detect_interaction,
    item::{item_bobbing, spawn_item},
    map::initialize_map,
    music::{music_scene, music_system},
    player::player_system,
    textures::{check_textures, load_textures},
};

mod components;
mod data;
mod helpers;
mod resources;
mod systems;

#[derive(States, Debug, Default, Clone, PartialEq, Eq, Hash)]
pub enum AppState {
    #[default]
    Setup,
    Finished,
}

#[derive(Resource, Default)]
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
        Loaded::Static(handle) => commands.spawn(SpriteBundle {
            texture: handle.clone(),
            transform: Transform {
                translation,
                ..Default::default()
            },
            ..Default::default()
        }),
        Loaded::Animations(animations) => {
            let mut cmd = commands.spawn(SpriteSheetBundle {
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
        .spawn({
            let mut bundle = Camera2dBundle::default();
            let proj = &mut bundle.projection;
            proj.scaling_mode = ScalingMode::FixedHorizontal(1920.0);
            bundle
        })
        .insert(FollowCam {});
    commands.spawn(SpriteBundle {
        texture: asset_server.load("map/map.jpg"),
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            ..Default::default()
        },
        ..Default::default()
    });

    spawn_entity(
        &mut commands,
        entity_types.map.get("wolfgang").unwrap(),
        Vec3::new(0.0, 0.0, 0.0),
        Some("idle"),
        |cmd| {
            cmd.insert(Player::default())
                // XXX initial timer value?
                .insert(AnimationTimer::from_seconds(0.1));
        },
    );

    commands.spawn(TextBundle {
        style: Style {
            margin: UiRect::all(Val::Px(5.0)),
            ..Default::default()
        },
        text: Text::from_section(
            "Text Example",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 30.0,
                color: Color::WHITE,
            },
        ),
        ..Default::default()
    });

    spawn_item(&mut commands, asset_server);
}

fn resize_window(mut windows: Query<&mut Window>) {
    let mut window = windows.get_single_mut().unwrap();
    window.resolution = WindowResolution::new(1920.0, 1080.0);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::load();
    let map = load_map()?;
    let entity_types = load_entity_types()?;

    let mut app = App::new();
    app.add_state::<AppState>();
    app.insert_resource(config);
    app.init_resource::<ImageHandles>();
    app.init_resource::<Map>();
    app.insert_resource(map);
    app.insert_resource(entity_types);
    app.add_plugins(DefaultPlugins);
    app.add_plugins(AudioPlugin);
    app.add_systems(Startup, music_system);
    app.add_systems(Startup, resize_window);
    app.add_systems(Startup, load_textures);
    app.add_systems(Update, check_textures.run_if(in_state(AppState::Setup)));
    app.add_systems(OnEnter(AppState::Finished), (initialize_map, setup));
    app.add_systems(
        Update,
        (
            player_input,
            player_system,
            animation_system,
            detect_interaction,
            camera_system,
            item_bobbing,
            music_scene,
        )
            .run_if(in_state(AppState::Finished)),
    );
    app.add_systems(Update, close_on_esc);
    app.run();

    Ok(())
}
