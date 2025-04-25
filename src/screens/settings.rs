//! The settings screen accessible from the title screen.
//! We can add all manner of settings and accessibility options here.
//! For 3D, we'd also place the camera sensitivity and FOV here.

use std::borrow::Cow;

use bevy::{ecs::spawn::SpawnIter, prelude::*, ui::Val::*};

use crate::{screens::Screen, theme::prelude::*};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Settings), spawn_settings_screen);
}

fn spawn_settings_screen(mut commands: Commands) {
    commands.spawn((
        widget::ui_root("Settings Screen"),
        StateScoped(Screen::Settings),
        children![
            widget::header("Settings"),
            settings("Music Volume"),
            widget::button("Back", enter_title_screen),
        ],
    ));
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

struct PercentageSettings {
    name: String,
    value: f32,
}

fn settings(name: impl Into<String>) -> impl Bundle {
    (
        Name::new("Settings"),
        Node {
            display: Display::Grid,
            row_gap: Px(10.0),
            column_gap: Px(30.0),
            grid_template_columns: RepeatedGridTrack::px(2, 400.0),
            ..default()
        },
        children![
            (
                widget::label(format!("{}:", name.into())),
                Node {
                    justify_self: JustifySelf::End,
                    ..default()
                }
            ),
            (
                Node {
                    justify_self: JustifySelf::Start,
                    ..default()
                },
                children![
                    widget::button_small("-", enter_title_screen),
                    (
                        Node {
                            padding: UiRect::horizontal(Val::Px(10.0)),
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                        children![widget::label("0")],
                    ),
                    widget::button_small("+", enter_title_screen),
                ],
            ),
        ],
    )
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
