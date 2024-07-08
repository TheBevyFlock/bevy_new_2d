use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, apply_interaction_palette);
}

pub(crate) type InteractionQuery<'w, 's, T> =
    Query<'w, 's, (&'static Interaction, &'static T), Changed<Interaction>>;

/// Palette for widget interactions.
#[derive(Component)]
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
    mut interaction_query: Query<
        (&Interaction, &InteractionPalette, &mut BackgroundColor),
        Changed<Interaction>,
    >,
) {
    for (interaction, palette, mut background) in &mut interaction_query {
        *background = match interaction {
            Interaction::None => palette.none,
            Interaction::Hovered => palette.hovered,
            Interaction::Pressed => palette.pressed,
        }
        .into();
    }
}
