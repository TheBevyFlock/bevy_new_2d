use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, apply_interaction_palette);
}

pub(crate) type ButtonInteractionQuery<'w, 's, 'a, T> =
    Query<'w, 's, (&'a Interaction, &'a T), (Changed<Interaction>, With<Button>)>;

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
        (&mut BackgroundColor, &Interaction, &InteractionPalette),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (mut background, interaction, palette) in &mut interaction_query {
        *background = match interaction {
            Interaction::None => palette.none,
            Interaction::Hovered => palette.hovered,
            Interaction::Pressed => palette.pressed,
        }
        .into();
    }
}
