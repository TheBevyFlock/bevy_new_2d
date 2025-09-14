//! Helper functions for creating common widgets.

use std::borrow::Cow;

use bevy::{
    ecs::{
        spawn::SpawnWith,
        system::{IntoObserverSystem, SystemId},
    },
    feathers::{controls::ButtonProps, theme::ThemedText},
    prelude::*,
    ui::Val::*,
    ui_widgets::{Activate, Callback},
};

use crate::theme::{interaction::InteractionPalette, palette::*};

/// A root UI node that fills the window and centers its content.
pub fn ui_root(name: impl Into<Cow<'static, str>>) -> impl Bundle {
    (
        Name::new(name),
        Node {
            position_type: PositionType::Absolute,
            width: Percent(100.0),
            height: Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            row_gap: Px(20.0),
            ..default()
        },
        // Don't block picking events for other UI roots.
        Pickable::IGNORE,
    )
}

/// A simple header label. Bigger than [`label`].
pub fn header(text: impl Into<String>) -> impl Bundle {
    (
        Name::new("Header"),
        Text(text.into()),
        TextFont::from_font_size(40.0),
        TextColor(HEADER_TEXT),
    )
}

/// A simple text label.
pub fn label(text: impl Into<String>) -> impl Bundle {
    (
        Name::new("Label"),
        Text(text.into()),
        TextFont::from_font_size(24.0),
        TextColor(LABEL_TEXT),
    )
}

/// A large rounded button with text and an action defined as an [`Observer`].
pub fn button(text: impl Into<String>, action: SystemId<In<Activate>>) -> impl Bundle {
    button_base(
        text,
        action,
        (
            Node {
                width: Px(380.0),
                height: Px(80.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BorderRadius::MAX,
        ),
    )
}

/// A small square button with text and an action defined as an [`Observer`].
pub fn button_small(text: impl Into<String>, action: SystemId<In<Activate>>) -> impl Bundle {
    button_base(
        text,
        action,
        Node {
            width: Px(30.0),
            height: Px(30.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
    )
}

/// A simple button with text and an action defined as an [`Observer`]. The button's layout is provided by `button_bundle`.
fn button_base(
    text: impl Into<String>,
    action: SystemId<In<Activate>>,
    // TODO: figure out how to add this
    button_bundle: impl Bundle,
) -> impl Bundle {
    bevy::feathers::controls::button(
        ButtonProps {
            on_click: Callback::System(action),
            ..default()
        },
        (),
        Spawn((Text::new(text), ThemedText)),
    )
}
