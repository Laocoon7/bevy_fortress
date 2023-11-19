use bevy::prelude::*;

use crate::prelude::Animation;

#[derive(Event, Clone)]
pub enum AnimationEvent {
    /// The animation has finished a full animation and is starting a new `AnimationType::LOOP`
    Finished((Entity, AssetId<Animation>)),
    /// The animation has ended due to `AnimationType::ONESHOT`
    End((Entity, AssetId<Animation>)),
}
