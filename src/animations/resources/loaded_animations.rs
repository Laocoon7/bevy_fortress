use bevy::{prelude::*, utils::HashMap};

use crate::prelude::Animation;

/// Holds a lookup table for `Animation.name` to `AssetId<Animation>`
#[derive(Resource, Default)]
pub struct LoadedAnimations {
    animations: HashMap<String, AssetId<Animation>>,
}

impl LoadedAnimations {
    pub(crate) fn add_animation(&mut self, name: &str, id: &AssetId<Animation>) {
        self.animations.insert(name.to_string(), id.clone());
    }

    pub(crate) fn remove_animation(&mut self, id: &AssetId<Animation>) {
        self.animations.retain(|_name, asset_id| *asset_id != *id);
    }

    #[allow(dead_code)]
    pub(crate) fn remove_animation_by_name(&mut self, name: &str) { self.animations.remove(name); }

    /// Retrieve the `AssetId<Animation>` by `AnimationName`
    pub fn get(&self, name: &str) -> Option<AssetId<Animation>> { self.animations.get(name).cloned() }
}
