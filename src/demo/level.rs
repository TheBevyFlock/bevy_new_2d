//! Spawn the main level by triggering other observers.

use bevy::prelude::*;

use super::player::SpawnPlayerCommands as _;

pub(super) fn plugin(_app: &mut App) {
    // No setup required for this plugin.
    // It's still good to have a function here so that we can add some setup
    // later if needed.
}

pub trait SpawnLevelCommands {
    fn spawn_level(&mut self);
}

impl SpawnLevelCommands for Commands<'_, '_> {
    fn spawn_level(&mut self) {
        // The only thing we have in our level is a player,
        // but add things like walls etc. here.
        self.spawn_player();
    }
}
