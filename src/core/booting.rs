use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_state::<BootingState>();
}

#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
pub enum BootingState {
    #[default]
    Booting,
    Ready,
}
