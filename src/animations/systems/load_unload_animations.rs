use bevy::prelude::*;

use crate::prelude::{Animation, LoadedAnimations};

pub fn load_unload_animations(
    mut asset_events: EventReader<AssetEvent<Animation>>,
    animation_assets: Res<Assets<Animation>>,
    mut loaded_animations: ResMut<LoadedAnimations>,
) {
    for event in asset_events.read() {
        match event {
            AssetEvent::Added { id } => {
                if let Some(animation) = animation_assets.get(id.clone()) {
                    loaded_animations.add_animation(&animation.name, id)
                }
            },
            AssetEvent::Removed { id } => {
                loaded_animations.remove_animation(id);
            },
            _ => (),
        }
    }
}
