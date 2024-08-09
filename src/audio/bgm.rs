use bevy::{
    audio::PlaybackMode,
    ecs::{system::RunSystemOnce as _, world::Command},
    prelude::*,
};

use crate::assets::BgmHandles;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<IsSoundtrack>();
}

/// Marker component for the soundtrack entity so we can find it later.
#[derive(Component, Reflect)]
#[reflect(Component)]
struct IsSoundtrack;

/// A custom command used to play soundtracks.
#[derive(Debug)]
enum PlaySoundtrack {
    Key(String),
    Disable,
}

impl Command for PlaySoundtrack {
    /// This command will despawn the current soundtrack, then spawn a new one
    /// if necessary.
    fn apply(self, world: &mut World) {
        world.run_system_once_with(self, play_soundtrack);
    }
}

fn play_soundtrack(
    In(config): In<PlaySoundtrack>,
    mut commands: Commands,
    soundtrack_query: Query<Entity, With<IsSoundtrack>>,
    soundtrack_handles: Res<BgmHandles>,
) {
    for entity in soundtrack_query.iter() {
        commands.entity(entity).despawn_recursive();
    }

    let soundtrack_key = match config {
        PlaySoundtrack::Key(key) => key,
        PlaySoundtrack::Disable => return,
    };

    commands.spawn((
        AudioSourceBundle {
            source: soundtrack_handles[&soundtrack_key].clone_weak(),
            settings: PlaybackSettings {
                mode: PlaybackMode::Loop,
                ..default()
            },
        },
        IsSoundtrack,
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
        self.add(PlaySoundtrack::Key(name.into()));
    }

    fn stop_bgm(&mut self) {
        self.add(PlaySoundtrack::Disable);
    }
}
