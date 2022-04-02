use bevy::{
    asset::LoadState,
    core::FixedTimestep,
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};
use bevy_kira_audio::{Audio, AudioPlugin};
use input::PlayerInput;
use sprites::{Map, WolfgangFrames};

mod input;
mod sprites;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum AppState {
    Setup,
    Finished,
}

const TIME_STEP: f32 = 1.0 / 60.0;
const PLAYER_SPEED: f32 = 300.0;

#[derive(Debug)]
enum PlayerAnimation {
    Idle,
    WalkRight,
    WalkLeft,
    WalkUp,
    WalkDown,
    InteractLeft,
    InteractRight,
}

#[derive(Debug)]
enum InteractDirection {
    Left,
    Right,
}

#[derive(Component, Debug)]
struct Player {
    animation: PlayerAnimation,
    interact_direction: InteractDirection,
    frame_index: usize,
}

#[derive(Debug, Component)]
struct Blocking {
    pos: Vec3,
    size: Vec2,
}

#[derive(Default)]
struct EntityTextures {
    handles: Vec<HandleUntyped>,
}

const Z_MIN: f32 = 0.0;
fn z_index(y: f32) -> f32 {
    1.0 - y / 65536.0
}

fn load_textures(mut entities: ResMut<EntityTextures>, asset_server: Res<AssetServer>) {
    entities.handles = asset_server.load_folder("entities").unwrap();
}

fn check_textures(
    mut state: ResMut<State<AppState>>,
    entities: ResMut<EntityTextures>,
    asset_server: Res<AssetServer>,
) {
    if let LoadState::Loaded =
        asset_server.get_group_load_state(entities.handles.iter().map(|handle| handle.id))
    {
        state.set(AppState::Finished).unwrap();
    }
}

fn initialize_map(mut commands: Commands, asset_server: Res<AssetServer>, map: Res<Map>) {
    for crystal in &map.crystals {
        let translation = Vec3::new(
            crystal.x.into(),
            crystal.y.into(),
            z_index(f32::from(crystal.y) - f32::from(crystal.size.size().1) / 2.0),
        );
        commands
            .spawn_bundle(SpriteBundle {
                texture: asset_server.load(crystal.size.image()),
                transform: Transform {
                    translation,
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Blocking {
                pos: translation + crystal.size.collision_origin(),
                size: crystal.size.collision_size(),
            });
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    entity_textures: ResMut<EntityTextures>,
    mut textures: ResMut<Assets<Image>>,
    mut wolfgang_frames: ResMut<WolfgangFrames>,
) {
    let mut atlas_builder = TextureAtlasBuilder::default();
    for handle in &entity_textures.handles {
        let texture = textures.get(handle).unwrap();
        atlas_builder.add_texture(handle.clone_weak().typed::<Image>(), texture);
    }
    let atlas = atlas_builder.finish(&mut textures).unwrap();

    for frame in wolfgang_frames.frames_mut() {
        let handle = asset_server.get_handle(&format!("entities/wolfgang/{}", frame.image));
        frame.index = atlas
            .get_texture_index(&handle)
            .unwrap_or_else(|| panic!("Missing image: {}", frame.image));
    }

    let atlas_handle = texture_atlases.add(atlas);

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    //commands.spawn_bundle(UiCameraBundle::default());
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("map/map.jpg"),
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, Z_MIN),
            ..Default::default()
        },
        ..Default::default()
    });

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: atlas_handle,
            sprite: TextureAtlasSprite::new(0),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, z_index(0.0)),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player {
            animation: PlayerAnimation::Idle,
            interact_direction: InteractDirection::Right,
            frame_index: 0,
        })
        .insert(Timer::from_seconds(0.15, true));

    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("entities/engine/Motor.png"),
        transform: Transform {
            translation: Vec3::new(500.0, 200.0, z_index(300.0 - 396.0 / 2.0)),
            ..Default::default()
        },
        ..Default::default()
    });
}

fn player_input(
    key: Res<Input<KeyCode>>,
    gamepads: Res<Gamepads>,
    gamepad_axis: Res<Axis<GamepadAxis>>,
    gamepad_button: Res<Input<GamepadButton>>,
    mut query: Query<(&mut Transform, &mut Player)>,
    time: Res<Time>,
    blocking_query: Query<(&Blocking, &Transform, Without<Player>)>,
) {
    let mut input = PlayerInput::from_keys(key);
    input.merge(
        gamepads
            .iter()
            .map(|gamepad| PlayerInput::from_gamepad(*gamepad, &gamepad_axis, &gamepad_button)),
    );
    let (mut transform, mut player) = query.single_mut();
    let delta = time.delta().as_secs_f32();

    player.animation = PlayerAnimation::Idle;

    if input.y > 0.0 {
        player.animation = PlayerAnimation::WalkUp;
    } else if input.y < 0.0 {
        player.animation = PlayerAnimation::WalkDown;
    }

    if input.x > 0.0 {
        player.animation = PlayerAnimation::WalkRight;
        player.interact_direction = InteractDirection::Right;
    } else if input.x < 0.0 {
        player.animation = PlayerAnimation::WalkLeft;
        player.interact_direction = InteractDirection::Left;
    }

    if input.interact {
        player.animation = match player.interact_direction {
            InteractDirection::Left => PlayerAnimation::InteractLeft,
            InteractDirection::Right => PlayerAnimation::InteractRight,
        };
    } else {
        transform.translation.x += input.x * PLAYER_SPEED * delta;
        transform.translation.y += input.y * PLAYER_SPEED * delta;
        // FIXME hardcoded player size... meh...
        transform.translation.z = z_index(transform.translation.y - 128.0);

        // now make sure we're not colliding with anything
        for (blocking, blocking_transform, _) in blocking_query.iter() {
            // FIXME hardcoded sizes
            if let Some(collision) = collide(
                transform.translation + Vec3::new(0.0, -64.0, 0.0),
                Vec2::new(128.0, 64.0),
                blocking.pos,
                blocking.size,
            ) {
                match collision {
                    Collision::Left => {
                        transform.translation.x = blocking.pos.x - blocking.size.x / 2.0 - 64.0;
                    }
                    Collision::Right => {
                        transform.translation.x = blocking.pos.x + blocking.size.x / 2.0 + 64.0;
                    }
                    Collision::Top => {
                        transform.translation.y = blocking.pos.y + blocking.size.y / 2.0 + 96.0;
                    }
                    Collision::Bottom => {
                        transform.translation.y = blocking.pos.y - blocking.size.y / 2.0 + 32.0;
                    }
                }
            }
        }
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

fn animate_sprite_system(
    time: Res<Time>,
    frames: Res<WolfgangFrames>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut Timer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
        &mut Player,
    )>,
    asset_server: Res<AssetServer>,
    mut wolfgang_frames: Res<WolfgangFrames>,
) {
    let vendor_handle: Handle<TextureAtlasSprite> =
        asset_server.get_handle("entities/wolfgang/Wolfgang_RunL_00001.png");
    for (mut timer, mut sprite, texture_atlas, mut player) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.finished() {
            let frames = match player.animation {
                PlayerAnimation::Idle => &wolfgang_frames.idle,
                PlayerAnimation::WalkLeft => &wolfgang_frames.walk_left,
                PlayerAnimation::WalkRight => &wolfgang_frames.walk_right,
                PlayerAnimation::WalkUp => &wolfgang_frames.walk_up,
                PlayerAnimation::WalkDown => &wolfgang_frames.walk_down,
                PlayerAnimation::InteractLeft => &wolfgang_frames.interact_left,
                PlayerAnimation::InteractRight => &wolfgang_frames.interact_right,
            };
            player.frame_index = (player.frame_index + 1) % frames.len();
            let frame = &frames[player.frame_index];
            sprite.index = frame.index;
            timer.set_duration(std::time::Duration::from_millis(frame.duration));
        }
    }
}

fn start_background_audio(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    audio.play_looped(asset_server.load("music/base.mp3"));
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let yaml = std::fs::File::open("assets/entities/wolfgang.yaml")?;
    let wolfgang_frames: WolfgangFrames = serde_yaml::from_reader(yaml)?;

    let yaml = std::fs::File::open("assets/map/map.yaml")?;
    let map: Map = serde_yaml::from_reader(yaml)?;

    App::new()
        .init_resource::<EntityTextures>()
        .insert_resource(wolfgang_frames)
        .insert_resource(map)
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
                .with_system(animate_sprite_system)
                .with_system(camera_system),
        )
        .run();

    Ok(())
}
