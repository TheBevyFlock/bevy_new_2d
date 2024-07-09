use bevy::{audio::PlaybackMode, prelude::*};
use rand::prelude::SliceRandom;

pub fn plugin(app: &mut App) {
    app.observe(play_soundtrack);
    app.observe(play_sfx);
}

fn play_soundtrack(
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
        Soundtrack::Credits => "audio/soundtracks/Monkeys Spinning Monkeys.ogg",
        Soundtrack::Gameplay => "audio/soundtracks/Fluffing A Duck.ogg",
        Soundtrack::Disable => {
            return;
        }
    };
    let source = asset_server.load::<AudioSource>(path);
    let settings = PlaybackSettings {
        mode: PlaybackMode::Loop,
        ..default()
    };
    commands.spawn((AudioSourceBundle { source, settings }, SoundtrackMarker));
}

fn play_sfx(trigger: Trigger<Sfx>, mut commands: Commands, asset_server: Res<AssetServer>) {
    let event = trigger.event();
    let mut rng = rand::thread_rng();
    let path = match event {
        Sfx::ButtonHover => "audio/sfx/button_hover.ogg",
        Sfx::ButtonPress => "audio/sfx/button_press.ogg",
        Sfx::Step => [
            "audio/sfx/step1.ogg",
            "audio/sfx/step2.ogg",
            "audio/sfx/step3.ogg",
            "audio/sfx/step4.ogg",
        ]
        .choose(&mut rng)
        .unwrap(),
    };
    let source = asset_server.load::<AudioSource>(path);
    let settings = PlaybackSettings {
        mode: PlaybackMode::Despawn,
        ..default()
    };
    commands.spawn(AudioSourceBundle { source, settings });
}

/// We mark our soundtrack entity so we can find it later.
#[derive(Component)]
struct SoundtrackMarker;

/// Play or disable soundtrack.
/// Playing a new soundtrack will overwrite the previous one.
/// Soundtracks will loop.
#[derive(Event)]
pub enum Soundtrack {
    Credits,
    Gameplay,
    Disable,
}

/// Play an a single sound effect.
#[derive(Event)]
pub enum Sfx {
    ButtonHover,
    ButtonPress,
    Step,
}
