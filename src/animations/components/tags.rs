use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Component, Reflect, Default, Debug, Clone, Copy)]
#[reflect(Serialize, Deserialize, Component)]
pub struct AnimateTag;

/// Insert this component into an Entity to play the animation based on `Real` time instead of
/// `Virtual` time.
#[derive(Serialize, Deserialize, Component, Reflect, Default, Debug, Clone, Copy)]
#[reflect(Serialize, Deserialize, Component)]
pub struct AnimateRealtimeTag;

#[derive(Serialize, Deserialize, Component, Reflect, Default, Debug, Clone, Copy)]
#[reflect(Serialize, Deserialize, Component)]
pub struct AnimateOneShotTag;

#[derive(Serialize, Deserialize, Component, Reflect, Default, Debug, Clone, Copy)]
#[reflect(Serialize, Deserialize, Component)]
pub struct AnimatePingPongTag;

#[derive(Serialize, Deserialize, Component, Reflect, Default, Debug, Clone, Copy)]
#[reflect(Serialize, Deserialize, Component)]
pub struct AnimateReverseTag;
