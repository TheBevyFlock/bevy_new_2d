use bevy::{
    prelude::*,
    render::texture::{ImageLoaderSettings, ImageSampler},
    utils::HashMap,
};

#[derive(PartialEq, Eq, Hash, Reflect)]
pub enum ImageKey {
    Ducky,
}

impl AssetKey for ImageKey {
    type Asset = Image;

    fn load(asset_server: &AssetServer) -> AssetMap<Self> {
        AssetMap(HashMap::from([(
            ImageKey::Ducky,
            asset_server.load_with_settings(
                "images/ducky.png",
                |settings: &mut ImageLoaderSettings| {
                    settings.sampler = ImageSampler::nearest();
                },
            ),
        )]))
    }
}

#[derive(PartialEq, Eq, Hash, Reflect)]
pub enum SfxKey {
    ButtonHover,
    ButtonPress,
    Step1,
    Step2,
    Step3,
    Step4,
}

impl AssetKey for SfxKey {
    type Asset = AudioSource;

    fn load(asset_server: &AssetServer) -> AssetMap<Self> {
        AssetMap(HashMap::from([
            (
                SfxKey::ButtonHover,
                asset_server.load("audio/sfx/button_hover.ogg"),
            ),
            (
                SfxKey::ButtonPress,
                asset_server.load("audio/sfx/button_press.ogg"),
            ),
            (SfxKey::Step1, asset_server.load("audio/sfx/step1.ogg")),
            (SfxKey::Step2, asset_server.load("audio/sfx/step2.ogg")),
            (SfxKey::Step3, asset_server.load("audio/sfx/step3.ogg")),
            (SfxKey::Step4, asset_server.load("audio/sfx/step4.ogg")),
        ]))
    }
}

#[derive(PartialEq, Eq, Hash, Reflect)]
pub enum SoundtrackKey {
    Credits,
    Gameplay,
}

impl AssetKey for SoundtrackKey {
    type Asset = AudioSource;

    fn load(asset_server: &AssetServer) -> AssetMap<Self> {
        AssetMap(HashMap::from([
            (
                SoundtrackKey::Credits,
                asset_server.load("audio/soundtracks/Monkeys Spinning Monkeys.ogg"),
            ),
            (
                SoundtrackKey::Gameplay,
                asset_server.load("audio/soundtracks/Fluffing A Duck.ogg"),
            ),
        ]))
    }
}

pub trait AssetKey: Sized {
    type Asset: Asset;

    fn load(asset_server: &AssetServer) -> AssetMap<Self>;
}

#[derive(Resource, Reflect, Deref, DerefMut)]
#[reflect(Resource)]
pub struct AssetMap<K: AssetKey>(HashMap<K, Handle<K::Asset>>);

impl<K: AssetKey> AssetMap<K> {
    pub fn all_loaded(&self, asset_server: &AssetServer) -> bool {
        self.0
            .values()
            .all(|x| asset_server.is_loaded_with_dependencies(x))
    }
}
