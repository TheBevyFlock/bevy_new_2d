//! Self-contained, re-usable utilities that are not specific to this game.

#![allow(dead_code)]
#![allow(unused_imports)]

pub mod ui;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(ui::plugin);
}
