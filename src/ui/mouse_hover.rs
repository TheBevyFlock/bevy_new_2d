use bevy::prelude::*;

use super::{BUTTON_HOVER, BUTTON_NORMAL, BUTTON_PRESSED};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, update);
}

fn update(
    mut interaction_query: Query<
        (&mut BackgroundColor, &Interaction),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (mut background, interaction) in &mut interaction_query {
        *background = match interaction {
            Interaction::Pressed => BUTTON_PRESSED,
            Interaction::Hovered => BUTTON_HOVER,
            Interaction::None => BUTTON_NORMAL,
        }
        .into();
    }
}
