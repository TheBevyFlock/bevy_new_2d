//! The states describing the game's main screen states and the transitions between them.

mod credits;
mod playing;
mod title;

use bevy::prelude::*;

use crate::core::booting::Booting;

pub(super) fn plugin(app: &mut App) {
    app.init_state::<Screen>()
        .enable_state_scoped_entities::<Screen>()
        .add_plugins((title::plugin, credits::plugin, playing::plugin));
}

/// The game's screen states. This is a sub-state of [`Booting::Done`].
#[derive(SubStates, Debug, Hash, PartialEq, Eq, Clone, Default)]
#[source(Booting = Booting::Done)]
pub(crate) enum Screen {
    #[default]
    Title,
    Credits,
    Playing,
}
