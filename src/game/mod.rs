//! Game mechanics and content.

use bevy::prelude::*;
pub(crate) use spawn_level::SpawnLevel;

mod movement;
mod spawn_level;

pub(super) fn plugin(app: &mut App) {
    app.configure_sets(Update, GameSystem::Movement);

    app.add_plugins((spawn_level::plugin, movement::plugin));
}

#[derive(Debug, SystemSet, Clone, Copy, Eq, PartialEq, Hash)]
enum GameSystem {
    Movement,
}
