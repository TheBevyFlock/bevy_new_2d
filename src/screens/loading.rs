//! A loading screen that displays until all game assets are loaded.
//! This reduces stuttering, especially for audio on Wasm.

use bevy::{prelude::*, utils::HashMap};

use super::Screen;
use crate::{
    assets::{ImageHandles, SoundEffectHandles, SoundtrackHandles},
    spawn::prelude::*,
    theme::prelude::*,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Loading), loading_screen.spawn());
    app.add_systems(
        Update,
        continue_to_title.run_if(in_state(Screen::Loading).and_then(all_assets_loaded)),
    );
}

fn loading_screen(In(id): In<Entity>, mut commands: Commands) {
    commands
        .entity(id)
        .add_fn(widget::ui_root)
        .insert((Name::new("Loading screen"), StateScoped(Screen::Loading)))
        .with_children(|children| {
            children.label("Loading...");
        });
}

fn all_assets_loaded(
    asset_server: Res<AssetServer>,
    image_handles: Res<ImageHandles>,
    sound_effects_handles: Res<SoundEffectHandles>,
    soundtrack_handles: Res<SoundtrackHandles>,
) -> bool {
    image_handles.all_loaded(&asset_server)
        && sound_effects_handles.all_loaded(&asset_server)
        && soundtrack_handles.all_loaded(&asset_server)
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
