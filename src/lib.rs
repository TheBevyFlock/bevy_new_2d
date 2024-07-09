mod asset;
mod camera;
#[cfg(feature = "dev")]
mod dev;
mod game;
mod screen;
mod util;
mod window;

use bevy::prelude::*;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        // Add Bevy plugins.
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

        // Add other plugins.
        app.add_plugins((game::plugin, screen::plugin, util::plugin, camera::plugin));

        // Enable dev tools for dev builds.
        #[cfg(feature = "dev")]
        app.add_plugins(dev::plugin);
    }
}
