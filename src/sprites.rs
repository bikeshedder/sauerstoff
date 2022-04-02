use bevy::math::{Vec2, Vec3};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct WolfgangFrames {
    pub idle: Frames,
    pub walk_left: Frames,
    pub walk_right: Frames,
    pub walk_up: Frames,
    pub walk_down: Frames,
    pub interact_left: Frames,
    pub interact_right: Frames,
}

impl WolfgangFrames {
    pub fn frames_mut(&mut self) -> impl Iterator<Item = &mut Frame> {
        itertools::chain!(
            self.idle.iter_mut(),
            self.walk_left.iter_mut(),
            self.walk_right.iter_mut(),
            self.walk_up.iter_mut(),
            self.walk_down.iter_mut(),
            self.interact_left.iter_mut(),
            self.interact_right.iter_mut()
        )
    }
}

pub type Frames = Vec<Frame>;

#[derive(Deserialize, Debug)]
pub struct Frame {
    pub image: String,
    pub duration: u64,
    #[serde(skip)]
    pub index: usize,
}

#[derive(Deserialize, Debug)]
pub struct Map {
    pub crystals: Vec<CrystalSpawn>,
}

#[derive(Deserialize, Debug)]
pub enum CrystalSize {
    Small,
    Medium,
    Large,
}

impl CrystalSize {
    pub fn image(&self) -> &'static str {
        match self {
            Self::Small => "entities/crystal_small/Crystals_Small.png",
            Self::Medium => "entities/crystal_medium/Crystals_Medium.png",
            Self::Large => "entities/crystal_big/Crystals_Big.png",
        }
    }
    pub fn size(&self) -> (u16, u16) {
        match self {
            Self::Small => (241, 190),
            Self::Medium => (454, 423),
            Self::Large => (672, 656),
        }
    }
    pub fn collision_origin(&self) -> Vec3 {
        let size = self.size();
        match self {
            Self::Small => Vec3::new(
                -241.0 / 2.0 + 200.0 / 2.0,
                -190.0 / 2.0 + 60.0 / 2.0 + 20.0,
                0.0,
            ),
            _ => Vec3::default(),
        }
    }
    pub fn collision_size(&self) -> Vec2 {
        match self {
            Self::Small => Vec2::new(200.0, 60.0),
            Self::Medium => Vec2::new(0.0, 0.0), // FIXME
            Self::Large => Vec2::new(0.0, 0.0),  // FIXME
        }
    }
    pub fn origin(&self) -> Vec3 {
        let size = self.size();
        Vec3::new(-f32::from(size.0) / 2.0, -f32::from(size.1) / 2.0, 0.0)
    }
}

#[derive(Deserialize, Debug)]
pub struct CrystalSpawn {
    pub id: String,
    pub size: CrystalSize,
    pub x: i16,
    pub y: i16,
}
