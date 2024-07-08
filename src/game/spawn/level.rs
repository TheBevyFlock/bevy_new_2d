//! Spawn our main level by triggering other spawn events.

use bevy::prelude::*;

use super::player::SpawnPlayer;

#[derive(Debug, Event)]
pub(crate) struct SpawnLevel;

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_level);
}

fn spawn_level(_trigger: Trigger<SpawnLevel>, mut commands: Commands) {
    // The only thing we have in our level is a player,
    // but add things like walls etc. here.
    commands.trigger(SpawnPlayer);
}
