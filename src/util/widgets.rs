//! Helper traits for creating common widgets.

use bevy::{ecs::system::EntityCommands, prelude::*, ui::Val::*};

use super::{
    interaction::InteractionPalette,
    palette::{
        BACKGROUND, BUTTON_HOVERED_BACKGROUND, BUTTON_PRESSED_BACKGROUND, BUTTON_TEXT, LABEL_TEXT,
        NODE_BACKGROUND,
    },
};

/// An internal trait for types that can spawn entities.
trait Spawn {
    fn spawn<B: Bundle>(&mut self, bundle: B) -> EntityCommands;
}

impl Spawn for Commands<'_, '_> {
    fn spawn<B: Bundle>(&mut self, bundle: B) -> EntityCommands {
        self.spawn(bundle)
    }
}

impl Spawn for ChildBuilder<'_> {
    fn spawn<B: Bundle>(&mut self, bundle: B) -> EntityCommands {
        self.spawn(bundle)
    }
}

/// An extension trait for spawning UI containers.
pub trait Containers {
    /// Spawns a root node that covers the full screen
    /// and centers its content horizontally and vertically.
    fn ui_root(&mut self) -> EntityCommands;
}

impl Containers for Commands<'_, '_> {
    fn ui_root(&mut self) -> EntityCommands {
        self.spawn(NodeBundle {
            style: Style {
                width: Percent(100.0),
                height: Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Px(10.0),
                position_type: PositionType::Absolute,
                ..default()
            },
            background_color: BACKGROUND.into(),
            ..default()
        })
    }
}

/// An extension trait for spawning UI widgets.
pub trait Widgets {
    /// Spawn a simple button with text.
    fn button(&mut self, text: impl Into<String>) -> EntityCommands;

    /// Spawn a simple text label.
    fn label(&mut self, text: impl Into<String>) -> EntityCommands;
}

impl<T: Spawn> Widgets for T {
    fn button(&mut self, text: impl Into<String>) -> EntityCommands {
        let mut entity = self.spawn((
            ButtonBundle {
                style: Style {
                    width: Px(200.0),
                    height: Px(65.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: BackgroundColor(NODE_BACKGROUND),
                ..default()
            },
            InteractionPalette {
                none: NODE_BACKGROUND,
                hovered: BUTTON_HOVERED_BACKGROUND,
                pressed: BUTTON_PRESSED_BACKGROUND,
            },
        ));
        entity.with_children(|children| {
            children.spawn(TextBundle::from_section(
                text,
                TextStyle {
                    font_size: 40.0,
                    color: BUTTON_TEXT,
                    ..default()
                },
            ));
        });
        entity
    }

    fn label(&mut self, text: impl Into<String>) -> EntityCommands {
        let mut entity = self.spawn(NodeBundle {
            style: Style {
                width: Px(300.0),
                height: Px(65.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: BackgroundColor(NODE_BACKGROUND),
            ..default()
        });
        entity.with_children(|children| {
            children.spawn(TextBundle::from_section(
                text,
                TextStyle {
                    font_size: 40.0,
                    color: LABEL_TEXT,
                    ..default()
                },
            ));
        });
        entity
    }
}
