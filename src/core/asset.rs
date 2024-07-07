use bevy::{asset::AssetMetaCheck, prelude::*};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(AssetPlugin {
        // Wasm builds will check for meta files (that don't exist) if this isn't set.
        // This causes errors and even panics on web build on itch.
        // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
        meta_check: AssetMetaCheck::Never,
        ..default()
    });
}
