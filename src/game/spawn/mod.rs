//! Handles spawning of well defined game entities. Here, we are using
//! [observers](https://docs.rs/bevy/latest/bevy/ecs/prelude/struct.Observer.html)
//! to separate spawn requests from spawn resource requirements, like assets.

use bevy::{ecs::system::EntityCommands, prelude::*};

pub(crate) mod level;
pub(crate) mod player;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((level::plugin, player::plugin));
}

trait SpawnTriggerHelper {
    /// Returns commands for the targeted entity if one exists.
    /// Spawns a new entity otherwise.
    fn get_trigger_entity_or_spawn<E>(&mut self, trigger: Trigger<E>) -> EntityCommands;
}

impl<'a, 'b> SpawnTriggerHelper for Commands<'a, 'b> {
    fn get_trigger_entity_or_spawn<E>(&mut self, trigger: Trigger<E>) -> EntityCommands {
        let entity = trigger.entity();
        // Cannot use `match` due to compile time trait restraints
        if entity == Entity::PLACEHOLDER {
            self.spawn_empty()
        } else {
            self.entity(entity)
        }
    }
}
