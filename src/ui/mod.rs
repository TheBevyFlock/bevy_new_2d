//! Collection of UI helpers.

mod mouse_hover;
mod widgets;

use bevy::{color::palettes::tailwind, prelude::*};

pub use widgets::*;

const BUTTON_NORMAL: Color = Color::Srgba(tailwind::GRAY_800);
const BUTTON_HOVER: Color = Color::Srgba(tailwind::GRAY_600);
const BUTTON_PRESSED: Color = Color::Srgba(tailwind::GRAY_400);

const BUTTON_TEXT: Color = Color::Srgba(tailwind::GRAY_100);
const LABEL_TEXT: Color = Color::Srgba(tailwind::AMBER_300);

const BACKGROUND: Color = Color::Srgba(tailwind::GRAY_900);

pub type ButtonQuery<'w, 's, 'a, T> =
    Query<'w, 's, (&'a Interaction, &'a T), (Changed<Interaction>, With<Button>)>;

pub(super) fn plugin(app: &mut App) {
    // Sub plugins
    app.add_plugins(mouse_hover::plugin);

    // Other
    app.insert_resource(ClearColor(BACKGROUND));
}
