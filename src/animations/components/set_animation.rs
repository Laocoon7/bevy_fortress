use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Insert this component into an entity in order to change the animation to be played
#[derive(Serialize, Deserialize, Component, Reflect, Default, Debug, Clone)]
#[reflect(Serialize, Deserialize, Component)]
pub struct SetAnimation(pub String);
