//! The game's menus and transitions between them.

mod credits;
mod main;
mod pause;
mod settings;

use bevy::prelude::*;

use crate::{pause::Pause, screens::Screen};

pub(super) fn plugin(app: &mut App) {
    app.init_state::<Menu>();
    app.add_systems(
        OnEnter(Menu::None),
        unpause_game.run_if(in_state(Screen::Gameplay)),
    );

    app.add_plugins((
        credits::plugin,
        main::plugin,
        settings::plugin,
        pause::plugin,
    ));
}

#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
#[states(scoped_entities)]
pub enum Menu {
    #[default]
    None,
    Main,
    Credits,
    Settings,
    Pause,
}

fn unpause_game(mut next_pause: ResMut<NextState<Pause>>) {
    next_pause.set(Pause(false));
}
