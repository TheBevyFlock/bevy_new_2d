use bevy::{ecs::system::SystemId, prelude::*};

use crate::game::{assets::SfxKey, audio::sfx::PlaySfx};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<InteractionPalette>();
    app.register_type::<OnPress>();
    app.add_systems(
        Update,
        (
            apply_interaction_palette,
            apply_interaction_callback,
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

#[derive(Component, Debug, Reflect, Deref, DerefMut)]
#[reflect(Component, from_reflect = false)]
pub struct OnPress(#[reflect(ignore)] pub SystemId);

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

fn apply_interaction_callback(
    interactions: Query<(&Interaction, &OnPress), Changed<Interaction>>,
    mut commands: Commands,
) {
    for (interaction, OnPress(system_id)) in &interactions {
        if matches!(interaction, Interaction::Pressed) {
            commands.run_system(*system_id);
        }
    }
}
