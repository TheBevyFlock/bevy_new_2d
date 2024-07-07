use bevy::core::FrameCount;
pub use bevy::prelude::*;

use super::AppState;

pub(super) fn plugin(app: &mut App) {
    // Setup(s), update(s), teardown(s)
    app.add_systems(Update, frames_elapsed.run_if(in_state(AppState::Loading)));
}

// Based on: <https://github.com/bevyengine/bevy/blob/v0.14.0/examples/window/window_settings.rs#L56>
fn frames_elapsed(mut next_app_state: ResMut<NextState<AppState>>, frames: Res<FrameCount>) {
    if frames.0 >= 3 {
        next_app_state.set(AppState::Ready);
    }
}
