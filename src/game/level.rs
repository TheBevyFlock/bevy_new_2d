//! Setup the level

use bevy::prelude::*;

use super::physics::{PhysicalTransform, Velocity};
use crate::screen::Screen;

#[derive(Debug, Event)]
pub(crate) struct SpawnLevel;

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_level);
}

fn spawn_level(
    _trigger: Trigger<SpawnLevel>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        Name::new("Player"),
        SpriteBundle {
            texture: asset_server.load("ducky.png"),
            transform: Transform::from_scale(Vec3::splat(0.4)),
            ..Default::default()
        },
        Velocity::default(),
        // If your physics engine of choice uses `Transform` directly,
        // a good hierarchy to follow instead is to have a `Player` root entity
        // with the physical transform and the rendered transform as individual children.
        PhysicalTransform::default(),
        StateScoped(Screen::Playing),
    ));
}
