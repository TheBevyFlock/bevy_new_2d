//! A settings screen that can be accessed from the title screen.
//!
//! Settings and accessibility options should go here.

use bevy::{audio::Volume, prelude::*, ui::Val::*};

use crate::{screens::Screen, theme::prelude::*};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Settings), spawn_settings_screen);

    app.register_type::<GlobalVolumeLabel>();
    app.add_systems(
        Update,
        update_volume_label.run_if(in_state(Screen::Settings)),
    );
}

fn spawn_settings_screen(mut commands: Commands) {
    commands.spawn((
        widget::ui_root("Settings Screen"),
        StateScoped(Screen::Settings),
        children![
            widget::header("Settings"),
            settings_grid(),
            widget::button("Back", enter_title_screen),
        ],
    ));
}

fn settings_grid() -> impl Bundle {
    (
        Name::new("Settings Grid"),
        Node {
            display: Display::Grid,
            row_gap: Px(10.0),
            column_gap: Px(30.0),
            grid_template_columns: RepeatedGridTrack::px(2, 400.0),
            ..default()
        },
        children![
            (
                widget::label("Audio Volume"),
                Node {
                    justify_self: JustifySelf::End,
                    ..default()
                }
            ),
            volume_widget(),
        ],
    )
}

fn volume_widget() -> impl Bundle {
    (
        Name::new("Volume Widget"),
        Node {
            justify_self: JustifySelf::Start,
            ..default()
        },
        children![
            widget::button_small("-", lower_volume),
            (
                Name::new("Current Volume"),
                Node {
                    padding: UiRect::horizontal(Px(10.0)),
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                children![(widget::label(""), GlobalVolumeLabel)],
            ),
            widget::button_small("+", raise_volume),
        ],
    )
}

const MIN_VOLUME: f32 = 0.0;
const MAX_VOLUME: f32 = 3.0;

fn lower_volume(_: Trigger<Pointer<Click>>, mut global_volume: ResMut<GlobalVolume>) {
    let new_factor = global_volume.volume.to_linear() - 0.1;
    global_volume.volume = Volume::Linear(new_factor.max(MIN_VOLUME));
}

fn raise_volume(_: Trigger<Pointer<Click>>, mut global_volume: ResMut<GlobalVolume>) {
    let new_factor = global_volume.volume.to_linear() + 0.1;
    global_volume.volume = Volume::Linear(new_factor.min(MAX_VOLUME));
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct GlobalVolumeLabel;

fn update_volume_label(
    global_volume: Res<GlobalVolume>,
    mut label: Single<&mut Text, With<GlobalVolumeLabel>>,
) {
    let percent = 100.0 * global_volume.volume.to_linear();
    label.0 = format!("{percent:3.0}%");
}

fn enter_title_screen(_: Trigger<Pointer<Click>>, mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Title);
}
