//! Spawn the player.

use bevy::prelude::*;

use crate::{
    game::physics::{PhysicalTransform, Velocity},
    screen::Screen,
};

#[derive(Debug, Event)]
pub(crate) struct SpawnPlayer;

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_player);
}

fn spawn_player(
    _trigger: Trigger<SpawnPlayer>,
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
