//! Common UI widgets.

use bevy::{
    ecs::system::{EntityCommand, EntityCommands, RunSystemOnce as _, SystemId},
    prelude::*,
    ui::Val::*,
};

use super::{
    interaction::{InteractionPalette, OnPress},
    palette::*,
};
use crate::spawn::{SpawnExt, WorldSpawnExt};

/// An extension trait to improve the ergonomics of deferred spawning UI widgets.
pub trait Widgets {
    /// Spawn a [`Button`].
    fn button(&mut self, text: impl Into<String>, on_press: SystemId) -> EntityCommands;

    /// Spawn a [`Header`].
    fn header(&mut self, text: impl Into<String>) -> EntityCommands;

    /// Spawn a [`Label`].
    fn label(&mut self, text: impl Into<String>) -> EntityCommands;
}

impl<T: SpawnExt> Widgets for T {
    fn button(&mut self, text: impl Into<String>, on_press: SystemId) -> EntityCommands {
        self.spawn_with(Button::new(text, on_press))
    }

    fn header(&mut self, text: impl Into<String>) -> EntityCommands {
        self.spawn_with(Header::new(text))
    }

    fn label(&mut self, text: impl Into<String>) -> EntityCommands {
        self.spawn_with(Label::new(text))
    }
}

/// An extension trait to improve the ergonomics of immediate spawning UI widgets.
pub trait WorldWidgets {
    /// Spawn a [`Button`].
    fn button(&mut self, text: impl Into<String>, on_press: SystemId) -> EntityWorldMut;

    /// Spawn a [`Header`].
    fn header(&mut self, text: impl Into<String>) -> EntityWorldMut;

    /// Spawn a [`Label`].
    fn label(&mut self, text: impl Into<String>) -> EntityWorldMut;
}

impl<T: WorldSpawnExt> WorldWidgets for T {
    fn button(&mut self, text: impl Into<String>, on_press: SystemId) -> EntityWorldMut {
        self.spawn_with(Button::new(text, on_press))
    }

    fn header(&mut self, text: impl Into<String>) -> EntityWorldMut {
        self.spawn_with(Header::new(text))
    }

    fn label(&mut self, text: impl Into<String>) -> EntityWorldMut {
        self.spawn_with(Label::new(text))
    }
}

/// A simple button with text.
pub struct Button {
    pub text: String,
    pub on_press: SystemId,
}

impl EntityCommand for Button {
    fn apply(self, id: Entity, world: &mut World) {
        world.run_system_once_with((id, self), Self::construct);
    }
}

impl Button {
    pub fn new(text: impl Into<String>, on_press: SystemId) -> Self {
        Self {
            text: text.into(),
            on_press,
        }
    }

    fn construct(In((id, this)): In<(Entity, Self)>, mut commands: Commands) {
        commands
            .entity(id)
            .insert((
                Name::new("Button"),
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
                OnPress(this.on_press),
                InteractionPalette {
                    none: NODE_BACKGROUND,
                    hovered: BUTTON_HOVERED_BACKGROUND,
                    pressed: BUTTON_PRESSED_BACKGROUND,
                },
            ))
            .with_children(|children| {
                children.spawn((
                    Name::new("Button text"),
                    TextBundle::from_section(
                        this.text,
                        TextStyle {
                            font_size: 40.0,
                            color: BUTTON_TEXT,
                            ..default()
                        },
                    ),
                ));
            });
    }
}

/// A simple header label. Bigger than [`Label`].
pub struct Header {
    pub text: String,
}

impl EntityCommand for Header {
    fn apply(self, id: Entity, world: &mut World) {
        world.run_system_once_with((id, self), Self::construct);
    }
}

impl Header {
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }

    fn construct(In((id, this)): In<(Entity, Self)>, mut commands: Commands) {
        commands
            .entity(id)
            .insert((
                Name::new("Header"),
                NodeBundle {
                    style: Style {
                        width: Px(500.0),
                        height: Px(65.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: BackgroundColor(NODE_BACKGROUND),
                    ..default()
                },
            ))
            .with_children(|children| {
                children.spawn((
                    Name::new("Header text"),
                    TextBundle::from_section(
                        this.text,
                        TextStyle {
                            font_size: 40.0,
                            color: HEADER_TEXT,
                            ..default()
                        },
                    ),
                ));
            });
    }
}

/// A simple text label.
pub struct Label {
    pub text: String,
}

impl EntityCommand for Label {
    fn apply(self, id: Entity, world: &mut World) {
        world.run_system_once_with((id, self), Self::construct);
    }
}

impl Label {
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }

    fn construct(In((id, this)): In<(Entity, Self)>, mut commands: Commands) {
        commands
            .entity(id)
            .insert((
                Name::new("Label"),
                NodeBundle {
                    style: Style {
                        width: Px(500.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ..default()
                },
            ))
            .with_children(|children| {
                children.spawn((
                    Name::new("Label text"),
                    TextBundle::from_section(
                        this.text,
                        TextStyle {
                            font_size: 24.0,
                            color: LABEL_TEXT,
                            ..default()
                        },
                    ),
                ));
            });
    }
}

/// Construct a UI node that fills the window and centers its children.
pub fn ui_root(id: Entity, world: &mut World) {
    world.entity_mut(id).insert((
        Name::new("UI root"),
        NodeBundle {
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
            ..default()
        },
    ));
}
