use bevy::prelude::*;

use super::CoreState;

pub fn plugin(app: &mut App) {
    // Setup, update, teardown
    app.add_systems(OnEnter(CoreState::Game), setup);
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SpriteBundle {
        texture: asset_server.load("ducky.png"),
        ..Default::default()
    });
}
