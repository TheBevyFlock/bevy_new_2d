use bevy::{
    prelude::*,
    render::texture::{ImageLoaderSettings, ImageSampler},
    utils::HashMap,
};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<AssetMap<ImageKey>>();
    app.init_resource::<AssetMap<ImageKey>>();

    app.register_type::<AssetMap<SfxKey>>();
    app.init_resource::<AssetMap<SfxKey>>();

    app.register_type::<AssetMap<SoundtrackKey>>();
    app.init_resource::<AssetMap<SoundtrackKey>>();
}

#[derive(PartialEq, Eq, Hash, Reflect)]
pub enum ImageKey {
    Ducky,
}

impl AssetKey for ImageKey {
    type Asset = Image;

    fn load(asset_server: &AssetServer) -> impl Into<AssetMap<Self>> {
        [(
            Self::Ducky,
            asset_server.load_with_settings(
                "images/ducky.png",
                |settings: &mut ImageLoaderSettings| {
                    settings.sampler = ImageSampler::nearest();
                },
            ),
        )]
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

    fn load(asset_server: &AssetServer) -> impl Into<AssetMap<Self>> {
        [
            (
                Self::ButtonHover,
                asset_server.load("audio/sfx/button_hover.ogg"),
            ),
            (
                Self::ButtonPress,
                asset_server.load("audio/sfx/button_press.ogg"),
            ),
            (Self::Step1, asset_server.load("audio/sfx/step1.ogg")),
            (Self::Step2, asset_server.load("audio/sfx/step2.ogg")),
            (Self::Step3, asset_server.load("audio/sfx/step3.ogg")),
            (Self::Step4, asset_server.load("audio/sfx/step4.ogg")),
        ]
    }
}

#[derive(PartialEq, Eq, Hash, Reflect)]
pub enum SoundtrackKey {
    Credits,
    Gameplay,
}

impl AssetKey for SoundtrackKey {
    type Asset = AudioSource;

    fn load(asset_server: &AssetServer) -> impl Into<AssetMap<Self>> {
        [
            (
                Self::Credits,
                asset_server.load("audio/soundtracks/Monkeys Spinning Monkeys.ogg"),
            ),
            (
                Self::Gameplay,
                asset_server.load("audio/soundtracks/Fluffing A Duck.ogg"),
            ),
        ]
    }
}

pub trait AssetKey: Sized {
    type Asset: Asset;

    fn load(asset_server: &AssetServer) -> impl Into<AssetMap<Self>>;
}

#[derive(Resource, Reflect, Deref, DerefMut)]
#[reflect(Resource)]
pub struct AssetMap<K: AssetKey>(HashMap<K, Handle<K::Asset>>);

impl<K: AssetKey, T> From<T> for AssetMap<K>
where
    T: Into<HashMap<K, Handle<K::Asset>>>,
{
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

impl<K: AssetKey> FromWorld for AssetMap<K> {
    fn from_world(world: &mut World) -> Self {
        K::load(&world.resource::<AssetServer>()).into()
    }
}

impl<K: AssetKey> AssetMap<K> {
    pub fn all_loaded(&self, asset_server: &AssetServer) -> bool {
        self.values()
            .all(|x| asset_server.is_loaded_with_dependencies(x))
    }
}
