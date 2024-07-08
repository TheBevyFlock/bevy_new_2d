//! The game's main screen states and transitions between them.

mod credits;
mod playing;
mod splash;
mod title;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::core::booting::Booting;

pub(super) fn plugin(app: &mut App) {
    app.init_state::<Screen>();
    app.enable_state_scoped_entities::<Screen>();
    app.register_type::<Screen>();

    app.add_plugins((
        splash::plugin,
        title::plugin,
        credits::plugin,
        playing::plugin,
    ));
}

/// The game's screen states. This is a sub-state of [`Booting::Done`].
#[derive(
    SubStates, Debug, Hash, PartialEq, Eq, Clone, Default, Reflect, Serialize, Deserialize,
)]
#[reflect(Debug, Hash, PartialEq, Default, Serialize, Deserialize)]
#[source(Booting = Booting::Done)]
pub(crate) enum Screen {
    #[default]
    Splash,
    Title,
    Credits,
    Playing,
}
