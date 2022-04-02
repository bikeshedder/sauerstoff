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
