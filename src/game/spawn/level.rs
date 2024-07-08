//! Spawn the main level by triggering other observers.

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use super::player::SpawnPlayer;

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_level);
    app.register_type::<SpawnLevel>();
}

#[derive(Event, Debug, Hash, PartialEq, Eq, Clone, Default, Reflect, Serialize, Deserialize)]
#[reflect(Debug, Hash, PartialEq, Default, Serialize, Deserialize)]
pub(crate) struct SpawnLevel;

fn spawn_level(_trigger: Trigger<SpawnLevel>, mut commands: Commands) {
    // The only thing we have in our level is a player,
    // but add things like walls etc. here.
    commands.trigger(SpawnPlayer);
}
