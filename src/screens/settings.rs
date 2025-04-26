//! The settings screen accessible from the title screen.
//! We can add all manner of settings and accessibility options here.
//! For 3D, we'd also place the camera sensitivity and FOV here.

use bevy::{audio::Volume, prelude::*, ui::Val::*};

use crate::{screens::Screen, theme::prelude::*};

const MIN_VOLUME: f32 = 0.0;
const MAX_VOLUME: f32 = 3.0;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Settings), spawn_settings_screen);
    app.add_systems(
        Update,
        update_volume_label.run_if(in_state(Screen::Settings)),
    );
}

#[derive(Component)]
struct GlobalVolumeLabel;

fn spawn_settings_screen(mut commands: Commands) {
    commands.spawn((
        widget::ui_root("Settings Screen"),
        StateScoped(Screen::Settings),
        children![
            widget::header("Settings"),
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
                    (
                        Node {
                            justify_self: JustifySelf::Start,
                            ..default()
                        },
                        children![
                            widget::button_small("-", lower_volume),
                            (
                                Node {
                                    padding: UiRect::horizontal(Px(10.0)),
                                    justify_content: JustifyContent::Center,
                                    ..default()
                                },
                                children![(widget::label(""), GlobalVolumeLabel)],
                            ),
                            widget::button_small("+", raise_volume),
                        ],
                    ),
                ],
            ),
            widget::button("Back", enter_title_screen),
        ],
    ));
}

fn lower_volume(_: Trigger<Pointer<Click>>, mut global_volume: ResMut<GlobalVolume>) {
    let new_factor = global_volume.volume.to_linear() - 0.1;
    global_volume.volume = Volume::Linear(new_factor.max(MIN_VOLUME));
}

fn raise_volume(_: Trigger<Pointer<Click>>, mut global_volume: ResMut<GlobalVolume>) {
    let new_factor = global_volume.volume.to_linear() + 0.1;
    global_volume.volume = Volume::Linear(new_factor.min(MAX_VOLUME));
}

fn update_volume_label(
    mut label: Single<&mut Text, With<GlobalVolumeLabel>>,
    global_volume: Res<GlobalVolume>,
) {
    let factor = global_volume.volume.to_linear();
    let percent = (factor * 100.0).round();
    let text = format!("{}%", percent);
    label.0 = text;
}

fn enter_title_screen(_: Trigger<Pointer<Click>>, mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Title);
}
