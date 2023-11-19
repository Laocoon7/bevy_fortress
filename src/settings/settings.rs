use std::path::PathBuf;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use super::{CameraSettings, SoundSettings, WindowSettings};

const PATH: &'static str = "game.settings";

#[derive(Serialize, Deserialize, Resource, Debug, Clone)]
pub struct Settings {
    #[serde(skip)]
    pub path: PathBuf,

    pub camera_settings: CameraSettings,
    pub sound_settings: SoundSettings,
    pub window_settings: WindowSettings,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            path: PathBuf::from(PATH),

            camera_settings: CameraSettings::default(),
            sound_settings: SoundSettings::default(),
            window_settings: WindowSettings::default(),
        }
    }
}

impl Settings {
    pub fn new<P: Into<PathBuf>>(path: P) -> Self {
        Self {
            path: path.into(),

            ..Default::default()
        }
    }

    pub fn load(mut self) -> Self {
        match std::fs::read_to_string(&self.path) {
            Ok(contents) => match toml::from_str::<Self>(&contents) {
                Ok(data) => {
                    self.camera_settings = data.camera_settings;
                    self.sound_settings = data.sound_settings;
                    self.window_settings = data.window_settings;
                },
                Err(e) => {
                    error!("{e}");
                },
            },
            Err(e) => {
                error!("{e}");
            },
        }
        self
    }

    pub fn save(&self) {
        match toml::to_string_pretty(self) {
            Ok(contents) => match std::fs::write(&self.path, contents) {
                Err(e) => error!("{e}"),
                _ => (),
            },
            Err(e) => error!("{e}"),
        }
    }
}
