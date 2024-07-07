mod core;
mod game;
mod screen;
mod util;

use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((core::plugin, game::plugin, screen::plugin, util::plugin));
    }
}
