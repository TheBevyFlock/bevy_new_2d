use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_state::<Booting>();
}

#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
pub(crate) enum Booting {
    #[default]
    Pending,
    Done,
}
