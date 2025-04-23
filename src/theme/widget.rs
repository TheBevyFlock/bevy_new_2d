//! Helper functions for creating common widgets.

use std::borrow::Cow;

use bevy::{
    ecs::{spawn::SpawnableList, system::IntoObserverSystem},
    prelude::*,
    ui::Val::*,
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

/// A simple button with text and an action defined as an [`Observer`].
pub fn button<E: Event, B: Bundle, M, I: IntoObserverSystem<E, B, M>>(
    text: impl Into<String>,
    action: I,
) -> impl Bundle {
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
        Children::spawn((
            Spawn((
                Name::new("Button Text"),
                Text(text.into()),
                TextFont::from_font_size(40.0),
                TextColor(BUTTON_TEXT),
            )),
            SpawnObserver(Observer::new(action)),
        )),
    )
}

/// A [`SpawnableList`] that spawns an [`Observer`] as a child entity.
struct SpawnObserver(Observer);

impl SpawnableList<ChildOf> for SpawnObserver {
    fn spawn(self, world: &mut World, entity: Entity) {
        world.spawn(self.0.with_entity(entity));
    }

    // Size hint is not important for this simple use case, so return 0.
    fn size_hint(&self) -> usize {
        0
    }
}
