mod core;
mod game;
mod screen;
mod util;

use bevy::prelude::*;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((core::plugin, game::plugin, screen::plugin, util::plugin));
    }
}
