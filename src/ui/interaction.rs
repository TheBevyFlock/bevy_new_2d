use bevy::{ecs::system::SystemId, prelude::*};

use crate::game::{assets::SfxKey, audio::sfx::PlaySfx};

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
}

/// Palette for widget interactions. Add this to an entity that supports [`Interaction`]s, such as a button,
/// to change its [`BackgroundColor`] based on the current interaction state.
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct InteractionPalette {
    pub none: Color,
    pub hovered: Color,
    pub pressed: Color,
}

/// Component that calls a [one-shot system](https://bevyengine.org/news/bevy-0-12/#one-shot-systems)
/// when the [`Interaction`] component on the same entity changes to [`Interaction::Pressed`].
/// Use this in conjuction with [`Commands::register_one_shot_system`] to create a callback for e.g. a button press.
#[derive(Component, Debug, Reflect, Deref, DerefMut)]
#[reflect(Component, from_reflect = false)]
pub struct OnPress(#[reflect(ignore)] pub SystemId);

fn apply_on_press(
    interactions: Query<(&Interaction, &OnPress), Changed<Interaction>>,
    mut commands: Commands,
) {
    for (interaction, &OnPress(system_id)) in &interactions {
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
    interactions: Query<&Interaction, Changed<Interaction>>,
    mut commands: Commands,
) {
    for interaction in &interactions {
        match interaction {
            Interaction::Hovered => commands.trigger(PlaySfx::Key(SfxKey::ButtonHover)),
            Interaction::Pressed => commands.trigger(PlaySfx::Key(SfxKey::ButtonPress)),
            _ => (),
        }
    }
}
