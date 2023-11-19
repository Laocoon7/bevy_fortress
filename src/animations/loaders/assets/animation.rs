use bevy::prelude::*;
use bitflags::bitflags;
use serde::{Deserialize, Serialize};

/// Defines an Animation to be played
#[derive(Asset, TypePath, Clone)]
pub struct Animation {
    pub name: String,
    pub texture_atlas: Handle<TextureAtlas>,
    pub animation_type: AnimationType,
    pub frame_time: f32,
    pub custom_size: Option<Vec2>,
    pub stamps: Vec<Stamp>,
}

/// Defines how each `AnimationRecord` should be deserialized
#[derive(Deserialize, Clone)]
pub struct AnimationRecord {
    pub name: String,
    pub animation_type: AnimationType,
    pub fps: f32,
    pub custom_size: Option<Vec2>,
    pub stamps: Vec<Stamp>,
}

#[rustfmt::skip]
bitflags! {
    /// Defines how the animation should work
    #[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
    pub struct AnimationType: u8 {
        /// Loop through the animation before starting over from the beginning
        const LOOP      = 0b0000_0000;
        /// Run through the animation before stopping on the last frame
        const ONESHOT   = 0b0000_0001;
        /// Reverse the animation does not work with PINGPONG
        const REVERSE   = 0b0000_0010;
        /// Run through the animation before running the animation in reverse
        const PINGPONG  = 0b0000_0100;
    }
}

impl Default for AnimationType {
    fn default() -> Self { Self::LOOP }
}

/// Defines a frame to be played in an animation
#[derive(Serialize, Deserialize, Clone)]
pub struct Stamp {
    pub index: usize,
    #[serde(default)]
    pub color: Color,
    #[serde(default)]
    pub flip_x: bool,
    #[serde(default)]
    pub flip_y: bool,
}
