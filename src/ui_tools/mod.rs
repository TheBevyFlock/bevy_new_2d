//! Collection of UI helpers.

pub mod interaction;
pub mod palette;

#[allow(unused_imports)]
pub mod prelude {
    pub use super::{
        interaction::{InteractionPalette, InteractionQuery},
        palette as ui_palette,
        widgets::{RootContainers as _, Widgets as _},
    };
}

mod widgets;

use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_plugins(interaction::plugin);
}
