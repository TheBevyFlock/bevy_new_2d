//! Spawn the main level.

use bevy::{ecs::world::Command, prelude::*};

use super::player::SpawnPlayer;

pub(super) fn plugin(_app: &mut App) {
    // No setup required for this plugin.
    // It's still good to have a function here so that we can add some setup
    // later if needed.
}

#[derive(Debug)]
pub struct SpawnLevel;

impl Command for SpawnLevel {
    fn apply(self, world: &mut World) {
        // The only thing we have in our level is a player,
        // but add things like walls etc. here.
        world.commands().add(SpawnPlayer::default());
    }
}
