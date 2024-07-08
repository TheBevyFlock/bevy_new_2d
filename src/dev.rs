//! Development tools for the game. This plugin is only enabled in dev builds.

use crate::screen::{Booting, Screen};
use bevy::{dev_tools::states::log_transitions, prelude::*};

pub struct DevPlugin;

impl Plugin for DevPlugin {
    fn build(&self, app: &mut App) {
        // Print state transitions in dev builds
        app.add_systems(
            Update,
            (log_transitions::<Booting>, log_transitions::<Screen>).chain(),
        );
    }
}
