//! This module contains the asset handles used throughout the game.
//! During `Screen::Loading`, the game will load the assets specified here.
//! Your systems can then request the resources defined here to access the
//! loaded assets.

use bevy::{
    prelude::*,
    render::texture::{ImageLoaderSettings, ImageSampler},
    utils::HashMap,
};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<ImageHandles>();
    app.init_resource::<ImageHandles>();

    app.register_type::<BgmHandles>();
    app.init_resource::<BgmHandles>();

    app.register_type::<SfxHandles>();
    app.init_resource::<SfxHandles>();
}

#[derive(Resource, Debug, Deref, DerefMut, Reflect)]
#[reflect(Resource)]
pub struct ImageHandles(HashMap<String, Handle<Image>>);

impl ImageHandles {
    pub const PATH_DUCKY: &'static str = "images/ducky.png";
}

impl FromWorld for ImageHandles {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();

        let pixel_art_settings = |settings: &mut ImageLoaderSettings| {
            // Use `nearest` image sampling to preserve the pixel art style.
            settings.sampler = ImageSampler::nearest();
        };

        let pixel_art_paths = [Self::PATH_DUCKY];
        let map = pixel_art_paths
            .into_iter()
            .map(|path| {
                (
                    path.to_string(),
                    asset_server.load_with_settings(path, pixel_art_settings),
                )
            })
            .collect();

        Self(map)
    }
}

/// Stores the handles for background music, aka soundtracks.
#[derive(Resource, Debug, Deref, DerefMut, Reflect)]
#[reflect(Resource)]
pub struct BgmHandles(HashMap<String, Handle<AudioSource>>);

impl BgmHandles {
    pub const PATH_CREDITS: &'static str = "audio/bgm/Monkeys Spinning Monkeys.ogg";
    pub const PATH_GAMEPLAY: &'static str = "audio/bgm/Fluffing A Duck.ogg";
}

impl FromWorld for BgmHandles {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();

        let paths = [Self::PATH_CREDITS, Self::PATH_GAMEPLAY];
        let map = paths
            .into_iter()
            .map(|path| (path.to_string(), asset_server.load(path)))
            .collect();

        Self(map)
    }
}

/// The values stored here are a `Vec<Handle<AudioSource>>` because
/// a single sound effect can have multiple variations.
#[derive(Resource, Debug, Deref, DerefMut, Reflect)]
#[reflect(Resource)]
pub struct SfxHandles(HashMap<String, Vec<Handle<AudioSource>>>);

impl SfxHandles {
    pub const PATH_BUTTON_HOVER: &'static str = "audio/sfx/button_hover.ogg";
    pub const PATH_BUTTON_PRESS: &'static str = "audio/sfx/button_press.ogg";
    pub const PATH_STEP: &'static str = "audio/sfx/step";
}

impl FromWorld for SfxHandles {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource::<AssetServer>().unwrap();

        let paths = [Self::PATH_BUTTON_HOVER, Self::PATH_BUTTON_PRESS];
        let mut map: HashMap<_, _> = paths
            .into_iter()
            .map(|path| (path.to_string(), vec![asset_server.load(path)]))
            .collect();

        // Using string parsing to strip numbered suffixes + `AssetServer::load_folder`
        // is a good way to load many sound effects at once, but is not supported on
        // Wasm or Android.
        const STEP_VARIATIONS: u32 = 4;
        let mut step_sfx = Vec::new();
        for i in 1..=STEP_VARIATIONS {
            let file = format!("{key}{i}.ogg", key = Self::PATH_STEP);
            step_sfx.push(asset_server.load(file));
        }
        map.insert(Self::PATH_STEP.to_string(), step_sfx);

        Self(map)
    }
}
