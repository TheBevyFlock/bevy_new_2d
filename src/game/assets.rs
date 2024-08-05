use bevy::{
    prelude::*,
    render::texture::{ImageLoaderSettings, ImageSampler},
    utils::HashMap,
};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<HandleMap<ImageKey>>();
    app.init_resource::<HandleMap<ImageKey>>();

    app.register_type::<HandleMap<SoundtrackKey>>();
    app.init_resource::<HandleMap<SoundtrackKey>>();

    app.register_type::<SoundEffects>();
    app.init_resource::<SoundEffects>();
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Reflect)]
pub enum ImageKey {
    Ducky,
}

impl AssetKey for ImageKey {
    type Asset = Image;
}

impl FromWorld for HandleMap<ImageKey> {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        [(
            ImageKey::Ducky,
            asset_server.load_with_settings(
                "images/ducky.png",
                |settings: &mut ImageLoaderSettings| {
                    settings.sampler = ImageSampler::nearest();
                },
            ),
        )]
        .into()
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Reflect)]
pub enum SoundtrackKey {
    Credits,
    Gameplay,
}

impl AssetKey for SoundtrackKey {
    type Asset = AudioSource;
}

impl FromWorld for HandleMap<SoundtrackKey> {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        [
            (
                SoundtrackKey::Credits,
                asset_server.load("audio/soundtracks/Monkeys Spinning Monkeys.ogg"),
            ),
            (
                SoundtrackKey::Gameplay,
                asset_server.load("audio/soundtracks/Fluffing A Duck.ogg"),
            ),
        ]
        .into()
    }
}

pub trait AssetKey: Sized {
    type Asset: Asset;
}

#[derive(Resource, Reflect, Deref, DerefMut)]
#[reflect(Resource)]
pub struct HandleMap<K: AssetKey>(HashMap<K, Handle<K::Asset>>);

impl<K: AssetKey, T> From<T> for HandleMap<K>
where
    T: Into<HashMap<K, Handle<K::Asset>>>,
{
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

impl<K: AssetKey> HandleMap<K> {
    pub fn all_loaded(&self, asset_server: &AssetServer) -> bool {
        self.values()
            .all(|x| asset_server.is_loaded_with_dependencies(x))
    }
}

/// This uses a map of `String` to `Vec<Handle<AudioSource>>` to store sound effects instead
/// of the `HandleMap` used for the other assets.
/// The key is a `String` because sound effects are often tied closely to file paths or other strings.
/// The value is a `Vec<Handle<AudioSource>>` because a single sound effect can have multiple variations.
#[derive(Resource, Debug, Deref, DerefMut, Reflect)]
#[reflect(Debug, Resource)]
pub struct SoundEffects(HashMap<String, Vec<Handle<AudioSource>>>);

impl SoundEffects {
    pub fn all_loaded(&self, asset_server: &AssetServer) -> bool {
        self.values()
            .flatten()
            .all(|x| asset_server.is_loaded_with_dependencies(x))
    }
}

impl FromWorld for SoundEffects {
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
        map.insert(sound_effects_key::STEP.to_string(), step_sfx);
        map.insert(
            sound_effects_key::BUTTON_HOVER.to_string(),
            vec![asset_server.load("audio/sfx/button_hover.ogg")],
        );
        map.insert(
            sound_effects_key::BUTTON_PRESS.to_string(),
            vec![asset_server.load("audio/sfx/button_press.ogg")],
        );

        Self(map)
    }
}

pub mod sound_effects_key {
    pub const BUTTON_HOVER: &str = "ButtonHover";
    pub const BUTTON_PRESS: &str = "ButtonPress";
    pub const STEP: &str = "Step";
}
