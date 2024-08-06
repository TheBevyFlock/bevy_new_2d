//! Entity spawning utilities.

// Unused utilities may trigger these lints undesirably.
#![allow(dead_code)]

use bevy::ecs::system::EntityCommand;
use bevy::ecs::system::EntityCommands;
use bevy::ecs::system::RunSystemOnce as _;
use bevy::prelude::*;

/// Re-exported extension traits.
#[allow(unused_imports)]
pub mod prelude {
    pub use super::{
        AddExt as _, AddFnExt as _, SpawnExt as _, SpawnSystemExt as _, WorldSpawnExt as _,
    };
}

/// An extension trait that provides helper functions for deferred entity spawning.
pub trait SpawnExt {
    // Workaround for https://github.com/bevyengine/bevy/issues/14231#issuecomment-2216321086.
    /// Spawn an entity with an [`EntityCommand`].
    fn spawn_with<M: 'static>(&mut self, command: impl EntityCommand<M>) -> EntityCommands;

    /// Spawn an entity with a [`System`] that receives the new entity ID via [`In<Entity>`].
    fn spawn_fn<M>(
        &mut self,
        system: impl IntoSystem<Entity, (), M> + Send + 'static,
    ) -> EntityCommands;
}

impl SpawnExt for Commands<'_, '_> {
    fn spawn_with<M: 'static>(&mut self, command: impl EntityCommand<M>) -> EntityCommands {
        let mut e = self.spawn_empty();
        e.add(command);
        e
    }

    fn spawn_fn<M>(
        &mut self,
        system: impl IntoSystem<Entity, (), M> + Send + 'static,
    ) -> EntityCommands {
        let mut e = self.spawn_empty();
        e.add_fn(system);
        e
    }
}

impl SpawnExt for ChildBuilder<'_> {
    fn spawn_with<M: 'static>(&mut self, command: impl EntityCommand<M>) -> EntityCommands {
        let mut e = self.spawn_empty();
        e.add(command);
        e
    }

    fn spawn_fn<M>(
        &mut self,
        system: impl IntoSystem<Entity, (), M> + Send + 'static,
    ) -> EntityCommands {
        let mut e = self.spawn_empty();
        e.add_fn(system);
        e
    }
}

/// An extension trait that provides helper functions for immediate entity spawning.
pub trait WorldSpawnExt {
    // Workaround for https://github.com/bevyengine/bevy/issues/14231#issuecomment-2216321086.
    /// Spawn an entity with an [`EntityCommand`].
    fn spawn_with<M: 'static>(&mut self, command: impl EntityCommand<M>) -> EntityWorldMut;

    /// Spawn an entity with a [`System`] that receives the new entity ID via [`In<Entity>`].
    fn spawn_fn<M>(
        &mut self,
        system: impl IntoSystem<Entity, (), M> + Send + 'static,
    ) -> EntityWorldMut;
}

impl WorldSpawnExt for World {
    fn spawn_with<M: 'static>(&mut self, command: impl EntityCommand<M>) -> EntityWorldMut {
        let mut e = self.spawn_empty();
        e.add(command);
        e
    }

    fn spawn_fn<M>(
        &mut self,
        system: impl IntoSystem<Entity, (), M> + Send + 'static,
    ) -> EntityWorldMut {
        let mut e = self.spawn_empty();
        e.add_fn(system);
        e
    }
}

impl WorldSpawnExt for WorldChildBuilder<'_> {
    fn spawn_with<M: 'static>(&mut self, command: impl EntityCommand<M>) -> EntityWorldMut {
        let mut e = self.spawn_empty();
        e.add(command);
        e
    }

    fn spawn_fn<M>(
        &mut self,
        system: impl IntoSystem<Entity, (), M> + Send + 'static,
    ) -> EntityWorldMut {
        let mut e = self.spawn_empty();
        e.add_fn(system);
        e
    }
}

/// An extension trait that supports arbitrary [`EntityCommand`]s after `world.spawn()`.
pub trait AddExt {
    // Workaround for https://github.com/bevyengine/bevy/issues/14278.
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

/// An extension trait that enables using systems as [`EntityCommand`]s.
pub trait AddFnExt {
    /// Apply a system to the current [`Entity`].
    fn add_fn<M>(&mut self, system: impl IntoSystem<Entity, (), M> + Send + 'static) -> &mut Self;
}

impl AddFnExt for EntityCommands<'_> {
    fn add_fn<M>(&mut self, system: impl IntoSystem<Entity, (), M> + Send + 'static) -> &mut Self {
        let id = self.id();
        self.commands()
            .add(move |world: &mut World| world.run_system_once_with(id, system));
        self
    }
}

impl AddFnExt for EntityWorldMut<'_> {
    fn add_fn<M>(&mut self, system: impl IntoSystem<Entity, (), M> + Send + 'static) -> &mut Self {
        let id = self.id();
        self.world_scope(move |world| {
            world.run_system_once_with(id, system);
        });
        self
    }
}

/// An extension trait that allows entity spawning systems to be used as actual systems.
pub trait SpawnSystemExt<M> {
    /// Create a system that spawns an entity.
    fn spawn(self) -> impl Fn(Commands);
}

impl<M, T: 'static + Send + Clone + IntoSystem<Entity, (), M>> SpawnSystemExt<M> for T {
    fn spawn(self) -> impl Fn(Commands) {
        move |mut commands: Commands| {
            commands.spawn_fn(self.clone());
        }
    }
}
