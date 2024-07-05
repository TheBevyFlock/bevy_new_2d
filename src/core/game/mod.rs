use bevy::prelude::*;

use super::CoreState;

pub(super) fn plugin(app: &mut App) {
    // Setup, update, teardown
    // For core game logic, you'll probably need multiple setups, updates, etc.
    app.add_systems(OnEnter(CoreState::Game), setup);
    app.add_systems(Update, update.run_if(in_state(CoreState::Game)));
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("ducky.png"),
            ..Default::default()
        },
        StateScoped(CoreState::Game),
    ));
}

fn update(mut next_core_state: ResMut<NextState<CoreState>>, input: Res<ButtonInput<KeyCode>>) {
    if input.just_pressed(KeyCode::Escape) {
        next_core_state.set(CoreState::Menu);
    }
}
