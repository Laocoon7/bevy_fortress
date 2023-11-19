use bevy::prelude::*;

use crate::{
    animations::{components::AnimationAssetId, events::AnimationEvent},
    prelude::{
        tags::{AnimateOneShotTag, AnimatePingPongTag, AnimateRealtimeTag, AnimateReverseTag, AnimateTag},
        Animation, AnimationTimer,
    },
};

pub fn animate(
    mut commands: Commands,
    real_time: Res<Time<Real>>,
    virtual_time: Res<Time<Virtual>>,

    animation_assets: Res<Assets<Animation>>,

    mut animation_events: EventWriter<AnimationEvent>,

    mut q_animations_real: Query<
        (
            Entity,
            &mut TextureAtlasSprite,
            &mut AnimationTimer,
            &AnimationAssetId,
            Option<&AnimateOneShotTag>,
            Option<&AnimatePingPongTag>,
            Option<&AnimateReverseTag>,
        ),
        (With<AnimateTag>, With<AnimateRealtimeTag>),
    >,
    mut q_animations_virtual: Query<
        (
            Entity,
            &mut TextureAtlasSprite,
            &mut AnimationTimer,
            &AnimationAssetId,
            Option<&AnimateOneShotTag>,
            Option<&AnimatePingPongTag>,
            Option<&AnimateReverseTag>,
        ),
        (With<AnimateTag>, Without<AnimateRealtimeTag>),
    >,
) {
    let mut process_animation = |entity,
                                 delta,
                                 sprite: &mut TextureAtlasSprite,
                                 animation_timer: &mut AnimationTimer,
                                 animation_asset_id: &AssetId<Animation>,
                                 one_shot: bool,
                                 ping_pong: bool,
                                 reverse: bool| {
        let Some(animation) = animation_assets.get(animation_asset_id.clone()) else {
            error!("Failed to get animation asset for {:?}", animation_asset_id);
            return;
        };

        animation_timer.timer.tick(delta);

        let mut index = animation_timer.stamp_index as i32;
        let times_finished = animation_timer.timer.times_finished_this_tick() as i32;
        let len = animation.stamps.len() as i32;

        let index = if ping_pong {
            // We are running a PingPong animation
            if reverse {
                // We are running the second half of the animation
                index -= times_finished;
                if index < 0 {
                    if one_shot {
                        // Remove our tags, and stop animating
                        commands.entity(entity).remove::<AnimateReverseTag>().remove::<AnimateTag>();
                        // Send animation end event
                        animation_events.send(AnimationEvent::End((entity, animation_asset_id.clone())));
                        // lock index to the last frame
                        0
                    } else {
                        // Remove our tags
                        commands.entity(entity).remove::<AnimateReverseTag>();
                        // Send animation finished event
                        animation_events.send(AnimationEvent::Finished((
                            entity,
                            animation_asset_id.clone(),
                        )));
                        // Calculate and set index
                        let len_ping_pong = (len * 2) - 2;
                        while index < 0 {
                            // technically this should reverse each time... but eh...
                            index += len_ping_pong;
                        }
                        len_ping_pong - index
                    }
                } else {
                    // Animation still running
                    index
                }
            } else {
                // We are running the first half of the animation
                index += times_finished;
                if index >= len {
                    // Insert our tags
                    commands.entity(entity).insert(AnimateReverseTag);
                    // Calculate and set index
                    let len_ping_pong = (len * 2) - 2;
                    while index > len_ping_pong {
                        // technically this should reverse each time... but eh...
                        index -= len_ping_pong;
                    }
                    len_ping_pong - index
                } else {
                    // Animation still running
                    index
                }
            }
        } else {
            if reverse {
                // We are running in reverse
                index -= times_finished;
                if index < 0 {
                    if one_shot {
                        // Stop animating
                        commands.entity(entity).remove::<AnimateTag>();
                        // Send animation end event
                        animation_events.send(AnimationEvent::End((entity, animation_asset_id.clone())));
                        // Lock index to last frame
                        0
                    } else {
                        // Send animation finished event
                        animation_events.send(AnimationEvent::Finished((
                            entity,
                            animation_asset_id.clone(),
                        )));
                        // Calculate and set index
                        while index < 0 {
                            index += len;
                        }
                        index
                    }
                } else {
                    // Animation still running
                    index
                }
            } else {
                // We are running normally
                index += times_finished;
                if index >= len {
                    if one_shot {
                        // Stop animating
                        commands.entity(entity).remove::<AnimateTag>();
                        // Send animation end event
                        animation_events.send(AnimationEvent::End((entity, animation_asset_id.clone())));
                        // Lock index to the last frame
                        len - 1
                    } else {
                        // Send animation finished event
                        animation_events.send(AnimationEvent::Finished((
                            entity,
                            animation_asset_id.clone(),
                        )));
                        // Calculate and set index
                        index % len
                    }
                } else {
                    // Animation still running
                    index
                }
            }
        };

        animation_timer.stamp_index = index as usize;
        let stamp = &animation.stamps[animation_timer.stamp_index];

        sprite.index = stamp.index;
        sprite.color = stamp.color;
        sprite.flip_x = stamp.flip_x;
        sprite.flip_y = stamp.flip_y;
        sprite.custom_size = animation.custom_size;
    };

    for (
        entity,
        mut sprite,
        mut animation_timer,
        animation_asset_id,
        maybe_one_shot,
        maybe_ping_pong,
        maybe_reverse,
    ) in q_animations_real.iter_mut()
    {
        process_animation(
            entity,
            real_time.delta(),
            &mut sprite,
            &mut animation_timer,
            &animation_asset_id.0,
            maybe_one_shot.is_some(),
            maybe_ping_pong.is_some(),
            maybe_reverse.is_some(),
        );
    }

    for (entity, mut sprite, mut animation_timer, animation_asset_id, one_shot, ping_pong, reverse) in
        q_animations_virtual.iter_mut()
    {
        process_animation(
            entity,
            virtual_time.delta(),
            &mut sprite,
            &mut animation_timer,
            &animation_asset_id.0,
            one_shot.is_some(),
            ping_pong.is_some(),
            reverse.is_some(),
        );
    }
}
