mod boot;
mod credits;
mod playing;
mod title;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_state::<Screen>()
        .enable_state_scoped_entities::<Screen>();

    app.add_plugins((
        boot::plugin,
        title::plugin,
        credits::plugin,
        playing::plugin,
    ));
}

#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
pub enum Screen {
    #[default]
    Boot,
    Title,
    Credits,
    Playing,
}
