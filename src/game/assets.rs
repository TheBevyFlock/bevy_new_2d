use bevy::prelude::*;

#[derive(Resource, Reflect)]
pub struct ImageAssets {
    pub ducky: Handle<Image>,
}

impl ImageAssets {
    pub fn new(asset_server: &AssetServer) -> Self {
        Self {
            ducky: asset_server.load("images/ducky.png"),
        }
    }

    pub fn all_loaded(&self, assets: &Assets<Image>) -> bool {
        assets.contains(&self.ducky)
    }
}

#[derive(Resource, Reflect)]
pub struct SfxAssets {
    pub button_hover: Handle<AudioSource>,
    pub button_press: Handle<AudioSource>,
    pub step1: Handle<AudioSource>,
    pub step2: Handle<AudioSource>,
    pub step3: Handle<AudioSource>,
    pub step4: Handle<AudioSource>,
}

impl SfxAssets {
    pub fn new(asset_server: &AssetServer) -> Self {
        Self {
            button_hover: asset_server.load("audio/sfx/button_hover.ogg"),
            button_press: asset_server.load("audio/sfx/button_press.ogg"),
            step1: asset_server.load("audio/sfx/step1.ogg"),
            step2: asset_server.load("audio/sfx/step2.ogg"),
            step3: asset_server.load("audio/sfx/step3.ogg"),
            step4: asset_server.load("audio/sfx/step4.ogg"),
        }
    }

    pub fn all_loaded(&self, assets: &Assets<AudioSource>) -> bool {
        assets.contains(&self.button_hover)
            && assets.contains(&self.button_press)
            && assets.contains(&self.step1)
            && assets.contains(&self.step2)
            && assets.contains(&self.step3)
            && assets.contains(&self.step4)
    }
}

#[derive(Resource, Reflect)]
pub struct SoundtrackAssets {
    pub credits: Handle<AudioSource>,
    pub gameplay: Handle<AudioSource>,
}

impl SoundtrackAssets {
    pub fn new(asset_server: &AssetServer) -> Self {
        Self {
            credits: asset_server.load("audio/soundtracks/Monkeys Spinning Monkeys.ogg"),
            gameplay: asset_server.load("audio/soundtracks/Fluffing A Duck.ogg"),
        }
    }

    pub fn all_loaded(&self, assets: &Assets<AudioSource>) -> bool {
        assets.contains(&self.credits) && assets.contains(&self.gameplay)
    }
}
