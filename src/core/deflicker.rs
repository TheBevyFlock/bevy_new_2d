//! Based on: <https://github.com/bevyengine/bevy/blob/v0.14.0/examples/window/window_settings.rs#L56>
//! This is useful when you want to avoid the white flash that shows up before
//! the GPU is ready to render the app.

use bevy::{core::FrameCount, prelude::*};

use super::booting::Booting;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Booting::Pending), hide_window);
    app.add_systems(Update, finish_deflicker.run_if(in_state(Booting::Pending)));
    app.add_systems(OnExit(Booting::Pending), show_window);
}

/// This will make the window invisible during `Boot`.
/// This workaround does not currently work on Windows: <https://github.com/bevyengine/bevy/issues/14135>
fn hide_window(mut window: Query<&mut Window>) {
    window.single_mut().visible = cfg!(target_os = "windows");
}

fn finish_deflicker(frames: Res<FrameCount>, mut next_deflicker: ResMut<NextState<Booting>>) {
    if frames.0 >= 3 {
        // TODO: when adding asset loading, make sure both are finished
        next_deflicker.set(Booting::Done)
    }
}

fn show_window(mut window: Query<&mut Window>) {
    window.single_mut().visible = true;
}
