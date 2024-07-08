//! Collection of UI helpers.

pub(crate) mod interaction;
pub(crate) mod palette;
mod widgets;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(interaction::plugin);
}

pub(crate) mod prelude {
    pub(crate) use super::{
        interaction::{InteractionPalette, InteractionQuery},
        palette as ui_palette,
        widgets::{RootContainers as _, Widgets as _},
    };
}
