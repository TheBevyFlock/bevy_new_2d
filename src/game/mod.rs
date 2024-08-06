//! Game mechanics and content.

use bevy::prelude::*;

mod animation;
mod movement;
pub mod spawn;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((animation::plugin, movement::plugin, spawn::plugin));
}
