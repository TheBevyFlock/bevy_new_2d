//! Helper functions for creating common widgets.

use std::borrow::Cow;

use bevy::{ecs::system::IntoObserverSystem, prelude::*};

use crate::theme::{interaction::InteractionPalette, palette::*};

/// A root UI node that fills the window and centers its content.
pub fn ui_root(name: impl Into<Cow<'static, str>>) -> impl Scene {
    let name = name.into();
    bsn! {
        Name::new(name)
        Node {
            position_type: PositionType::Absolute,
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            row_gap: px(20),
        }
        // Don't block picking events for other UI roots.
        Pickable::IGNORE
    }
}

/// A simple header label. Bigger than [`label`].
pub fn header(text: impl Into<String>) -> impl Scene {
    bsn! {
        #Header
        Text(text)
        TextFont { font_size: px(40.0) }
        TextColor(HEADER_TEXT)
    }
}

/// A simple text label.
pub fn label(text: impl Into<String>) -> impl Scene {
    bsn! {
        #Label
        Text(text)
        TextFont { font_size: px(24.0) }
        TextColor(LABEL_TEXT)
    }
}

/// A large rounded button with text and an action defined as an [`Observer`].
pub fn button<E, M, I>(text: impl Into<String>, action: I) -> impl Scene
where
    E: EntityEvent,
    I: IntoObserverSystem<E, M, ()> + Clone + Sync,
    M: 'static,
{
    button_base(
        text,
        action,
        bsn! {
            Node {
                width: px(380),
                height: px(80),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                border_radius: BorderRadius::MAX,
            }
        },
    )
}

/// A small square button with text and an action defined as an [`Observer`].
pub fn button_small<E, M, I>(text: impl Into<String>, action: I) -> impl Scene
where
    E: EntityEvent,
    I: IntoObserverSystem<E, M, ()> + Clone + Sync,
    M: 'static,
{
    button_base(
        text,
        action,
        bsn! {
            Node {
                width: px(30),
                height: px(30),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
            }
        },
    )
}

/// A simple button with text and an action defined as an [`Observer`]. The button's layout is provided by `button_bundle`.
fn button_base<E, M, I>(text: impl Into<String>, action: I, button_bundle: impl Scene) -> impl Scene
where
    E: EventPattern<Event: EntityEvent>,
    I: IntoObserverSystem<E, M, ()> + Clone + Sync,
    M: 'static,
{
    let text = text.into();

    bsn! {
        #Button
        Node
        Children [
            #ButtonInner
            // Button
            BackgroundColor(BUTTON_BACKGROUND)
            InteractionPalette {
                none: BUTTON_BACKGROUND,
                hovered: BUTTON_HOVERED_BACKGROUND,
                pressed: BUTTON_PRESSED_BACKGROUND,
            }
            Children [
                #ButtonText
                Text(text)
                TextFont { font_size: px(40.0) }
                TextColor(BUTTON_TEXT)
                // Don't bubble picking events from the text up to the button.
                Pickable::IGNORE
            ]
            @button_bundle
            @on(action)
        ]
    }
}
