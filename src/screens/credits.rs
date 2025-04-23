//! A credits screen that can be accessed from the title screen.

use bevy::{ecs::spawn::SpawnIter, prelude::*, ui::Val::*};

use crate::{asset_tracking::LoadResource, audio::Music, screens::Screen, theme::prelude::*};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Credits), spawn_credits_screen);

    app.register_type::<CreditsMusic>();
    app.load_resource::<CreditsMusic>();
    app.add_systems(OnEnter(Screen::Credits), start_credits_music);
    app.add_systems(OnExit(Screen::Credits), stop_credits_music);
}

fn spawn_credits_screen(mut commands: Commands) {
    commands
        .spawn((
            widget::ui_root("Credits Screen"),
            StateScoped(Screen::Credits),
            children![
                widget::header("Created by"),
                created_by(),
                widget::header("Assets"),
                assets(),
            ],
        ))
        .with_children(|parent| {
            parent
                .spawn(widget::button("Back"))
                .observe(enter_title_screen);
        });
}

fn created_by() -> impl Bundle {
    grid(vec![
        ["Joe Shmoe", "Implemented alligator wrestling AI"],
        ["Jane Doe", "Made the music for the alien invasion"],
    ])
}

fn assets() -> impl Bundle {
    grid(vec![
        ["Ducky sprite", "CC0 by Caz Creates Games"],
        ["Button SFX", "CC0 by Jaszunio15"],
        ["Music", "CC BY 3.0 by Kevin MacLeod"],
        [
            "Bevy logo",
            "All rights reserved by the Bevy Foundation, permission granted for splash screen use when unmodified",
        ],
    ])
}

fn grid(content: Vec<[&'static str; 2]>) -> impl Bundle {
    (
        Name::new("Grid"),
        Node {
            display: Display::Grid,
            row_gap: Px(10.0),
            column_gap: Px(30.0),
            grid_template_columns: RepeatedGridTrack::px(2, 400.0),
            ..default()
        },
        Children::spawn(SpawnIter(content.into_iter().flatten().enumerate().map(
            |(i, text)| {
                (
                    widget::label(text),
                    Node {
                        justify_self: if i % 2 == 0 {
                            JustifySelf::End
                        } else {
                            JustifySelf::Start
                        },
                        ..default()
                    },
                )
            },
        ))),
    )
}

fn enter_title_screen(_: Trigger<Pointer<Click>>, mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Title);
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
struct CreditsMusic {
    #[dependency]
    music: Handle<AudioSource>,
    entity: Option<Entity>,
}

impl FromWorld for CreditsMusic {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            music: assets.load("audio/music/Monkeys Spinning Monkeys.ogg"),
            entity: None,
        }
    }
}

fn start_credits_music(mut commands: Commands, mut music: ResMut<CreditsMusic>) {
    music.entity = Some(
        commands
            .spawn((
                AudioPlayer(music.music.clone()),
                PlaybackSettings::LOOP,
                Music,
            ))
            .id(),
    );
}

fn stop_credits_music(mut commands: Commands, mut music: ResMut<CreditsMusic>) {
    if let Some(entity) = music.entity.take() {
        commands.entity(entity).despawn();
    }
}
