//! A splash screen that plays briefly at startup.

use bevy::{asset::embedded_asset, prelude::*};

use super::Screen;
use crate::ui_tools::prelude::*;

pub(super) fn plugin(app: &mut App) {
    // Add splash image
    embedded_asset!(app, "splash.png");
    app.add_systems(OnEnter(Screen::Splash), spawn_splash)
        // Make the screen-flicker bug less jarring on Windows (see `core/deflicker.rs`).
        .insert_resource(ClearColor(SPLASH_BACKGROUND_COLOR));

    // Add splash timer
    app.add_systems(OnEnter(Screen::Splash), insert_splash_timer)
        .add_systems(OnExit(Screen::Splash), remove_splash_timer)
        .add_systems(
            Update,
            (tick_splash_timer, check_splash_timer)
                .chain()
                .run_if(in_state(Screen::Splash)),
        );
}

const SPLASH_BACKGROUND_COLOR: Color = Color::srgb(0.157, 0.157, 0.157);

fn spawn_splash(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .ui_root()
        .insert((
            BackgroundColor(SPLASH_BACKGROUND_COLOR),
            StateScoped(Screen::Splash),
        ))
        .with_children(|children| {
            children.spawn(ImageBundle {
                style: Style {
                    margin: UiRect::all(Val::Auto),
                    width: Val::Percent(70.0),
                    ..default()
                },
                image: UiImage::new(
                    asset_server.load("embedded://bevy_template/screen/splash/splash.png"),
                ),
                ..default()
            });
        });
}

const SPLASH_DURATION_SECS: f32 = 1.2;

#[derive(Resource)]
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
