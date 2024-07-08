use bevy::{ecs::system::EntityCommands, prelude::*};

use super::spawn::Spawn;

/// Trait for spawning empty entitie and triggering them.
pub(crate) trait SpawnTrigger {
    /// Spawns an empty entity and triggers it with the event.
    fn spawn_trigger<E: Event>(&mut self, event: E) -> EntityCommands;
}

impl<T: Spawn> SpawnTrigger for T {
    fn spawn_trigger<E: Event>(&mut self, event: E) -> EntityCommands {
        // Type juggling to get around lifetime downcasting.
        // We cannot recover `entity_commands` if we ever drop it.
        let mut entity_commands = self.spawn_empty();
        let entity = entity_commands.id();
        let mut commands = entity_commands.commands();
        commands.trigger_targets(event, entity);
        entity_commands
    }
}
