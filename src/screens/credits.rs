//! A credits screen that can be accessed from the title screen.

use bevy::prelude::*;

use super::Screen;
use crate::{asset_tracking::LoadResource, audio::BackgroundMusic, theme::prelude::*};

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<CreditsMusic>();
    app.add_systems(OnEnter(Screen::Credits), show_credits_screen);
    app.add_systems(OnExit(Screen::Credits), stop_bgm);
}

#[derive(Resource, Asset, Reflect, Clone)]
pub struct CreditsMusic {
    #[dependency]
    music: Handle<AudioSource>,
    entity: Option<Entity>,
}

impl FromWorld for CreditsMusic {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            music: assets.load("audio/bgm/Monkeys Spinning Monkeys.ogg"),
            entity: None,
        }
    }
}

fn show_credits_screen(mut commands: Commands, mut music: ResMut<CreditsMusic>) {
    commands
        .ui_root()
        .insert(StateScoped(Screen::Credits))
        .with_children(|children| {
            children.header("Made by");
            children.label("Joe Shmoe - Implemented aligator wrestling AI");
            children.label("Jane Doe - Made the music for the alien invasion");

            children.header("Assets");
            children.label("Bevy logo - All rights reserved by the Bevy Foundation. Permission granted for splash screen use when unmodified.");
            children.label("Ducky sprite - CC0 by Caz Creates Games");
            children.label("Button SFX - CC0 by Jaszunio15");
            children.label("Music - CC BY 3.0 by Kevin MacLeod");

            children.button("Back").observe(enter_title);
        });

    music.entity = Some(
        commands
            .spawn((
                AudioBundle {
                    source: music.music.clone(),
                    settings: PlaybackSettings::LOOP,
                },
                BackgroundMusic,
            ))
            .id(),
    );
}

fn stop_bgm(mut commands: Commands, mut music: ResMut<CreditsMusic>) {
    if let Some(entity) = music.entity.take() {
        commands.entity(entity).despawn();
    }
}

fn enter_title(_trigger: Trigger<OnPress>, mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Title);
}
