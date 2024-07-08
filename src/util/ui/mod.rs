//! Collection of UI helpers.

mod mouse_hover;
mod palette;
mod widgets;

use bevy::{color::palettes::tailwind, prelude::*};

pub use palette::*;
pub use widgets::*;

pub(crate) type ButtonInteractionQuery<'w, 's, 'a, T> =
    Query<'w, 's, (&'a Interaction, &'a T), (Changed<Interaction>, With<Button>)>;

pub(super) fn plugin(app: &mut App) {
    // Sub plugins
    app.add_plugins(mouse_hover::plugin);

    // Other
    app.insert_resource(ClearColor(BACKGROUND));
}
