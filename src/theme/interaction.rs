use crate::{asset_tracking::LoadResource, screens::Screen};
use bevy::{
    ecs::{system::SystemId, world::Command},
    prelude::*,
};
use std::{collections::VecDeque, marker::PhantomData};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<InteractionPalette>();
    app.load_resource::<ButtonAssets>();
    app.add_systems(
        Update,
        (
            trigger_on_press,
            apply_interaction_palette,
            trigger_interaction_sfx,
        )
            .run_if(not(in_state(Screen::Loading)))
            .run_if(not(in_state(Screen::Splash))),
    );
}

/// Palette for widget interactions. Add this to an entity that supports
/// [`Interaction`]s, such as a button, to change its [`BackgroundColor`] based
/// on the current interaction state.
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct InteractionPalette {
    pub none: Color,
    pub hovered: Color,
    pub pressed: Color,
}

/// Event triggered on a UI entity when the [`Interaction`] component on the same entity changes to
/// [`Interaction::Pressed`]. Observe this event to detect e.g. button presses.
#[derive(Event)]
pub struct OnPress;

fn trigger_on_press(
    interaction_query: Query<(Entity, &Interaction), Changed<Interaction>>,
    mut commands: Commands,
) {
    for (entity, interaction) in &interaction_query {
        if matches!(interaction, Interaction::Pressed) {
            commands.trigger_targets(OnPress, entity);
        }
    }
}

fn apply_interaction_palette(
    mut palette_query: Query<
        (&Interaction, &InteractionPalette, &mut BackgroundColor),
        Changed<Interaction>,
    >,
) {
    for (interaction, palette, mut background) in &mut palette_query {
        *background = match interaction {
            Interaction::None => palette.none,
            Interaction::Hovered => palette.hovered,
            Interaction::Pressed => palette.pressed,
        }
        .into();
    }
}

#[derive(Resource, Asset, Reflect, Clone)]
pub struct ButtonAssets {
    #[dependency]
    hover: Handle<AudioSource>,
    #[dependency]
    press: Handle<AudioSource>,
}

impl ButtonAssets {
    pub const PATH_BUTTON_HOVER: &'static str = "audio/sfx/button_hover.ogg";
    pub const PATH_BUTTON_PRESS: &'static str = "audio/sfx/button_press.ogg";
}

impl FromWorld for ButtonAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            hover: assets.load(Self::PATH_BUTTON_HOVER),
            press: assets.load(Self::PATH_BUTTON_PRESS),
        }
    }
}

fn trigger_interaction_sfx(
    interaction_query: Query<&Interaction, Changed<Interaction>>,
    button_assets: Res<ButtonAssets>,
    mut commands: Commands,
) {
    for interaction in &interaction_query {
        match interaction {
            Interaction::Hovered => {
                commands.spawn(AudioBundle {
                    source: button_assets.hover.clone(),
                    settings: PlaybackSettings::DESPAWN,
                });
            }
            Interaction::Pressed => {
                commands.spawn(AudioBundle {
                    source: button_assets.press.clone(),
                    settings: PlaybackSettings::DESPAWN,
                });
            }
            _ => {}
        }
    }
}
