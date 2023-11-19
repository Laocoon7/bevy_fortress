use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Holds the `Timer` information for an animation
#[derive(Serialize, Deserialize, Component, Reflect, Default, Debug, Clone)]
#[reflect(Serialize, Deserialize, Component)]
pub struct AnimationTimer {
    pub timer: Timer,
    pub stamp_index: usize,
}
