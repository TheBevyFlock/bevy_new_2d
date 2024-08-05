use bevy::prelude::*;

use crate::game::{assets::sound_effects_key, audio::sound_effects::SfxCommands as _};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<InteractionPalette>();
    app.add_systems(Update, (apply_interaction_palette, trigger_interaction_sfx));
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
    mut interactions: Query<&Interaction, Changed<Interaction>>,
    mut commands: Commands,
) {
    for interaction in &mut interactions {
        match interaction {
            Interaction::Hovered => commands.play_sound_effect(sound_effects_key::BUTTON_HOVER),
            Interaction::Pressed => commands.play_sound_effect(sound_effects_key::BUTTON_PRESS),
            _ => (),
        }
    }
}
