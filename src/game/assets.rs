//! This module contains the asset handles used throughout the game.
//! During `Screen::Loading`, the game will load the assets specified here.
//! Your systems can then request the resources defined here to access the loaded assets.

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
    pub const DUCKY: &'static str = "Ducky";
}

impl FromWorld for ImageHandles {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        let map = [(
            ImageHandles::DUCKY.to_string(),
            asset_server.load_with_settings(
                "images/ducky.png",
                |settings: &mut ImageLoaderSettings| {
                    settings.sampler = ImageSampler::nearest();
                },
            ),
        )]
        .into();
        Self(map)
    }
}

#[derive(Resource, Debug, Deref, DerefMut, Reflect)]
#[reflect(Resource)]
pub struct SoundtrackHandles(HashMap<String, Handle<AudioSource>>);

impl SoundtrackHandles {
    pub const CREDITS: &'static str = "Credits";
    pub const GAMEPLAY: &'static str = "Gameplay";
}

impl FromWorld for SoundtrackHandles {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        let map = [
            (
                SoundtrackHandles::CREDITS.to_string(),
                asset_server.load("audio/soundtracks/Monkeys Spinning Monkeys.ogg"),
            ),
            (
                SoundtrackHandles::GAMEPLAY.to_string(),
                asset_server.load("audio/soundtracks/Fluffing A Duck.ogg"),
            ),
        ]
        .into();
        Self(map)
    }
}

/// The values stored here are a `Vec<Handle<AudioSource>>` because
/// a single sound effect can have multiple variations.
#[derive(Resource, Debug, Deref, DerefMut, Reflect)]
#[reflect(Resource)]
pub struct SoundEffectHandles(HashMap<String, Vec<Handle<AudioSource>>>);

impl SoundEffectHandles {
    pub const BUTTON_HOVER: &'static str = "ButtonHover";
    pub const BUTTON_PRESS: &'static str = "ButtonPress";
    pub const STEP: &'static str = "Step";
}

impl FromWorld for SoundEffectHandles {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource::<AssetServer>().unwrap();
        let mut map = HashMap::default();

        // Load sound effects here.
        // Using string parsing to strip numbered suffixes + `AssetServer::load_folder` is a good way to load many sound effects at once, but is not supported on Wasm or Android.
        let step_sfx = vec![
            asset_server.load("audio/sfx/step1.ogg"),
            asset_server.load("audio/sfx/step2.ogg"),
            asset_server.load("audio/sfx/step3.ogg"),
            asset_server.load("audio/sfx/step4.ogg"),
        ];
        map.insert(Self::STEP.to_string(), step_sfx);
        map.insert(
            Self::BUTTON_HOVER.to_string(),
            vec![asset_server.load("audio/sfx/button_hover.ogg")],
        );
        map.insert(
            Self::BUTTON_PRESS.to_string(),
            vec![asset_server.load("audio/sfx/button_press.ogg")],
        );

        Self(map)
    }
}
