use bevy::prelude::*;

use super::{BUTTON_HOVER, BUTTON_NORMAL, BUTTON_PRESSED};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, update);
}

type InteractiveFilter = (Changed<Interaction>, With<Button>);

fn update(mut interaction_query: Query<(&mut BackgroundColor, &Interaction), InteractiveFilter>) {
    for (mut background, interaction) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *background = BUTTON_PRESSED.into();
            }
            Interaction::Hovered => {
                *background = BUTTON_HOVER.into();
            }
            Interaction::None => {
                *background = BUTTON_NORMAL.into();
            }
        }
    }
}
