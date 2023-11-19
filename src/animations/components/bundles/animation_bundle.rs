use bevy::prelude::*;

use crate::prelude::SetAnimation;

/// A bundle used for adding components to an entity in order to animate the sprite.
#[derive(Bundle, Default)]
pub struct AnimationBundle {
    pub sprite_sheet_bundle: SpriteSheetBundle,
    pub set_animation: SetAnimation,
}
