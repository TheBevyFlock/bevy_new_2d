//! Spawn the main level.

use bevy::prelude::*;

use crate::demo::player::{PlayerAssets, player};

/// The main level.
pub fn level(
    player_assets: &PlayerAssets,
    texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
) -> impl Bundle {
    (
        Name::new("Level"),
        Transform::default(),
        Visibility::default(),
        children![player(400.0, player_assets, texture_atlas_layouts)],
    )
}
