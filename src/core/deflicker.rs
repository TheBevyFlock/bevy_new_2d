//! Based on: <https://github.com/bevyengine/bevy/blob/v0.14.0/examples/window/window_settings.rs#L56>
//! This is useful when you want to avoid the white flash that shows up before the GPU is ready to render the app.

use bevy::{core::FrameCount, prelude::*};

pub(super) fn plugin(app: &mut App) {
    app.init_state::<Deflicker>();
    app.add_systems(OnEnter(Deflicker::Running), make_window_invisible);
    app.add_systems(
        Update,
        finish_deflicker.run_if(in_state(Deflicker::Running)),
    );
    app.add_systems(OnExit(Deflicker::Running), make_window_visible);
}

#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
enum Deflicker {
    #[default]
    Running,
    Done,
}

fn hide_window(mut window: Query<&mut Window>) {
    // This will make the window invisible during `Boot`.
    // This workaround does not currently work on Windows: <https://github.com/bevyengine/bevy/issues/14135>
    window.single_mut().visible = cfg!(target_os = "windows");
}

fn finish_deflicker(frames: Res<FrameCount>, mut next_deflicker: ResMut<NextState<Deflicker>>) {
    if frames.0 >= 3 {
        next_deflicker.set(Deflicker::Done)
    }
}

fn show_window(mut window: Query<&mut Window>) {
    window.single_mut().visible = true;
}
