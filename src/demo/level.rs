//! Spawn the main level.

use bevy::prelude::*;

use crate::{
    demo::player::{PlayerAssets, player},
    screens::Screen,
};

/// A system that spawns the main level.
pub fn spawn_level(
    mut commands: Commands,
    player_assets: Res<PlayerAssets>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn((
        Name::new("Level"),
        Transform::default(),
        Visibility::default(),
        StateScoped(Screen::Gameplay),
        children![player(400.0, &player_assets, &mut texture_atlas_layouts)],
    ));
}
