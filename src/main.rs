// Disable console on Windows for non-dev builds.
#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]

use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;
use bevy_template::GamePlugin;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    // Wasm builds will check for meta files (that don't exist) if this isn't set.
                    // This causes errors and even panics on web build on itch.
                    // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Window {
                        title: "bevy-template".to_string(),
                        canvas: Some("#bevy".to_string()),
                        fit_canvas_to_parent: true,
                        prevent_default_event_handling: true,
                        // This will spawn an invisible window.
                        // The window will be made visible after a few frames.
                        // This is useful when you want to avoid the white window that shows up before the GPU is ready to render the app.
                        // Based on: <https://github.com/bevyengine/bevy/blob/v0.14.0/examples/window/window_settings.rs#L56>
                        // Not invisible on Windows because of https://github.com/bevyengine/bevy/issues/14135
                        visible: cfg!(target_os = "windows"),
                        ..default()
                    }
                    .into(),
                    ..default()
                }),
        )
        // We separate Bevy configuration from our game configuration.
        .add_plugins(GamePlugin)
        .run();
}
