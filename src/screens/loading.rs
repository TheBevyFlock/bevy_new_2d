//! A loading screen during which game assets are loaded.
//! This reduces stuttering, especially for audio on WASM.

use bevy::prelude::*;

use super::Screen;
use crate::{
    demo::player::PlayerAssets,
    screens::{credits::CreditsMusic, playing::GameplayMusic},
    theme::{interaction::InteractionAssets, prelude::*},
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Loading), show_loading_screen);
    app.add_systems(
        Update,
        continue_to_title.run_if(in_state(Screen::Loading).and_then(all_assets_loaded)),
    );
}

fn show_loading_screen(mut commands: Commands) {
    commands
        .ui_root()
        .insert(StateScoped(Screen::Loading))
        .with_children(|children| {
            children.label("Loading...").insert(Style {
                justify_content: JustifyContent::Center,
                ..default()
            });
        });
}

fn all_assets_loaded(
    player_assets: Option<Res<PlayerAssets>>,
    interaction_assets: Option<Res<InteractionAssets>>,
    credits_music: Option<Res<CreditsMusic>>,
    gameplay_music: Option<Res<GameplayMusic>>,
) -> bool {
    player_assets.is_some()
        && interaction_assets.is_some()
        && credits_music.is_some()
        && gameplay_music.is_some()
}

fn continue_to_title(mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Title);
}
