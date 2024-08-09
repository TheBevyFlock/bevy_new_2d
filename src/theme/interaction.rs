use bevy::{ecs::system::SystemId, prelude::*};

use crate::{assets::SfxHandles, audio::sfx::SfxCommands as _};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<InteractionPalette>();
    app.register_type::<OnPress>();
    app.add_systems(
        Update,
        (
            apply_on_press,
            apply_interaction_palette,
            trigger_interaction_sfx,
        ),
    );
    app.observe(despawn_one_shot_system);
}

/// Palette for widget interactions. Add this to an entity that supports
/// [`Interaction`]s, such as a button, to change its [`BackgroundColor`] based
/// on the current interaction state.
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct InteractionPalette {
    pub none: Color,
    pub hovered: Color,
    pub pressed: Color,
}

/// Component that calls a [one-shot system](https://bevyengine.org/news/bevy-0-12/#one-shot-systems)
/// when the [`Interaction`] component on the same entity changes to
/// [`Interaction::Pressed`]. Use this in conjuction with
/// [`Commands::register_one_shot_system`] to create a callback for e.g. a
/// button press.
#[derive(Component, Debug, Reflect, Deref, DerefMut)]
#[reflect(Component, from_reflect = false)]
// The reflect attributes are currently needed due to
// [`SystemId` not implementing `Reflect`](https://github.com/bevyengine/bevy/issues/14496)
pub struct OnPress(#[reflect(ignore)] pub SystemId);

fn apply_on_press(
    interaction_query: Query<(&Interaction, &OnPress), Changed<Interaction>>,
    mut commands: Commands,
) {
    for (interaction, &OnPress(system_id)) in &interaction_query {
        if matches!(interaction, Interaction::Pressed) {
            commands.run_system(system_id);
        }
    }
}

fn apply_interaction_palette(
    mut palette_query: Query<
        (&Interaction, &InteractionPalette, &mut BackgroundColor),
        Changed<Interaction>,
    >,
) {
    for (interaction, palette, mut background) in &mut palette_query {
        *background = match interaction {
            Interaction::None => palette.none,
            Interaction::Hovered => palette.hovered,
            Interaction::Pressed => palette.pressed,
        }
        .into();
    }
}

fn trigger_interaction_sfx(
    interaction_query: Query<&Interaction, Changed<Interaction>>,
    mut commands: Commands,
) {
    for interaction in &interaction_query {
        match interaction {
            Interaction::Hovered => commands.play_sound_effect(SfxHandles::PATH_BUTTON_HOVER),
            Interaction::Pressed => commands.play_sound_effect(SfxHandles::PATH_BUTTON_PRESS),
            _ => (),
        }
    }
}

/// Remove the one-shot system entity when the [`OnPress`] component is removed.
/// This is necessary as otherwise, the system would still exist after the button
/// is removed, causing a memory leak.
fn despawn_one_shot_system(
    trigger: Trigger<OnRemove, OnPress>,
    mut commands: Commands,
    on_press_query: Query<&OnPress>,
) {
    let on_press = on_press_query.get(trigger.entity()).unwrap();
    let one_shot_system_entity = on_press.entity();
    commands.entity(one_shot_system_entity).despawn_recursive();
}
