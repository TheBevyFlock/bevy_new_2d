//! A loading screen during which game assets are loaded.
//! This reduces stuttering, especially for audio on WASM.

use bevy::prelude::*;

use super::Screen;
use crate::{
    game::assets::{HandleMap, ImageKey, SfxKey, SoundtrackKey},
    ui::prelude::*,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Loading), enter_loading);
    app.add_systems(
        Update,
        continue_to_title.run_if(in_state(Screen::Loading).and_then(all_assets_loaded)),
    );
}

fn enter_loading(mut commands: Commands) {
    commands
        .ui_root()
        .insert(StateScoped(Screen::Loading))
        .with_children(|children| {
            children.label("Loading...");
        });
}

fn all_assets_loaded(
    asset_server: Res<AssetServer>,
    image_handles: Res<HandleMap<ImageKey>>,
    sfx_handles: Res<HandleMap<SfxKey>>,
    soundtrack_handles: Res<HandleMap<SoundtrackKey>>,
) -> bool {
    image_handles.all_loaded(&asset_server)
        && sfx_handles.all_loaded(&asset_server)
        && soundtrack_handles.all_loaded(&asset_server)
}

fn continue_to_title(mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Title);
}
