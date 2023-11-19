use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CameraSettings {
    #[serde(default)]
    pub tile_size: u32,
    #[serde(default)]
    pub snap_distance: f32,
    #[serde(default)]
    pub camera_speed: f32,
}

impl Default for CameraSettings {
    fn default() -> Self {
        Self {
            tile_size: 32,
            snap_distance: 0.1,
            camera_speed: 0.3,
        }
    }
}
