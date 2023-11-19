use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SoundSettings {
    #[serde(default)]
    pub overall_volume: f32,
}

impl Default for SoundSettings {
    fn default() -> Self {
        Self {
            overall_volume: 0.5,
        }
    }
}
