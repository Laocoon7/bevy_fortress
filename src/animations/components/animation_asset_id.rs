use bevy::prelude::*;

use crate::prelude::Animation;

/// Holds the current `AssetId<Animation>` for the given Entity.
#[derive(Component, Reflect, Default, Debug, Clone)]
#[reflect(Component)]
pub struct AnimationAssetId(pub AssetId<Animation>);
