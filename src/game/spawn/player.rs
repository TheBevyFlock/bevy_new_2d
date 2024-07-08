//! Spawn the player.

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    game::physics::{PhysicalTransform, Velocity},
    screen::Screen,
};

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_player);
    app.register_type::<(Player, SpawnPlayer)>();
}

#[derive(Event, Debug, Hash, PartialEq, Eq, Clone, Default, Reflect, Serialize, Deserialize)]
#[reflect(Debug, Hash, PartialEq, Default, Serialize, Deserialize)]
pub(crate) struct SpawnPlayer;

#[derive(
    Component, Debug, Clone, Copy, PartialEq, Eq, Default, Hash, Reflect, Serialize, Deserialize,
)]
#[reflect(Component, Debug, PartialEq, Hash, Serialize, Deserialize)]
pub(crate) struct Player;

fn spawn_player(
    _trigger: Trigger<SpawnPlayer>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        Name::new("Player"),
        Player,
        SpriteBundle {
            texture: asset_server.load("ducky.png"),
            transform: Transform::from_scale(Vec3::splat(0.5)),
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
