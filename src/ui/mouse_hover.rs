use bevy::prelude::*;

use super::{BUTTON_HOVER, BUTTON_NORMAL, BUTTON_PRESSED};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, update);
}

type InteractiveFilter = (Changed<Interaction>, With<Button>);

fn update(mut interaction_query: Query<(&mut BackgroundColor, &Interaction), InteractiveFilter>) {
    for (mut background, interaction) in &mut interaction_query {
        *background = match interaction {
            Interaction::Pressed => BUTTON_PRESSED,
            Interaction::Hovered => BUTTON_HOVER,
            Interaction::None => BUTTON_NORMAL,
        }
            .into();
    }
}
