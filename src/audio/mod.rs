//! Functionality relating to playing audio in the game.

pub mod sfx;
pub mod bgm;

use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_plugins((sfx::plugin, bgm::plugin));
}
