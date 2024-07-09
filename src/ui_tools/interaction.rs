use bevy::prelude::*;

use crate::game::audio::Sfx;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<InteractionPalette>();
    app.add_systems(Update, apply_interaction);
}

pub type InteractionQuery<'w, 's, T> =
    Query<'w, 's, (&'static Interaction, T), Changed<Interaction>>;

/// Palette for widget interactions.
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct InteractionPalette {
    pub none: Color,
    pub hovered: Color,
    pub pressed: Color,
}

fn apply_interaction(
    mut palette_query: InteractionQuery<(&InteractionPalette, &mut BackgroundColor)>,
    mut commands: Commands,
) {
    for (interaction, (palette, mut background)) in &mut palette_query {
        *background = match interaction {
            Interaction::None => palette.none,
            Interaction::Hovered => {
                commands.trigger(Sfx::ButtonHover);
                palette.hovered
            }
            Interaction::Pressed => {
                commands.trigger(Sfx::ButtonPress);
                palette.pressed
            }
        }
        .into();
    }
}
