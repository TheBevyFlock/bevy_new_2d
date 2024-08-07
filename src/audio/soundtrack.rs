use bevy::{
    audio::PlaybackMode,
    ecs::{system::RunSystemOnce as _, world::Command},
    prelude::*,
};

use crate::assets::SoundtrackHandles;

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
        world.run_system_once_with(self, apply_play_soundstrack);
    }
}

fn apply_play_soundstrack(
    In(play_soundtrack): In<PlaySoundtrack>,
    mut commands: Commands,
    soundtracks: Query<Entity, With<IsSoundtrack>>,
    soundtrack_handles: Res<SoundtrackHandles>,
) {
    for entity in soundtracks.iter() {
        commands.entity(entity).despawn_recursive();
    }

    let soundtrack_key = match play_soundtrack {
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
pub trait SoundtrackCommands {
    /// Play a soundtrack, replacing the current one.
    /// Soundtracks will loop.
    fn play_soundtrack(&mut self, name: impl Into<String>);

    /// Stop the current soundtrack.
    fn stop_current_soundtrack(&mut self);
}

impl SoundtrackCommands for Commands<'_, '_> {
    fn play_soundtrack(&mut self, name: impl Into<String>) {
        self.add(PlaySoundtrack::Key(name.into()));
    }

    fn stop_current_soundtrack(&mut self) {
        self.add(PlaySoundtrack::Disable);
    }
}
