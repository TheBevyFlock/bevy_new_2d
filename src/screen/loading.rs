//! A loading screen during which game assets are loaded.
//! This reduces stuttering, especially for audio on WASM.

use bevy::prelude::*;

use super::Screen;
use crate::{
    game::assets::{AssetKey as _, AssetMap, ImageKey, SfxKey, SoundtrackKey},
    ui::prelude::*,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Loading), enter_loading);
    app.add_systems(
        Update,
        continue_to_title.run_if(in_state(Screen::Loading).and_then(all_assets_loaded)),
    );
}

fn enter_loading(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .ui_root()
        .insert(StateScoped(Screen::Loading))
        .with_children(|children| {
            children.label("Loading...");
        });

    // Preload assets so the game runs smoothly.
    commands.insert_resource(ImageKey::load(&asset_server));
    commands.insert_resource(SfxKey::load(&asset_server));
    commands.insert_resource(SoundtrackKey::load(&asset_server));
}

fn all_assets_loaded(
    asset_server: Res<AssetServer>,
    image_map: Res<AssetMap<ImageKey>>,
    sfx_map: Res<AssetMap<SfxKey>>,
    soundtrack_map: Res<AssetMap<SoundtrackKey>>,
) -> bool {
    image_map.all_loaded(&asset_server)
        && sfx_map.all_loaded(&asset_server)
        && soundtrack_map.all_loaded(&asset_server)
}

fn continue_to_title(mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Title);
}
