//! A splash screen that plays briefly at startup.

use bevy::{
    asset::load_internal_binary_asset,
    prelude::*,
    render::{
        render_asset::RenderAssetUsages,
        texture::{ImageSampler, ImageType},
    },
};

use super::Screen;
use crate::ui_tools::prelude::*;

pub(super) fn plugin(app: &mut App) {
    // Add splash image as an embedded asset. We do this here because the splash
    // screen is the first screen to display, so you don't have to wait for the
    // asset to load. Note that Bevy supports a short version of the boilerplate
    // below in form of the `embedded_asset!` macro, but that does currently
    // [not work on Windows Wasm builds](https://github.com/bevyengine/bevy/issues/14246).
    load_internal_binary_asset!(
        app,
        SPLASH_IMAGE_HANDLE,
        "splash.png",
        |bytes, _path: String| {
            Image::from_buffer(
                bytes,
                ImageType::Extension("png"),
                default(),
                true,
                ImageSampler::linear(),
                RenderAssetUsages::RENDER_WORLD | RenderAssetUsages::MAIN_WORLD,
            )
            .unwrap()
        }
    );
    app.add_systems(OnEnter(Screen::Splash), spawn_splash);
    app.insert_resource(ClearColor(SPLASH_BACKGROUND_COLOR));

    // Add splash timer
    app.register_type::<SplashTimer>();
    app.add_systems(OnEnter(Screen::Splash), insert_splash_timer);
    app.add_systems(OnExit(Screen::Splash), remove_splash_timer);
    app.add_systems(
        Update,
        (tick_splash_timer, check_splash_timer)
            .chain()
            .run_if(in_state(Screen::Splash)),
    );
}

const SPLASH_IMAGE_HANDLE: Handle<Image> =
    Handle::weak_from_u128(145948501136218819748366695396142082633);
const SPLASH_BACKGROUND_COLOR: Color = Color::srgb(0.157, 0.157, 0.157);

fn spawn_splash(mut commands: Commands) {
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
                    image: UiImage::new(SPLASH_IMAGE_HANDLE),
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
