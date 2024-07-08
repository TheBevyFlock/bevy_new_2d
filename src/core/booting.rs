use bevy::prelude::*;
use serde::{Deserialize, Serialize};

pub(super) fn plugin(app: &mut App) {
    app.init_state::<Booting>();
    app.register_type::<Booting>();
}

#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default, Reflect, Serialize, Deserialize)]
#[reflect(Debug, Hash, PartialEq, Default, Serialize, Deserialize)]
pub(crate) enum Booting {
    #[default]
    Pending,
    Done,
}
