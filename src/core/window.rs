use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_state::<WindowState>();
    app.add_plugins(WindowPlugin {
        primary_window: Window {
            title: "bevy-template".to_string(),
            canvas: Some("#bevy".to_string()),
            fit_canvas_to_parent: true,
            prevent_default_event_handling: true,
            ..default()
        }
        .into(),
        ..default()
    });
}

#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
pub enum WindowState {
    #[default]
    Booting,
    Ready,
}
