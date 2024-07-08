//! Game mechanics and content.
//!
//! The basic movement code shipped with the template is based on the
//! corresponding [Bevy example](https://github.com/janhohenheim/bevy/blob/fixed-time-movement/examples/movement/physics_in_fixed_timestep.rs).
//! See that link for an in-depth explanation of the code and the motivation
//! behind it.

use bevy::prelude::*;
pub(crate) use level::SpawnLevel;

mod level;
mod movement;
mod physics;
mod render;

pub(super) fn plugin(app: &mut App) {
    app.configure_sets(Update, GameSystem::Movement);

    app.add_plugins((
        movement::plugin,
        physics::plugin,
        render::plugin,
        level::plugin,
    ));
}

#[derive(Debug, SystemSet, Clone, Copy, Eq, PartialEq, Hash)]
enum GameSystem {
    Movement,
}
