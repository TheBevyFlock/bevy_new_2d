//! Helper functions for creating common widgets.

use std::borrow::Cow;

use bevy::{prelude::*, ui::Val::*};

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
    )
}

/// A simple button with text.
///
/// Add a [`Pointer<Click>`] observer to the button to make it do something on click.
pub fn button(text: impl Into<String>) -> impl Bundle {
    (
        Name::new("Button"),
        Button,
        Node {
            width: Px(300.0),
            height: Px(80.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        BorderRadius::MAX,
        BackgroundColor(BUTTON_BACKGROUND),
        InteractionPalette {
            none: BUTTON_BACKGROUND,
            hovered: BUTTON_HOVERED_BACKGROUND,
            pressed: BUTTON_PRESSED_BACKGROUND,
        },
        children![(
            Name::new("Button Text"),
            Text(text.into()),
            TextFont::from_font_size(40.0),
            TextColor(BUTTON_TEXT),
        )],
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
