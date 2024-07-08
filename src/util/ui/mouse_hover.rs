use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, update);
}

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

fn update(
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
