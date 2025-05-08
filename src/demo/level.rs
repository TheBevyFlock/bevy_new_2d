//! Spawn the main level.

use bevy::prelude::*;

use crate::{
    asset_tracking::LoadResource,
    audio::music,
    demo::player::{PlayerAssets, player},
    screens::Screen,
};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<LevelAssets>();
    app.load_resource::<LevelAssets>();
}

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

#[derive(Component, Reflect)]
#[reflect(Component)]
pub(crate) struct LevelMusicPlayer;

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub(crate) struct LevelAssets {
    #[dependency]
    music: Handle<AudioSource>,
}

impl FromWorld for LevelAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            music: assets.load("audio/music/Fluffing A Duck.ogg"),
        }
    }
}

pub(crate) fn start_gameplay_music(mut commands: Commands, level_assets: Res<LevelAssets>) {
    commands.spawn((music(level_assets.music.clone()), LevelMusicPlayer));
}

pub(crate) fn stop_gameplay_music(
    mut commands: Commands,
    music_players: Query<Entity, With<LevelMusicPlayer>>,
) {
    for entity in music_players.iter() {
        commands.entity(entity).despawn();
    }
}
