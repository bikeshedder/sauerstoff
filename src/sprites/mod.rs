use serde::Deserialize;

pub mod wolfgang;

type Frames = Vec<Frame>;

#[derive(Deserialize, Debug)]
struct Frame {
    pub image: String,
    pub duration: usize,
}
