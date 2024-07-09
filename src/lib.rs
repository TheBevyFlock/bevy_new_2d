mod camera;
#[cfg(feature = "dev")]
mod dev;
mod game;
mod screen;
mod util;

use bevy::{asset::AssetMetaCheck, prelude::*};

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        // Add Bevy plugins.
        app.add_plugins(
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
                        ..default()
                    }
                    .into(),
                    ..default()
                }),
        );

        // Add other plugins.
        app.add_plugins((game::plugin, screen::plugin, util::plugin, camera::plugin));

        // Enable dev tools for dev builds.
        #[cfg(feature = "dev")]
        app.add_plugins(dev::plugin);
    }
}
