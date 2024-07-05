use bevy::{ecs::system::EntityCommands, prelude::*};

use super::{BUTTON_NORMAL, BUTTON_TEXT, LABEL_TEXT};

/// Helper trait for creating common widgets.
pub trait MyWidgets<'w> {
    /// Spawns a simple root node.
    fn my_root(&mut self) -> EntityCommands;

    /// Spawns a simple button node.
    fn my_button<I: Into<String>>(&mut self, text: I) -> EntityCommands;

    /// Spawns a simple label.
    fn my_label<I: Into<String>>(&mut self, text: I) -> EntityCommands;
}

impl<'w, 's> MyWidgets<'w> for Commands<'w, 's> {
    fn my_root(&mut self) -> EntityCommands {
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

    fn my_button<I: Into<String>>(&mut self, text: I) -> EntityCommands {
        let button = self
            .spawn((ButtonBundle {
                style: Style {
                    width: Val::Px(200.),
                    height: Val::Px(65.),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: BackgroundColor(BUTTON_NORMAL),
                ..default()
            },))
            .id();
        self.spawn(TextBundle::from_section(
            text,
            TextStyle {
                font_size: 40.0,
                color: BUTTON_TEXT,
                ..default()
            },
        ))
        .set_parent(button);
        self.entity(button)
    }

    fn my_label<I: Into<String>>(&mut self, text: I) -> EntityCommands {
        let label = self
            .spawn(NodeBundle {
                style: Style {
                    width: Val::Px(300.),
                    height: Val::Px(65.),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: BackgroundColor(BUTTON_NORMAL),
                ..default()
            })
            .id();
        self.spawn(TextBundle::from_section(
            text,
            TextStyle {
                font_size: 40.0,
                color: LABEL_TEXT,
                ..default()
            },
        ))
        .set_parent(label);
        self.entity(label)
    }
}
