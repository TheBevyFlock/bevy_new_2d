use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
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
