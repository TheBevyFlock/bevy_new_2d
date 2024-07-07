use super::RunningState;
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    // Setup(s), update(s), teardown(s)
    app.add_systems(OnEnter(RunningState::Game), setup);
    app.add_systems(Update, update.run_if(in_state(RunningState::Game)));
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("ducky.png"),
            ..Default::default()
        },
        StateScoped(RunningState::Game),
    ));
}

fn update(mut next_core_state: ResMut<NextState<RunningState>>, input: Res<ButtonInput<KeyCode>>) {
    if input.just_pressed(KeyCode::Escape) {
        next_core_state.set(RunningState::Menu);
    }
}
