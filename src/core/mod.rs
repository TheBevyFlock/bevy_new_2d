#[cfg(feature = "dev")]
mod dev;
mod loading;
mod ready;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((loading::plugin, ready::plugin));

    #[cfg(feature = "dev")]
    app.add_plugins(dev::plugin);
}
