//! Helper traits for creating common widgets.

use bevy::{ecs::system::EntityCommands, prelude::*};

use super::{
    mouse_hover::InteractionPalette,
    palette::{
        BUTTON_HOVERED_BACKGROUND, BUTTON_PRESSED_BACKGROUND, BUTTON_TEXT, LABEL_TEXT,
        NODE_BACKGROUND,
    },
};

/// Internal trait for things that can spawn entities.
trait Spawner<'a> {
    fn spawn<B: Bundle>(&mut self, bundle: B) -> EntityCommands;
}

impl<'a> Spawner<'a> for Commands<'a, 'a> {
    fn spawn<B: Bundle>(&mut self, bundle: B) -> EntityCommands {
        self.spawn(bundle)
    }
}

impl<'a> Spawner<'a> for ChildBuilder<'a> {
    fn spawn<B: Bundle>(&mut self, bundle: B) -> EntityCommands {
        self.spawn(bundle)
    }
}

/// Root container spawning trait.
pub(crate) trait RootContainers {
    /// Spawns root node that covers full screen and verticaly and horizontaly centers it's content.
    fn ui_root(&mut self) -> EntityCommands;
}

impl<'a, 'b> RootContainers for Commands<'a, 'b> {
    fn ui_root(&mut self) -> EntityCommands {
        self.spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(10.),
                position_type: PositionType::Absolute,
                ..default()
            },
            ..default()
        })
    }
}

/// Widgets spawning trait.
pub(crate) trait Widgets<'a> {
    /// Spawns a simple button node with text.
    fn button<I: Into<String>>(&mut self, text: I) -> EntityCommands;

    /// Spawns a simple label.
    fn label<I: Into<String>>(&mut self, text: I) -> EntityCommands;
}

impl<'a, T: Spawner<'a>> Widgets<'a> for T {
    fn button<I: Into<String>>(&mut self, text: I) -> EntityCommands {
        let mut entity_commands = self.spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(200.),
                    height: Val::Px(65.),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: BackgroundColor(NODE_BACKGROUND),
                ..default()
            },
            InteractionPalette::new(
                NODE_BACKGROUND,
                BUTTON_HOVERED_BACKGROUND,
                BUTTON_PRESSED_BACKGROUND,
            ),
        ));
        entity_commands.with_children(|children| {
            children.spawn(TextBundle::from_section(
                text,
                TextStyle {
                    font_size: 40.0,
                    color: BUTTON_TEXT,
                    ..default()
                },
            ));
        });
        entity_commands
    }

    fn label<I: Into<String>>(&mut self, text: I) -> EntityCommands {
        let mut entity_commands = self.spawn(NodeBundle {
            style: Style {
                width: Val::Px(300.),
                height: Val::Px(65.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: BackgroundColor(NODE_BACKGROUND),
            ..default()
        });
        entity_commands.with_children(|children| {
            children.spawn(TextBundle::from_section(
                text,
                TextStyle {
                    font_size: 40.0,
                    color: LABEL_TEXT,
                    ..default()
                },
            ));
        });
        entity_commands
    }
}
