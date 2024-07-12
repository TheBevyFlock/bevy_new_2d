use bevy::{
    prelude::*,
    render::texture::{ImageLoaderSettings, ImageSampler},
    utils::HashMap,
};

#[derive(PartialEq, Eq, Hash, Reflect)]
pub enum ImageAsset {
    Ducky,
}

#[derive(Resource, Reflect, Deref, DerefMut)]
pub struct ImageAssets(HashMap<ImageAsset, Handle<Image>>);

impl ImageAssets {
    pub fn new(asset_server: &AssetServer) -> Self {
        let mut assets = HashMap::new();

        assets.insert(
            ImageAsset::Ducky,
            asset_server.load_with_settings(
                "images/ducky.png",
                |settings: &mut ImageLoaderSettings| {
                    settings.sampler = ImageSampler::nearest();
                },
            ),
        );

        Self(assets)
    }

    pub fn all_loaded(&self, assets: &Assets<Image>) -> bool {
        self.0.iter().all(|(_, handle)| assets.contains(handle))
    }
}

#[derive(PartialEq, Eq, Hash, Reflect)]
pub enum SfxAsset {
    ButtonHover,
    ButtonPress,
    Step1,
    Step2,
    Step3,
    Step4,
}

#[derive(Resource, Reflect, Deref, DerefMut)]
pub struct SfxAssets(HashMap<SfxAsset, Handle<AudioSource>>);

impl SfxAssets {
    pub fn new(asset_server: &AssetServer) -> Self {
        let mut assets = HashMap::new();

        assets.insert(
            SfxAsset::ButtonHover,
            asset_server.load("audio/sfx/button_hover.ogg"),
        );
        assets.insert(
            SfxAsset::ButtonPress,
            asset_server.load("audio/sfx/button_press.ogg"),
        );
        assets.insert(SfxAsset::Step1, asset_server.load("audio/sfx/step1.ogg"));
        assets.insert(SfxAsset::Step2, asset_server.load("audio/sfx/step2.ogg"));
        assets.insert(SfxAsset::Step3, asset_server.load("audio/sfx/step3.ogg"));
        assets.insert(SfxAsset::Step4, asset_server.load("audio/sfx/step4.ogg"));

        Self(assets)
    }

    pub fn all_loaded(&self, assets: &Assets<AudioSource>) -> bool {
        self.0.iter().all(|(_, handle)| assets.contains(handle))
    }
}

#[derive(PartialEq, Eq, Hash, Reflect)]
pub enum SoundtrackAsset {
    Credits,
    Gameplay,
}

#[derive(Resource, Reflect, Deref, DerefMut)]
pub struct SoundtrackAssets(HashMap<SoundtrackAsset, Handle<AudioSource>>);

impl SoundtrackAssets {
    pub fn new(asset_server: &AssetServer) -> Self {
        let mut assets = HashMap::new();
        assets.insert(
            SoundtrackAsset::Credits,
            asset_server.load("audio/soundtracks/Monkeys Spinning Monkeys.ogg"),
        );
        assets.insert(
            SoundtrackAsset::Gameplay,
            asset_server.load("audio/soundtracks/Fluffing A Duck.ogg"),
        );
        Self(assets)
    }

    pub fn all_loaded(&self, assets: &Assets<AudioSource>) -> bool {
        self.0.iter().all(|(_, handle)| assets.contains(handle))
    }
}
