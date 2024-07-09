//! Self-contained, re-usable utilities that are not specific to this game.

// Unused utilities and re-exports may trigger these lints undesirably.
#![allow(dead_code, unused_imports)]

pub mod interaction;
pub mod palette;
mod widgets;

pub mod prelude {
    pub use super::{
        interaction::{InteractionPalette, InteractionQuery},
        palette as ui_palette,
        widgets::{RootContainers as _, Widgets as _},
    };
}

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(interaction::plugin);
}
