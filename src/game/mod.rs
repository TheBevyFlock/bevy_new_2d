//! Game mechanics and content.

use bevy::prelude::*;

mod animation;
pub mod audio;
mod movement;
pub mod spawn;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        audio::plugin,
        movement::plugin,
        animation::plugin,
        spawn::plugin,
    ));
}
