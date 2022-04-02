use bevy::{core::FixedTimestep, prelude::*};
use input::PlayerInput;

mod input;

const TIME_STEP: f32 = 1.0 / 60.0;
const PLAYER_SPEED: f32 = 128.0;

#[derive(Component, Debug)]
struct Map {}

#[derive(Component, Debug)]
struct Player {}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("map.jpg"),
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            ..Default::default()
        },
        ..Default::default()
    }).insert(Map {});
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("wolfgang.png"),
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 1.0),
            scale: Vec3::splat(0.2),
            ..Default::default()
        },
        ..Default::default()
    }).insert(Player {});
}

fn player_input(
    key: Res<Input<KeyCode>>,
    gamepads: Res<Gamepads>,
    gamepad_axis: Res<Axis<GamepadAxis>>,
    gamepad_button: Res<Input<GamepadButton>>,
    mut query: Query<(&mut Transform, &Player)>,
    time: Res<Time>,
) {
    let mut input = PlayerInput::from_keys(key);
    input.merge(
        gamepads
            .iter()
            .map(|gamepad| PlayerInput::from_gamepad(*gamepad, &gamepad_axis, &gamepad_button)),
    );
    let (mut transform, _) = query.single_mut();
    let delta = time.delta().as_secs_f32();
    transform.translation.x += input.x * PLAYER_SPEED * delta;
    transform.translation.y += input.y * PLAYER_SPEED * delta;
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(player_input),
        )
        .run()
}
