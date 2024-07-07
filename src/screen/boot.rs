use bevy::{core::FrameCount, prelude::*};

use super::Screen;

pub(super) fn plugin(app: &mut App) {
    // Screen setup and teardown.
    app.add_systems(OnEnter(Screen::Boot), enter_boot)
        .add_systems(OnExit(Screen::Boot), exit_boot);

    app.add_systems(
        Update,
        continue_from_boot_after_delay.run_if(in_state(Screen::Boot)),
    );
}

fn enter_boot(mut window: Query<&mut Window>) {
    // This will make the window invisible during `Boot`.
    // This is useful when you want to avoid the white flash that shows up before
    // the GPU is ready to render the app. Based on: <https://github.com/bevyengine/bevy/blob/v0.14.0/examples/window/window_settings.rs#L56>
    // This workaround does not currently work on Windows: <https://github.com/bevyengine/bevy/issues/14135>
    window.single_mut().visible = cfg!(target_os = "windows");
}

fn exit_boot(mut window: Query<&mut Window>) {
    window.single_mut().visible = true;
}

/// Based on: <https://github.com/bevyengine/bevy/blob/v0.14.0/examples/window/window_settings.rs#L56>
fn continue_from_boot_after_delay(
    mut next_screen: ResMut<NextState<Screen>>,
    frames: Res<FrameCount>,
) {
    if frames.0 >= 3 {
        next_screen.set(Screen::Title);
    }
}
