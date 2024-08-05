pub mod sound_effects;
pub mod soundtrack;

use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_plugins((sound_effects::plugin, soundtrack::plugin));
}
