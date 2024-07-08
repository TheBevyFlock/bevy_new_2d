//! The game's main screen states and transitions between them.

mod credits;
mod deflicker;
mod playing;
mod splash;
mod title;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_state::<Screen>()
        .init_state::<Booting>()
        .enable_state_scoped_entities::<Screen>()
        .add_plugins((
            deflicker::plugin,
            splash::plugin,
            title::plugin,
            credits::plugin,
            playing::plugin,
        ));
}

/// The game's screen states. This is a sub-state of [`Booting::Done`].
#[derive(SubStates, Debug, Hash, PartialEq, Eq, Clone, Default)]
#[source(Booting = Booting::Done)]
pub enum Screen {
    #[default]
    Splash,
    Title,
    Credits,
    Playing,
}

#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
pub enum Booting {
    #[default]
    Pending,
    Done,
}
