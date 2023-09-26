use std::f32::consts::TAU;

use bevy::{
    math::Vec3,
    prelude::{AssetServer, BuildChildren, Commands, GlobalTransform, Query, Res, Transform},
    sprite::{Sprite, SpriteBundle},
    time::{Stopwatch, Time},
};

use crate::{
    components::item::{ItemShadow, ItemSprite},
    helpers::z_index,
};

pub fn spawn_item(commands: &mut Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_empty()
        .insert(Transform {
            translation: Vec3::new(200.0, 200.0, z_index(200.0 - 50.0)),
            scale: Vec3::splat(0.4),
            ..Default::default()
        })
        .insert(GlobalTransform::default())
        .with_children(|parent| {
            parent
                .spawn(SpriteBundle {
                    texture: asset_server.load("entities/Crystal_Shard.png"),
                    ..Default::default()
                })
                .insert(ItemSprite {
                    watch: Stopwatch::new(),
                });
            parent
                .spawn(SpriteBundle {
                    texture: asset_server.load("entities/Crystal_Shard_Shadow.png"),
                    transform: Transform {
                        translation: Vec3::new(0.0, -80.0, 0.0),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(ItemShadow {
                    watch: Stopwatch::new(),
                });
        });
}

const BOBBING_DURATION: f32 = 2.0;
const BOBBING_HEIGHT: f32 = 40.0;

pub fn item_bobbing(
    time: Res<Time>,
    mut query: Query<(&mut ItemSprite, &mut Transform)>,
    mut shadow_query: Query<(&mut ItemShadow, &mut Sprite)>,
) {
    for (mut sprite, mut transform) in query.iter_mut() {
        sprite.watch.tick(time.delta());
        transform.translation.y =
            (sprite.watch.elapsed_secs() / BOBBING_DURATION * TAU).sin() * BOBBING_HEIGHT;
    }
    for (mut sprite, mut spr) in shadow_query.iter_mut() {
        sprite.watch.tick(time.delta());
        spr.color.set_a(
            0.2 + 0.8
                * (1.0
                    - ((sprite.watch.elapsed_secs() / BOBBING_DURATION * TAU).sin() + 1.0) / 2.0),
        );
    }
}
