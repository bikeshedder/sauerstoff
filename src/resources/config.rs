use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub audio: AudioConfig,
}

#[derive(Debug, Deserialize)]
pub struct AudioConfig {
    pub music_volume: f32,
}

impl Config {
    pub fn load() -> Self {
        let file = std::fs::File::open("config.yaml").unwrap();
        serde_yaml::from_reader(file).unwrap()
    }
}
