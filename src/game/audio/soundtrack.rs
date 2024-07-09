use bevy::{audio::PlaybackMode, prelude::*};

pub(super) fn play_soundtrack(
    trigger: Trigger<Soundtrack>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<Entity, With<SoundtrackMarker>>,
) {
    let event = trigger.event();
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
    let path = match event {
        Soundtrack::Disable => {
            return;
        }
        Soundtrack::Credits => "audio/soundtracks/Monkeys Spinning Monkeys.ogg",
        Soundtrack::Gameplay => "audio/soundtracks/Fluffing A Duck.ogg",
    };
    let source = asset_server.load::<AudioSource>(path);
    let settings = PlaybackSettings {
        mode: PlaybackMode::Loop,
        ..default()
    };
    commands.spawn((AudioSourceBundle { source, settings }, SoundtrackMarker));
}

/// We mark our soundtrack entity so we can find it later.
#[derive(Component)]
pub(super) struct SoundtrackMarker;

/// Play or disable soundtrack.
/// Playing a new soundtrack will overwrite the previous one.
/// Soundtracks will loop.
#[derive(Event)]
pub enum Soundtrack {
    Credits,
    Gameplay,
    Disable,
}
