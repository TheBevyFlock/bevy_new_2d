pub mod sfx;
pub mod soundtrack;

use bevy::prelude::*;
use sfx::Sfx;

pub fn plugin(app: &mut App) {
    app.add_systems(Update, button_interaction_sfx);

    app.observe(soundtrack::play_soundtrack);
    app.observe(sfx::play_sfx);
}

fn button_interaction_sfx(
    mut interactions: Query<&'static Interaction, Changed<Interaction>>,
    mut commands: Commands,
) {
    for interaction in &mut interactions {
        match interaction {
            Interaction::Hovered => commands.trigger(Sfx::ButtonHover),
            Interaction::Pressed => commands.trigger(Sfx::ButtonPress),
            _ => {}
        }
    }
}
