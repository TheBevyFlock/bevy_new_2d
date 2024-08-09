use bevy::{
    audio::PlaybackMode,
    ecs::{system::RunSystemOnce as _, world::Command},
    prelude::*,
};

use crate::assets::BgmHandles;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<IsBgm>();
}

/// Marker component for the soundtrack entity so we can find it later.
#[derive(Component, Reflect)]
#[reflect(Component)]
struct IsBgm;

/// A custom command used to play soundtracks.
#[derive(Debug)]
enum PlayBgm {
    Key(String),
    Disable,
}

impl Command for PlayBgm {
    /// This command will despawn the current soundtrack, then spawn a new one
    /// if necessary.
    fn apply(self, world: &mut World) {
        world.run_system_once_with(self, play_bgm);
    }
}

fn play_bgm(
    In(config): In<PlayBgm>,
    mut commands: Commands,
    bgm_query: Query<Entity, With<IsBgm>>,
    bgm_handles: Res<BgmHandles>,
) {
    for entity in bgm_query.iter() {
        commands.entity(entity).despawn_recursive();
    }

    let bgm_key = match config {
        PlayBgm::Key(key) => key,
        PlayBgm::Disable => return,
    };

    commands.spawn((
        AudioSourceBundle {
            source: bgm_handles[&bgm_key].clone_weak(),
            settings: PlaybackSettings {
                mode: PlaybackMode::Loop,
                ..default()
            },
        },
        IsBgm,
    ));
}

/// An extension trait with convenience methods for soundtrack commands.
pub trait BgmCommands {
    /// Play a soundtrack, replacing the current one.
    /// Soundtracks will loop.
    fn play_bgm(&mut self, name: impl Into<String>);

    /// Stop the current soundtrack.
    fn stop_bgm(&mut self);
}

impl BgmCommands for Commands<'_, '_> {
    fn play_bgm(&mut self, name: impl Into<String>) {
        self.add(PlayBgm::Key(name.into()));
    }

    fn stop_bgm(&mut self) {
        self.add(PlayBgm::Disable);
    }
}
