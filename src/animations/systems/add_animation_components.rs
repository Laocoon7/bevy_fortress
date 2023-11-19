use bevy::prelude::*;

use crate::{
    animations::components::AnimationAssetId,
    prelude::{AnimationTimer, SetAnimation},
};

pub fn add_animation_components(
    mut commands: Commands,
    q_animations: Query<(Entity, Option<&AnimationTimer>, Option<&AnimationAssetId>), Changed<SetAnimation>>,
) {
    for (entity, maybe_timer, maybe_asset_id) in q_animations.iter() {
        if maybe_timer.is_none() {
            commands.entity(entity).insert(AnimationTimer::default());
        }
        if maybe_asset_id.is_none() {
            commands.entity(entity).insert(AnimationAssetId::default());
        }
    }
}
