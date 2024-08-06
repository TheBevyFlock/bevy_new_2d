//! Functionality relating to playing audio in the game.

pub mod sound_effects;
pub mod soundtrack;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((sound_effects::plugin, soundtrack::plugin));
}
