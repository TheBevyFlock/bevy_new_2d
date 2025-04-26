//! The settings screen accessible from the title screen.
//! We can add all manner of settings and accessibility options here.
//! For 3D, we'd also place the camera sensitivity and FOV here.

use std::marker::PhantomData;

use bevy::{audio::Volume, ecs::system::IntoObserverSystem, prelude::*, ui::Val::*};

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
    let music_settings = PercentageSettings {
        name: "Audio Volume".into(),
        value_marker: GlobalVolumeLabel,
        on_minus: lower_volume,
        on_plus: raise_volume,
        _marker: PhantomData,
    };
    commands.spawn((
        widget::ui_root("Settings Screen"),
        StateScoped(Screen::Settings),
        children![
            widget::header("Settings"),
            settings(music_settings),
            widget::button("Back", enter_title_screen),
        ],
    ));
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
                        children![(widget::label(""), config.value_marker)],
                    ),
                    widget::button_small("+", config.on_plus),
                ],
            ),
        ],
    )
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
