//! The screen state for the main gameplay.

use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use crate::{
    asset_tracking::LoadResource, audio::music, demo::level::spawn_level, screens::Screen,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay), spawn_level);

    app.register_type::<GameplayMusic>();
    app.load_resource::<GameplayMusic>();
    app.add_systems(OnEnter(Screen::Gameplay), start_gameplay_music);
    app.add_systems(OnExit(Screen::Gameplay), stop_gameplay_music);

    app.add_systems(
        Update,
        return_to_title_screen
            .run_if(in_state(Screen::Gameplay).and(input_just_pressed(KeyCode::Escape))),
    );
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
struct GameplayMusic {
    #[dependency]
    handle: Handle<AudioSource>,
    entity: Option<Entity>,
}

impl FromWorld for GameplayMusic {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            handle: assets.load("audio/music/Fluffing A Duck.ogg"),
            entity: None,
        }
    }
}

fn start_gameplay_music(mut commands: Commands, mut gameplay_music: ResMut<GameplayMusic>) {
    let handle = gameplay_music.handle.clone();
    gameplay_music.entity = Some(commands.spawn(music(handle)).id());
}

fn stop_gameplay_music(mut commands: Commands, mut gameplay_music: ResMut<GameplayMusic>) {
    if let Some(entity) = gameplay_music.entity.take() {
        commands.entity(entity).despawn();
    }
}

fn return_to_title_screen(mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Title);
}
