use bevy::prelude::*;

use super::{
    events::AnimationEvent,
    loaders::load_animations,
    resources::LoadedAnimations,
    systems::{add_animation_components, animate, load_unload_animations, set_animations},
};
use crate::prelude::{Animation, FnLoader};

/// Add this plugin to use the animation loader and systems
pub struct AnimationsPlugin;
impl Plugin for AnimationsPlugin {
    fn build(&self, app: &mut App) {
        app.register_asset_loader(FnLoader::new(vec!["atlas"], load_animations)).init_asset::<Animation>();

        app.init_resource::<LoadedAnimations>();

        app.add_event::<AnimationEvent>();

        app.add_systems(First, load_unload_animations);

        app.add_systems(PreUpdate, animate); // Last?

        app.add_systems(
            PostUpdate,
            ((add_animation_components, apply_deferred, set_animations).chain(),),
        );
    }
}
