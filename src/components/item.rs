use bevy::{core::Stopwatch, prelude::Component};

#[derive(Component, Debug)]
pub struct Item {}

#[derive(Component, Debug)]
pub struct ItemShadow {
    pub watch: Stopwatch,
}

#[derive(Component, Debug)]
pub struct ItemSprite {
    pub watch: Stopwatch,
}
