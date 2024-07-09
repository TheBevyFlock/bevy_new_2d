//! Collection of UI helpers.

pub mod interaction;
pub mod palette;
mod widgets;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(interaction::plugin);
}

pub mod prelude {
    pub use super::{
        interaction::{InteractionPalette, InteractionQuery},
        palette as ui_palette,
        widgets::{RootContainers as _, Widgets as _},
    };
}
