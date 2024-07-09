use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_state::<Booting>();
}

#[derive(States, Debug, PartialEq, Eq, Clone, Copy, Default)]
pub(crate) enum Booting {
    #[default]
    Pending,
    Done,
}
