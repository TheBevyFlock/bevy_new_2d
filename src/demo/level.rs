use bevy::prelude::*;

use super::player::player;
use crate::spawn::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Level>();
}

/// A marker component for the level entity.
#[derive(Component, Reflect, Clone, Copy, PartialEq, Eq, Debug)]
#[reflect(Component)]
pub struct Level;

/// Spawn a level entity.
pub fn level(In(id): In<Entity>, mut commands: Commands) {
    commands
        .entity(id)
        .insert((Name::new("Level"), Level, SpatialBundle::default()))
        .with_children(|children| {
            children.spawn_fn(player);
        });
}
