use bevy::prelude::*;
use serde::{Deserialize, Serialize};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, apply_interaction_palette);
    app.register_type::<InteractionPalette>();
}

pub(crate) type InteractionQuery<'w, 's, T> =
    Query<'w, 's, (&'static Interaction, T), Changed<Interaction>>;

/// Palette for widget interactions.
#[derive(Component, Debug, Clone, Copy, PartialEq, Reflect, Serialize, Deserialize)]
#[reflect(Component, Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct InteractionPalette {
    none: Color,
    hovered: Color,
    pressed: Color,
}

impl InteractionPalette {
    pub fn new(none: Color, hovered: Color, pressed: Color) -> Self {
        Self {
            none,
            hovered,
            pressed,
        }
    }
}

fn apply_interaction_palette(
    mut palette_query: InteractionQuery<(&InteractionPalette, &mut BackgroundColor)>,
) {
    for (interaction, (palette, mut background)) in &mut palette_query {
        *background = match interaction {
            Interaction::None => palette.none,
            Interaction::Hovered => palette.hovered,
            Interaction::Pressed => palette.pressed,
        }
        .into();
    }
}
