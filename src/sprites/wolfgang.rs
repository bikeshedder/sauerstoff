use serde::Deserialize;

use super::Frames;

#[derive(Deserialize, Debug)]
struct Wolfgang {
    pub walk_left: Frames,
    pub walk_right: Frames,
    pub walk_up: Frames,
    pub walk_down: Frames,
}
