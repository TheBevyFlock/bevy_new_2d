//! Game mechanics and content.

use bevy::prelude::*;
pub(crate) use spawn_level::SpawnLevel;

mod spawn_level;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(spawn_level::plugin);
}
