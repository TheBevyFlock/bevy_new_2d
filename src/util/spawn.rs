use bevy::{ecs::system::EntityCommands, prelude::*};

/// Things that can spawn entities.
pub(crate) trait Spawn {
    /// Spawn empty entity.
    fn spawn_empty(&mut self) -> EntityCommands;

    /// Spawn entity with bundle.
    fn spawn<B: Bundle>(&mut self, bundle: B) -> EntityCommands;
}

impl<'a, 'b> Spawn for Commands<'a, 'b> {
    fn spawn_empty(&mut self) -> EntityCommands {
        self.spawn_empty()
    }

    fn spawn<B: Bundle>(&mut self, bundle: B) -> EntityCommands {
        self.spawn(bundle)
    }
}

impl<'a> Spawn for ChildBuilder<'a> {
    fn spawn_empty(&mut self) -> EntityCommands {
        self.spawn_empty()
    }

    fn spawn<B: Bundle>(&mut self, bundle: B) -> EntityCommands {
        self.spawn(bundle)
    }
}
