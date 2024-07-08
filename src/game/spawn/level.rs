//! Spawn the main level by triggering other observers.

use bevy::prelude::*;

use crate::util::spawn_trigger::SpawnTrigger;

use super::{player::SpawnPlayer, SpawnTriggerHelper};

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_trigger_level);
}

#[derive(Debug, Event)]
pub(crate) struct SpawnLevel;

fn spawn_trigger_level(trigger: Trigger<SpawnLevel>, mut commands: Commands) {
    // Creating an entity for level allows us to easily despawn all of it's content later.
    commands
        .get_trigger_entity_or_spawn(trigger)
        .insert(Name::new("Level"))
        .with_children(|children| {
            children.spawn_trigger(SpawnPlayer);
        });
}
