//! A loading screen during which game assets are loaded.
//! This reduces stuttering, especially for audio on WASM.

use bevy::{prelude::*, utils::HashMap};

use super::Screen;
use crate::{
    assets::{BgmHandles, ImageHandles, SfxHandles},
    theme::prelude::*,
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
            children.label("Loading...");
        });
}

fn all_assets_loaded(
    asset_server: Res<AssetServer>,
    image_handles: Res<ImageHandles>,
    sfx_handles: Res<SfxHandles>,
    bgm_handles: Res<BgmHandles>,
) -> bool {
    image_handles.all_loaded(&asset_server)
        && sfx_handles.all_loaded(&asset_server)
        && bgm_handles.all_loaded(&asset_server)
}

fn continue_to_title(mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Title);
}

/// An extension trait to check if all the assets in an asset collection are
/// loaded.
trait AllLoaded {
    fn all_loaded(&self, asset_server: &AssetServer) -> bool;
}

impl<T: Asset> AllLoaded for HashMap<String, Handle<T>> {
    fn all_loaded(&self, asset_server: &AssetServer) -> bool {
        self.values()
            .all(|x| asset_server.is_loaded_with_dependencies(x))
    }
}

impl<T: Asset> AllLoaded for HashMap<String, Vec<Handle<T>>> {
    fn all_loaded(&self, asset_server: &AssetServer) -> bool {
        self.values()
            .flatten()
            .all(|x| asset_server.is_loaded_with_dependencies(x))
    }
}
