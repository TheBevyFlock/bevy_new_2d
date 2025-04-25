//! The settings screen accessible from the title screen.
//! We can add all manner of settings and accessibility options here.
//! For 3D, we'd also place the camera sensitivity and FOV here.

use std::{borrow::Cow, marker::PhantomData};

use bevy::{
    ecs::{spawn::SpawnIter, system::IntoObserverSystem},
    prelude::*,
    ui::Val::*,
};

use crate::{screens::Screen, theme::prelude::*};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Settings), spawn_settings_screen);
}

#[derive(Component)]
struct Volume;

fn spawn_settings_screen(mut commands: Commands) {
    let volume_settings = PercentageSettings {
        name: "Music Volume".into(),
        value_marker: Volume,
        on_minus: lower_volume,
        on_plus: raise_volume,
        _marker: PhantomData,
    };
    commands.spawn((
        widget::ui_root("Settings Screen"),
        StateScoped(Screen::Settings),
        children![
            widget::header("Settings"),
            settings(volume_settings),
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

struct PercentageSettings<E, B1, B2, M1, M2, I1, I2, C>
where
    E: Event,
    B1: Bundle,
    B2: Bundle,
    I1: IntoObserverSystem<E, B1, M1> + Sync,
    I2: IntoObserverSystem<E, B2, M2> + Sync,
    C: Component,
{
    name: String,
    value_marker: C,
    on_minus: I1,
    on_plus: I2,
    _marker: PhantomData<(E, B1, B2, M1, M2)>,
}

fn settings<E, B1, B2, M1, M2, I1, I2, C>(
    config: PercentageSettings<E, B1, B2, M1, M2, I1, I2, C>,
) -> impl Bundle
where
    E: Event,
    B1: Bundle,
    B2: Bundle,
    I1: IntoObserverSystem<E, B1, M1> + Sync,
    I2: IntoObserverSystem<E, B2, M2> + Sync,
    C: Component,
{
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
                widget::label(format!("{}:", config.name)),
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
                    widget::button_small("-", config.on_minus),
                    (
                        Node {
                            padding: UiRect::horizontal(Val::Px(10.0)),
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                        children![(widget::label("0"), config.value_marker)],
                    ),
                    widget::button_small("+", config.on_plus),
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

fn lower_volume(_: Trigger<Pointer<Click>>) {
    println!("Lower volume");
}

fn raise_volume(_: Trigger<Pointer<Click>>) {
    println!("Raise volume");
}

fn get_volume() -> f32 {
    0.0
}
