//! Foundational features and cross-cutting concerns.

mod asset;
pub(crate) mod booting;
mod camera;
mod deflicker;
#[cfg(feature = "dev")]
mod dev;
mod window;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    // Bevy plugins.
    app.add_plugins(
        DefaultPlugins
            .build()
            // Add `AssetPlugin` via `asset::plugin`.
            .disable::<AssetPlugin>()
            .add_after::<AssetPlugin, _>(asset::plugin)
            // Add `WindowPlugin` via `window::plugin`.
            .disable::<WindowPlugin>()
            .add_after::<WindowPlugin, _>(window::plugin),
    );

    // Other plugins.
    app.add_plugins((booting::plugin, deflicker::plugin, camera::plugin));

    // Debugging tools for dev builds.
    #[cfg(feature = "dev")]
    app.add_plugins(dev::plugin);
}
