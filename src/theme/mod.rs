//! Reusable UI widgets & theming.

// Unused utilities may trigger these lints undesirably.
#![allow(dead_code)]

pub mod interaction;
pub mod palette;
pub mod widget;

#[allow(unused_imports)]
pub mod prelude {
    pub use super::{
        interaction::{InteractionPalette, OnPress},
        palette as ui_palette,
        widget::{self, Widgets as _, WorldWidgets as _},
    };
}

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(interaction::plugin);
}
