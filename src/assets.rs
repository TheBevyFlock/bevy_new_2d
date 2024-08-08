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
    pub const PATH_DUCKY: &'static str = "images/ducky.png";
}

impl FromWorld for ImageHandles {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();

        let pixel_art_settings = |settings: &mut ImageLoaderSettings| {
            // Use `nearest` image sampling to preserve the pixel art style.
            settings.sampler = ImageSampler::nearest();
        };
        let mut map = HashMap::new();
        map.insert(
            Self::PATH_DUCKY.to_string(),
            asset_server.load_with_settings(Self::PATH_DUCKY, pixel_art_settings),
        );

        Self(map)
    }
}

#[derive(Resource, Debug, Deref, DerefMut, Reflect)]
#[reflect(Resource)]
pub struct SoundtrackHandles(HashMap<String, Handle<AudioSource>>);

impl SoundtrackHandles {
    pub const PATH_CREDITS: &'static str = "audio/soundtracks/Monkeys Spinning Monkeys.ogg";
    pub const PATH_GAMEPLAY: &'static str = "audio/soundtracks/Fluffing A Duck.ogg";
}

impl FromWorld for SoundtrackHandles {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();

        let paths = [
            SoundtrackHandles::PATH_CREDITS,
            SoundtrackHandles::PATH_GAMEPLAY,
        ];
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
pub struct SoundEffectHandles(HashMap<String, Vec<Handle<AudioSource>>>);

impl SoundEffectHandles {
    pub const PATH_BUTTON_HOVER: &'static str = "audio/sfx/button_hover.ogg";
    pub const PATH_BUTTON_PRESS: &'static str = "audio/sfx/button_press.ogg";
    pub const PATH_STEP: &'static str = "audio/sfx/step";
}

impl FromWorld for SoundEffectHandles {
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
