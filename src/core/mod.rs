mod loading;
mod ready;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((loading::plugin, ready::plugin));
}
