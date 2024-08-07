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

    app.register_type::<SoundtrackHandles>();
    app.init_resource::<SoundtrackHandles>();

    app.register_type::<SoundEffectHandles>();
    app.init_resource::<SoundEffectHandles>();
}

#[derive(Resource, Debug, Deref, DerefMut, Reflect)]
#[reflect(Resource)]
pub struct ImageHandles(HashMap<String, Handle<Image>>);

impl ImageHandles {
    pub const KEY_DUCKY: &'static str = "images/ducky.png";
}

impl FromWorld for ImageHandles {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        let pixel_art_settings = |settings: &mut ImageLoaderSettings| {
            // Using nearest sampling to preserve pixel art style
            settings.sampler = ImageSampler::nearest();
        };
        let pixel_art_files = [ImageHandles::KEY_DUCKY];
        let map = pixel_art_files
            .into_iter()
            .map(|key| {
                (
                    key.to_string(),
                    asset_server.load_with_settings(key, pixel_art_settings),
                )
            })
            .collect();
        Self(map)
    }
}

#[derive(Resource, Debug, Deref, DerefMut, Reflect)]
#[reflect(Resource)]
pub struct SoundtrackHandles(HashMap<String, Handle<AudioSource>>);

impl SoundtrackHandles {
    pub const KEY_CREDITS: &'static str = "audio/soundtracks/Monkeys Spinning Monkeys.ogg";
    pub const KEY_GAMEPLAY: &'static str = "audio/soundtracks/Fluffing A Duck.ogg";
}

impl FromWorld for SoundtrackHandles {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        let files = [
            SoundtrackHandles::KEY_CREDITS,
            SoundtrackHandles::KEY_GAMEPLAY,
        ];
        let map = files
            .into_iter()
            .map(|file| (file.to_string(), asset_server.load(file)))
            .collect();
        Self(map)
    }
}

/// The values stored here are a `Vec<Handle<AudioSource>>` because
/// a single sound effect can have multiple variations.
#[derive(Resource, Debug, Deref, DerefMut, Reflect)]
#[reflect(Resource)]
pub struct SoundEffectHandles(HashMap<String, Vec<Handle<AudioSource>>>);

impl SoundEffectHandles {
    pub const KEY_BUTTON_HOVER: &'static str = "audio/sfx/button_hover.ogg";
    pub const KEY_BUTTON_PRESS: &'static str = "audio/sfx/button_press.ogg";
    pub const KEY_STEP: &'static str = "audio/sfx/step";
}

impl FromWorld for SoundEffectHandles {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource::<AssetServer>().unwrap();
        let files = [Self::KEY_BUTTON_HOVER, Self::KEY_BUTTON_PRESS];
        let mut map: HashMap<_, _> = files
            .into_iter()
            .map(|file| (file.to_string(), vec![asset_server.load(file)]))
            .collect();

        // Using string parsing to strip numbered suffixes + `AssetServer::load_folder`
        // is a good way to load many sound effects at once, but is not supported on
        // Wasm or Android.
        let variation_files = [(Self::KEY_STEP, 4)];
        let variations = variation_files.into_iter().map(|(key, variations)| {
            (
                key.to_string(),
                (1..=variations)
                    .map(|i| asset_server.load(&format!("{key}{i}.ogg")))
                    .collect(),
            )
        });
        map.extend(variations);

        Self(map)
    }
}
