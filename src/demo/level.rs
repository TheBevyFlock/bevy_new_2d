//! Spawn the main level.

use bevy::prelude::*;

use crate::demo::player::{PlayerAssets, player};

/// A system that spawns the main level.
pub fn spawn_level(
    mut commands: Commands,
    player_assets: Res<PlayerAssets>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn(level(&player_assets, &mut texture_atlas_layouts));
}

/// The main level.
fn level(
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
