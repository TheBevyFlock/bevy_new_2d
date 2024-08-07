//! Entity spawning utilities.

// Unused utilities may trigger these lints undesirably.
#![allow(dead_code)]

use bevy::ecs::system::EntityCommand;
use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;

/// Re-exported extension traits.
#[allow(unused_imports)]
pub mod prelude {
    pub use super::{AddExt as _, SpawnExt as _, WorldSpawnExt as _};
}

/// An extension trait to support spawning an entity from an [`EntityCommand`].
pub trait SpawnExt {
    // Workaround for https://github.com/bevyengine/bevy/issues/14231#issuecomment-2216321086.
    /// Spawn an entity with an [`EntityCommand`].
    fn spawn_with<M: 'static>(&mut self, command: impl EntityCommand<M>) -> EntityCommands;
}

impl SpawnExt for Commands<'_, '_> {
    fn spawn_with<M: 'static>(&mut self, command: impl EntityCommand<M>) -> EntityCommands {
        let mut e = self.spawn_empty();
        e.add(command);
        e
    }
}

impl SpawnExt for ChildBuilder<'_> {
    fn spawn_with<M: 'static>(&mut self, command: impl EntityCommand<M>) -> EntityCommands {
        let mut e = self.spawn_empty();
        e.add(command);
        e
    }
}

/// An extension trait to support immediately spawning an entity from an [`EntityCommand`].
pub trait WorldSpawnExt {
    // Workaround for <https://github.com/bevyengine/bevy/issues/14231#issuecomment-2216321086>.
    /// Immediately spawn an entity with an [`EntityCommand`].
    fn spawn_with<M: 'static>(&mut self, command: impl EntityCommand<M>) -> EntityWorldMut;
}

impl WorldSpawnExt for World {
    fn spawn_with<M: 'static>(&mut self, command: impl EntityCommand<M>) -> EntityWorldMut {
        let mut e = self.spawn_empty();
        e.add(command);
        e
    }
}

impl WorldSpawnExt for WorldChildBuilder<'_> {
    fn spawn_with<M: 'static>(&mut self, command: impl EntityCommand<M>) -> EntityWorldMut {
        let mut e = self.spawn_empty();
        e.add(command);
        e
    }
}

/// An extension trait to support adding [`EntityCommand`]s after `world.spawn()`.
pub trait AddExt {
    // Workaround for <https://github.com/bevyengine/bevy/issues/14278>.
    /// Apply an [`EntityCommand`] to the current entity.
    fn add<M: 'static>(&mut self, command: impl EntityCommand<M>) -> &mut Self;
}

impl AddExt for EntityWorldMut<'_> {
    fn add<M: 'static>(&mut self, command: impl EntityCommand<M>) -> &mut Self {
        let id = self.id();
        self.world_scope(|world| {
            world.commands().add(command.with_entity(id));
            world.flush_commands();
        });
        self
    }
}
