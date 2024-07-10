//! A splash screen that plays briefly at startup.

use bevy::{
    prelude::*,
    render::texture::{ImageLoaderSettings, ImageSampler},
};

use super::Screen;
use crate::{ui_tools::prelude::*, AppStep};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Splash), spawn_splash);
    app.insert_resource(ClearColor(SPLASH_BACKGROUND_COLOR));

    // Add splash timer
    app.register_type::<SplashTimer>();
    app.add_systems(OnEnter(Screen::Splash), insert_splash_timer);
    app.add_systems(OnExit(Screen::Splash), remove_splash_timer);
    app.add_systems(
        Update,
        (
            tick_splash_timer.in_set(AppStep::TickTimers),
            check_splash_timer.in_set(AppStep::Update),
        )
            .run_if(in_state(Screen::Splash)),
    );
}

const SPLASH_BACKGROUND_COLOR: Color = Color::srgb(0.157, 0.157, 0.157);

fn spawn_splash(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .ui_root()
        .insert((
            Name::new("Splash screen"),
            BackgroundColor(SPLASH_BACKGROUND_COLOR),
            StateScoped(Screen::Splash),
        ))
        .with_children(|children| {
            children.spawn((
                Name::new("Splash image"),
                ImageBundle {
                    style: Style {
                        margin: UiRect::all(Val::Auto),
                        width: Val::Percent(70.0),
                        ..default()
                    },
                    image: UiImage::new(asset_server.load_with_settings(
                        // This should be an embedded asset for instant loading, but that is
                        // currently [broken on Windows Wasm builds](https://github.com/bevyengine/bevy/issues/14246).
                        "splash.png",
                        |settings: &mut ImageLoaderSettings| {
                            // Make an exception for the splash image in case
                            // `ImagePlugin::default_nearest()` is used for pixel art.
                            settings.sampler = ImageSampler::linear();
                        },
                    )),
                    ..default()
                },
            ));
        });
}

const SPLASH_DURATION_SECS: f32 = 1.2;

#[derive(Resource, Debug, Clone, PartialEq, Reflect)]
#[reflect(Resource)]
struct SplashTimer(Timer);

impl Default for SplashTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(SPLASH_DURATION_SECS, TimerMode::Once))
    }
}

fn insert_splash_timer(mut commands: Commands) {
    commands.init_resource::<SplashTimer>();
}

fn remove_splash_timer(mut commands: Commands) {
    commands.remove_resource::<SplashTimer>();
}

fn tick_splash_timer(time: Res<Time>, mut timer: ResMut<SplashTimer>) {
    timer.0.tick(time.delta());
}

fn check_splash_timer(timer: ResMut<SplashTimer>, mut next_screen: ResMut<NextState<Screen>>) {
    if timer.0.just_finished() {
        next_screen.set(Screen::Title);
    }
}
