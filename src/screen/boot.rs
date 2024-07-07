use bevy::prelude::*;

use super::Screen;

pub(super) fn plugin(app: &mut App) {
    // Screen setup and teardown.
    app.add_systems(OnEnter(Screen::Boot), progress_to_title);
}

fn progress_to_title(mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Title);
}
