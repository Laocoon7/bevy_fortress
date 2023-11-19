use bevy::prelude::*;

use crate::{
    animations::{components::AnimationAssetId, loaders::assets::AnimationType},
    prelude::{
        tags::{AnimateOneShotTag, AnimatePingPongTag, AnimateReverseTag, AnimateTag},
        Animation, AnimationTimer, LoadedAnimations, SetAnimation,
    },
};

pub fn set_animations(
    mut commands: Commands,
    // asset_server: Res<AssetServer>,
    animation_assets: Res<Assets<Animation>>,
    loaded_animations: Res<LoadedAnimations>,
    mut q_animations: Query<(
        Entity,
        &mut TextureAtlasSprite,
        &mut Handle<TextureAtlas>,
        &mut AnimationTimer,
        &mut AnimationAssetId,
        &SetAnimation,
    )>,
) {
    for (entity, mut sprite, mut texture_atlas, mut animation_timer, mut animation_asset_id, set_animation) in
        q_animations.iter_mut()
    {
        // Lookup the next AssetId
        let Some(next_animation_id) = loaded_animations.get(&set_animation.0) else {
            error!(
                "Failed to find animation in LoadedAnimations: {}",
                &set_animation.0
            );
            continue;
        };

        // Lookup the next Animation
        let Some(animation) = animation_assets.get(next_animation_id.clone()) else {
            error!(
                "Failed to find animation in Assets<Animation>: {}",
                &set_animation.0
            );
            continue;
        };

        // Reset the AsseetId
        animation_asset_id.0 = next_animation_id;

        // Reset the AnimationTimer
        animation_timer.timer = Timer::from_seconds(animation.frame_time, TimerMode::Repeating);
        animation_timer.stamp_index = 0;

        // Reset the TextureAtlas
        *texture_atlas = animation.texture_atlas.clone();

        // Build the TextureAtlasSprite from the stamp
        let stamp = &animation.stamps[animation_timer.stamp_index];

        sprite.index = stamp.index;
        sprite.color = stamp.color;
        sprite.flip_x = stamp.flip_x;
        sprite.flip_y = stamp.flip_y;
        sprite.custom_size = animation.custom_size;

        // Add / Remove OneShotTag
        if animation.animation_type.contains(AnimationType::ONESHOT) {
            commands.entity(entity).insert(AnimateOneShotTag);
        } else {
            commands.entity(entity).remove::<AnimateOneShotTag>();
        }

        // Add / Remove PingPongTag
        if animation.animation_type.contains(AnimationType::PINGPONG) {
            commands.entity(entity).insert(AnimatePingPongTag);
        } else {
            commands.entity(entity).remove::<AnimatePingPongTag>();
        }

        // Add / Remove ReverseTag
        if animation.animation_type.contains(AnimationType::REVERSE) {
            commands.entity(entity).insert(AnimateReverseTag);
        } else {
            commands.entity(entity).remove::<AnimateReverseTag>();
        }

        // Add AnimateTag
        commands.entity(entity).insert(AnimateTag);

        // Remove SetAnimation
        commands.entity(entity).remove::<SetAnimation>();
    }
}
