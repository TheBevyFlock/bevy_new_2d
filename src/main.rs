// Disable console on Windows for non-dev builds.
#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]
// Support configuring Bevy lints within code.
#![cfg_attr(bevy_lint, feature(register_tool), register_tool(bevy))]

use bevy::prelude::*;
use bevy_new_2d::AppPlugin;

fn main() -> AppExit {
    App::new().add_plugins(AppPlugin).run()
}
