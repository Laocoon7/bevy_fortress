use bevy::{asset::LoadContext, prelude::*};

use super::assets::{Animation, AtlasRecord};

// FnLoader
pub fn load_animations(load_context: &mut LoadContext, atlas_record: AtlasRecord) -> () {
    // Load the `TextureAtlas`
    let atlas_handle = load_context.labeled_asset_scope(atlas_record.name.clone(), |lc| {
        // Load the texture image
        let texture_handle = lc.load(&atlas_record.texture_path);

        // Build the `TextureAtlas`
        TextureAtlas::from_grid(
            texture_handle,
            atlas_record.tile_size,
            atlas_record.columns,
            atlas_record.rows,
            atlas_record.padding,
            atlas_record.offset,
        )
    });

    for animation_record in atlas_record.animations.iter() {
        load_context.labeled_asset_scope(animation_record.name.clone(), |_lc| {
            // Build the `Animation`
            Animation {
                name: animation_record.name.clone(),
                texture_atlas: atlas_handle.clone(),
                animation_type: animation_record.animation_type,
                frame_time: 1.0 / if animation_record.fps == 0.0 { 1.0 } else { animation_record.fps },
                custom_size: animation_record.custom_size,
                stamps: animation_record.stamps.clone(),
            }
        });
    }

    ()
}
