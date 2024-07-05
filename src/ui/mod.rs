//! Collection of UI helpers.

mod mouse_hover;
mod widgets;

use bevy::prelude::*;

pub use widgets::*;

const BUTTON_NORMAL: Color = Color::srgb(0.15, 0.15, 0.15);
const BUTTON_HOVER: Color = Color::srgb(0.25, 0.25, 0.25);
const BUTTON_PRESSED: Color = Color::srgb(0.35, 0.75, 0.35);

pub type ButtonQuery<'w, 's, 'a, T> =
    Query<'w, 's, (&'a Interaction, &'a T), (Changed<Interaction>, With<Button>)>;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(mouse_hover::plugin);
}
