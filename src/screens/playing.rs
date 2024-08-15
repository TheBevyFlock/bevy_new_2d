//! The screen state for the main game loop.

use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use super::Screen;
use crate::{asset_tracking::LoadResource, audio::Music, demo::level::SpawnLevel};

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<GameplayMusic>();
    app.add_systems(OnEnter(Screen::Playing), spawn_level);
    app.add_systems(OnExit(Screen::Playing), stop_bgm);

    app.add_systems(
        Update,
        return_to_title_screen
            .run_if(in_state(Screen::Playing).and_then(input_just_pressed(KeyCode::Escape))),
    );
}

#[derive(Resource, Asset, Reflect, Clone)]
pub struct GameplayMusic {
    #[dependency]
    handle: Handle<AudioSource>,
    entity: Option<Entity>,
}

impl FromWorld for GameplayMusic {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            handle: assets.load("audio/bgm/Fluffing A Duck.ogg"),
            entity: None,
        }
    }
}

fn spawn_level(mut commands: Commands, mut music: ResMut<GameplayMusic>) {
    commands.add(SpawnLevel);
    music.entity = Some(
        commands
            .spawn((
                AudioBundle {
                    source: music.handle.clone(),
                    settings: PlaybackSettings::LOOP,
                },
                Music,
            ))
            .id(),
    );
}

fn stop_bgm(mut commands: Commands, mut music: ResMut<GameplayMusic>) {
    if let Some(entity) = music.entity.take() {
        commands.entity(entity).despawn_recursive();
    }
}

fn return_to_title_screen(mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Title);
}
