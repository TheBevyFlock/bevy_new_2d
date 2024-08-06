use bevy::prelude::*;

use crate::{
    assets::ImageHandles,
    demo::{
        animation::PlayerAnimation,
        movement::{MovementController, ScreenWrap},
    },
    AppSet,
};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Player>();

    // Record directional input as movement controls.
    app.add_systems(
        Update,
        record_player_directional_input.in_set(AppSet::RecordInput),
    );
}

/// A marker component for the player entity.
#[derive(Component, Reflect, Clone, Copy, PartialEq, Eq, Debug)]
#[reflect(Component)]
pub struct Player;

/// Spawn a player entity.
pub fn player(
    In(id): In<Entity>,
    mut commands: Commands,
    image_handles: Res<ImageHandles>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // A texture atlas is a way to split one image into multiple sprites using a grid.
    // By attaching it to a [`SpriteBundle`] and changing the index, we can change which
    // part of the grid will be visible. This is used to animate the player character.
    //
    // See this example: https://github.com/bevyengine/bevy/blob/latest/examples/2d/texture_atlas.rs
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 6, 2, Some(UVec2::splat(1)), None);
    let layout = layouts.add(layout);
    let animation = PlayerAnimation::new();

    commands.entity(id).insert((
        Name::new("Player"),
        Player,
        SpriteBundle {
            texture: image_handles[ImageHandles::KEY_DUCKY].clone_weak(),
            transform: Transform::from_scale(Vec2::splat(8.0).extend(1.0)),
            ..default()
        },
        TextureAtlas {
            layout: layout.clone(),
            index: animation.get_atlas_index(),
        },
        animation,
        MovementController::default(),
        ScreenWrap,
    ));
}

fn record_player_directional_input(
    input: Res<ButtonInput<KeyCode>>,
    mut controller_query: Query<&mut MovementController, With<Player>>,
) {
    // Collect directional input.
    let mut intent = Vec2::ZERO;
    if input.pressed(KeyCode::KeyW) || input.pressed(KeyCode::ArrowUp) {
        intent.y += 1.0;
    }
    if input.pressed(KeyCode::KeyS) || input.pressed(KeyCode::ArrowDown) {
        intent.y -= 1.0;
    }
    if input.pressed(KeyCode::KeyA) || input.pressed(KeyCode::ArrowLeft) {
        intent.x -= 1.0;
    }
    if input.pressed(KeyCode::KeyD) || input.pressed(KeyCode::ArrowRight) {
        intent.x += 1.0;
    }

    // Normalize so that diagonal movement has the same speed as
    // horizontal and vertical movement.
    // This should be omitted if the input comes from an analog stick instead.
    let intent = intent.normalize_or_zero();

    // Apply movement intent to controllers.
    for mut controller in &mut controller_query {
        controller.intent = intent;
    }
}
