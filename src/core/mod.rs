//! Foundational features and cross-cutting concerns.

mod asset;
mod camera;
#[cfg(feature = "dev")]
mod dev;
mod window;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    // Bevy plugins.
    app.add_plugins(
        DefaultPlugins
            .build()
            .disable::<AssetPlugin>()
            .add_after::<AssetPlugin, _>(asset::plugin)
            .disable::<WindowPlugin>()
            .add_after::<WindowPlugin, _>(window::plugin),
    );

    // Other plugins.
    app.add_plugins(camera::plugin);

    // Debugging tools for dev builds.
    #[cfg(feature = "dev")]
    app.add_plugins(dev::plugin);
}
