use std::time::Duration;

use bevy::{
    asset::LoadState, ecs::system::EntityCommands, prelude::*, sprite::collide_aabb::collide,
    sprite::collide_aabb::Collision as BevyCollision, utils::HashMap,
};
use bevy_kira_audio::{Audio, AudioPlugin};

use components::collision::Collision;
use data::{
    entity_types::{
        load_entity_types, EntityImage, EntityType, EntityTypes, Loaded, LoadedAnimation,
        LoadedAnimations,
    },
    map::{load_map, Map, MapEntity},
};
use helpers::z_index;
use systems::input::PlayerInput;

mod components;
mod data;
mod helpers;
mod systems;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum AppState {
    Setup,
    Finished,
}

const PLAYER_SPEED: f32 = 300.0;

#[derive(Debug)]
enum InteractDirection {
    Left,
    Right,
}

#[derive(Component, Debug)]
struct Player {
    interact_direction: InteractDirection,
}

#[derive(Component, Debug)]
struct Animation {
    frames: HashMap<String, Vec<(usize, Duration)>>,
}

#[derive(Component, Debug)]
struct AnimationState {
    animation: &'static str,
    index: usize,
}

#[derive(Default)]
struct ImageHandles {
    handles: Vec<Handle<Image>>,
}

impl ImageHandles {
    fn add(&mut self, handle: Handle<Image>) -> usize {
        let index = self.handles.len();
        self.handles.push(handle);
        index
    }
}

fn load_textures(
    mut entity_types: ResMut<EntityTypes>,
    mut image_handles: ResMut<ImageHandles>,
    asset_server: Res<AssetServer>,
    mut textures: ResMut<Assets<Image>>,
) {
    for entity_type in entity_types.values_mut() {
        match &entity_type.image {
            EntityImage::Static(image) => {
                let handle = asset_server.load::<Image, _>(&format!("entities/{image}"));
                image_handles.add(handle.clone());
                entity_type.loaded = Some(Loaded::Static(handle));
            }

            /*
            EntityImage::Animation(frames) => {
                let mut atlas_builder = TextureAtlasBuilder::default();
                for frame in frames.iter() {
                    let handle = asset_server.get_handle(&format!("entities/{}", frame.image));
                    let texture = textures.get(&format!("entities/{}", frame.image)).unwrap();
                    atlas_builder.add_texture(handle, texture);
                }
                let atlas = atlas_builder.finish(&mut textures).unwrap();
                for frame in frames.iter() {
                    let frame_index = atlas
                        .get_texture_index(&handle)
                        .unwrap_or_else(|| panic!("Missing image: {}", frame.image));
                    // FIXME
                }
            }
             */
            EntityImage::Animations(animations) => {
                for animation in animations.values() {
                    for frame in animation.iter() {
                        let image = &frame.image;
                        let handle = asset_server.load::<Image, _>(&format!("entities/{image}"));
                        image_handles.add(handle.clone());
                        entity_type.loaded = Some(Loaded::Static(handle));
                    }
                }
            }
            _ => unimplemented!(),
        }
    }
}

fn check_textures(
    mut state: ResMut<State<AppState>>,
    image_handles: ResMut<ImageHandles>,
    asset_server: Res<AssetServer>,
    mut entity_types: ResMut<EntityTypes>,
    mut textures: ResMut<Assets<Image>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    if let LoadState::Loaded =
        asset_server.get_group_load_state(image_handles.handles.iter().map(|handle| handle.id))
    {
        state.set(AppState::Finished).unwrap();

        for entity_type in entity_types.values_mut() {
            match &entity_type.image {
                EntityImage::Static(image) => {
                    // The handle was already assigned in the load_textures method.
                }
                /*
                EntityImage::Animation(frames) => {
                    let mut atlas_builder = TextureAtlasBuilder::default();
                    for frame in frames.iter() {
                        let handle = asset_server.get_handle(&format!("entities/{}", frame.image));
                        let texture = textures.get(&format!("entities/{}", frame.image)).unwrap();
                        atlas_builder.add_texture(handle, texture);
                    }
                    let atlas = atlas_builder.finish(&mut textures).unwrap();
                    for frame in frames.iter() {
                        let frame_index = atlas
                            .get_texture_index(&handle)
                            .unwrap_or_else(|| panic!("Missing image: {}", frame.image));
                        // FIXME
                    }
                }
                 */
                EntityImage::Animations(animations) => {
                    let mut atlas_builder = TextureAtlasBuilder::default();
                    let frame_handles: HashMap<String, Vec<(Handle<Image>, Duration)>> = animations
                        .iter()
                        .map(|(animation_name, frames)| {
                            (
                                animation_name.clone(),
                                frames
                                    .iter()
                                    .map(|frame| {
                                        let file_name = format!("entities/{}", frame.image);
                                        let handle = asset_server.get_handle(&file_name);
                                        let texture = textures.get(&file_name).unwrap();
                                        atlas_builder.add_texture(handle.clone(), texture);
                                        (handle, Duration::from_millis(frame.duration))
                                    })
                                    .collect(),
                            )
                        })
                        .collect();
                    let atlas = atlas_builder.finish(&mut textures).unwrap();
                    let atlas_handle = texture_atlases.add(atlas);
                    let atlas = texture_atlases.get(atlas_handle.clone()).unwrap();
                    entity_type.loaded = Some(Loaded::Animations(LoadedAnimations {
                        atlas: atlas_handle,
                        frames: frame_handles
                            .into_iter()
                            .map(|(animation_name, frames)| {
                                (
                                    animation_name,
                                    frames
                                        .into_iter()
                                        .map(|(handle, duration)| {
                                            (atlas.get_texture_index(&handle).unwrap(), duration)
                                        })
                                        .collect(),
                                )
                            })
                            .collect(),
                    }));
                }
                _ => unimplemented!(),
            }
        }
        /*
        let mut atlas_builder = TextureAtlasBuilder::default();
        for handle in &entity_textures.handles {
            let texture = textures.get(handle).unwrap();
            atlas_builder.add_texture(handle.clone_weak().typed::<Image>(), texture);
        }

        for frame in wolfgang_frames.frames_mut() {
            let handle = asset_server.get_handle(&format!("entities/wolfgang/{}", frame.image));
            frame.index = atlas
                .get_texture_index(&handle)
                .unwrap_or_else(|| panic!("Missing image: {}", frame.image));
        }

        let atlas_handle = texture_atlases.add(atlas);
         */
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
                index: 0,
            });
            cmd
        }
        _ => unimplemented!(),
    };
    if let Some(collision) = collision {
        entity_cmds.insert(collision);
    }
    f(&mut entity_cmds);
}

fn initialize_map(mut commands: Commands, map: Res<Map>, entity_types: Res<EntityTypes>) {
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

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut textures: ResMut<Assets<Image>>,
    entity_types: Res<EntityTypes>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    //commands.spawn_bundle(UiCameraBundle::default());
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
            cmd.insert(Player {
                interact_direction: InteractDirection::Right,
            })
            .insert(Timer::from_seconds(0.15, true));
        },
    );
}

fn player_input(
    key: Res<Input<KeyCode>>,
    gamepads: Res<Gamepads>,
    gamepad_axis: Res<Axis<GamepadAxis>>,
    gamepad_button: Res<Input<GamepadButton>>,
    mut query: Query<(
        &mut Transform,
        &mut Player,
        &mut AnimationState,
        &mut Collision,
    )>,
    time: Res<Time>,
    blocking_query: Query<(&Collision, &Transform, Without<Player>)>,
) {
    let mut input = PlayerInput::from_keys(key);
    input.merge(
        gamepads
            .iter()
            .map(|gamepad| PlayerInput::from_gamepad(*gamepad, &gamepad_axis, &gamepad_button)),
    );
    let (mut transform, mut player, mut animation, mut player_collision) = query.single_mut();
    let delta = time.delta().as_secs_f32();

    animation.animation = "idle";

    if input.y > 0.0 {
        animation.animation = "walk_up";
    } else if input.y < 0.0 {
        animation.animation = "walk_down";
    }

    if input.x > 0.0 {
        animation.animation = "walk_right";
        player.interact_direction = InteractDirection::Right;
    } else if input.x < 0.0 {
        animation.animation = "walk_left";
        player.interact_direction = InteractDirection::Left;
    }

    if input.interact {
        animation.animation = match player.interact_direction {
            InteractDirection::Left => "interact_left",
            InteractDirection::Right => "interact_right",
        };
    } else {
        transform.translation.x += input.x * PLAYER_SPEED * delta;
        transform.translation.y += input.y * PLAYER_SPEED * delta;
        transform.translation.z = player_collision.update_position(transform.translation);

        /* FIXME this code is currently broken
        // now make sure we're not colliding with anything
        for (blocking, blocking_transform, _) in blocking_query.iter() {
            if let Some(collision) = collide(
                player_collision.pos,
                player_collision.size,
                blocking.pos,
                blocking.size,
            ) {
                match collision {
                    BevyCollision::Left => {
                        transform.translation.x =
                    }
                    BevyCollision::Right => {
                        transform.translation.x =
                    }
                    BevyCollision::Top => {
                        transform.translation.y =
                    }
                    BevyCollision::Bottom => {
                        transform.translation.y =
                    }
                    _ => {}
                }
            }
        }
         */
    }
}

fn camera_system(
    mut camera_query: Query<(&Camera, &mut Transform, Without<Player>)>,
    player_query: Query<(&Player, &Transform, Without<Camera>)>,
) {
    let (_, player_transform, _) = player_query.single();
    if let Ok((_, mut transform, _)) = camera_query.get_single_mut() {
        transform.translation.x = player_transform.translation.x.clamp(-1920.0, 1920.0);
        transform.translation.y = player_transform.translation.y.clamp(-1080.0, 1080.0);
    }
}

fn animation_system(
    time: Res<Time>,
    mut query: Query<(
        &mut Timer,
        &mut TextureAtlasSprite,
        &Animation,
        &mut AnimationState,
    )>,
) {
    for (mut timer, mut sprite, animation, mut state) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.finished() {
            let frames = &animation.frames[state.animation];
            state.index = (state.index + 1) % frames.len();
            let (atlas_index, duration) = frames[state.index];
            sprite.index = atlas_index;
            timer.set_duration(duration);
        }
    }
}

fn start_background_audio(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    audio.play_looped(asset_server.load("music/base.mp3"));
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
        //.add_startup_system(start_background_audio)
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
                .with_system(animation_system)
                .with_system(camera_system),
        )
        .run();

    Ok(())
}
