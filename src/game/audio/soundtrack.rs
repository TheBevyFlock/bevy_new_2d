use bevy::{audio::PlaybackMode, ecs::world::Command, prelude::*};

use crate::game::assets::SoundtrackHandles;

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
    fn apply(self, world: &mut World) {
        // When this command is applied, we remove all existing soundtracks.
        // Then, if we want to play a soundtrack, we spawn a new one.

        let mut soundtrack_query = world.query_filtered::<Entity, With<IsSoundtrack>>();
        let soundtracks: Vec<_> = soundtrack_query.iter(world).collect();
        for entity in soundtracks {
            world.entity_mut(entity).despawn_recursive();
        }

        let soundtrack_key = match &self {
            PlaySoundtrack::Key(key) => key.clone(),
            PlaySoundtrack::Disable => return,
        };

        let soundtrack_handles = world.resource::<SoundtrackHandles>();
        world.spawn((
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
}

pub trait SoundtrackCommands {
    /// Call this function to play the soundtrack.
    /// Playing a new soundtrack will overwrite the previous one.
    /// Soundtracks will loop.
    fn play_soundtrack(&mut self, name: impl Into<String>);

    fn stop_current_soundtrack(&mut self);
}

impl<'w, 's> SoundtrackCommands for Commands<'w, 's> {
    fn play_soundtrack(&mut self, name: impl Into<String>) {
        self.add(PlaySoundtrack::Key(name.into()));
    }

    fn stop_current_soundtrack(&mut self) {
        self.add(PlaySoundtrack::Disable);
    }
}
