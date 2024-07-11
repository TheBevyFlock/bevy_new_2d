//! Handle player input and translate it into movement.
//! Note that the approach used here is simple for demonstration purposes.
//! If you want to move the player in a smoother way,
//! consider using a [fixed timestep](https://github.com/bevyengine/bevy/pull/14223).

use std::time::Duration;

use bevy::{prelude::*, window::PrimaryWindow};

use super::audio::sfx::Sfx;
use crate::AppSet;

pub(super) fn plugin(app: &mut App) {
    // Record directional input as movement controls.
    app.register_type::<MovementController>();
    app.add_systems(
        Update,
        record_movement_controller.in_set(AppSet::RecordInput),
    );

    // Apply movement based on controls.
    app.register_type::<(Movement, WrapWithinWindow)>();
    app.add_systems(
        Update,
        (apply_movement, wrap_within_window)
            .chain()
            .in_set(AppSet::Update),
    );

    // Update facing based on controls.
    app.add_systems(Update, update_facing.in_set(AppSet::Update));

    // Animate and play sound effects on controls.
    app.register_type::<PlayerAnimation>();
    app.add_systems(
        Update,
        (update_animation, trigger_step_sfx)
            .chain()
            .in_set(AppSet::Update),
    );
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct MovementController(pub Vec2);

fn record_movement_controller(
    input: Res<ButtonInput<KeyCode>>,
    mut controller_query: Query<&mut MovementController>,
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
    let intent = intent.normalize_or_zero();

    // Apply movement intent to controllers.
    for mut controller in &mut controller_query {
        controller.0 = intent;
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Movement {
    /// Since Bevy's default 2D camera setup is scaled such that
    /// one unit is one pixel, you can think of this as
    /// "How many pixels per second should the player move?"
    /// Note that physics engines may use different unit/pixel ratios.
    pub speed: f32,
}

fn apply_movement(
    time: Res<Time>,
    mut movement_query: Query<(&MovementController, &Movement, &mut Transform)>,
) {
    for (controller, movement, mut transform) in &mut movement_query {
        let velocity = movement.speed * controller.0;
        transform.translation += velocity.extend(0.0) * time.delta_seconds();
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct WrapWithinWindow;

fn wrap_within_window(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut wrap_query: Query<&mut Transform, With<WrapWithinWindow>>,
) {
    let size = window_query.single().size() + 200.0;
    let half_size = size / 2.0;
    for mut transform in &mut wrap_query {
        let position = transform.translation.xy();
        let wrapped = (position + half_size).rem_euclid(size) - half_size;
        transform.translation = wrapped.extend(transform.translation.z);
    }
}

fn update_facing(mut player_query: Query<(&MovementController, &mut Sprite)>) {
    for (controller, mut sprite) in &mut player_query {
        let dx = controller.0.x;
        if dx != 0.0 {
            sprite.flip_x = dx < 0.0;
        }
    }
}

/// Update the animation.
fn update_animation(
    time: Res<Time>,
    mut query: Query<(&mut PlayerAnimation, &MovementController, &mut TextureAtlas)>,
) {
    for (mut animation, controller, mut atlas) in &mut query {
        animation.update_timer(time.delta());

        let animation_state = if controller.0 == Vec2::ZERO {
            PlayerAnimationState::Idle
        } else {
            PlayerAnimationState::Walking
        };
        animation.update_state(animation_state);

        if animation.changed() {
            atlas.index = animation.get_atlas_index();
        }
    }
}

/// If the player is moving, play a step sound effect synchronized with the animation.
fn trigger_step_sfx(mut commands: Commands, mut step_query: Query<&PlayerAnimation>) {
    for animation in &mut step_query {
        if animation.state == PlayerAnimationState::Walking
            && animation.changed()
            && (animation.frame == 1 || animation.frame == 4)
        {
            commands.trigger(Sfx::Step);
        }
    }
}

/// Component that tracks player's animation state.
/// It is tightly bound to the texture atlas we use.
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct PlayerAnimation {
    timer: Timer,
    frame: usize,
    state: PlayerAnimationState,
}

#[derive(Reflect, PartialEq)]
pub enum PlayerAnimationState {
    Idle,
    Walking,
}

impl PlayerAnimation {
    /// How many idle frames are there.
    const IDLE_FRAMES: usize = 2;
    /// What's the interval between idle frames.
    const IDLE_INTERVAL: Duration = Duration::from_millis(500);

    fn idle() -> Self {
        Self {
            timer: Timer::new(Self::IDLE_INTERVAL, TimerMode::Repeating),
            frame: 0,
            state: PlayerAnimationState::Idle,
        }
    }

    /// How many walking frames are there.
    const WALKING_FRAMES: usize = 6;
    /// What's the interval between walking frames.
    const WALKING_INTERVAL: Duration = Duration::from_millis(50);

    fn walking() -> Self {
        Self {
            timer: Timer::new(Self::WALKING_INTERVAL, TimerMode::Repeating),
            frame: 0,
            state: PlayerAnimationState::Walking,
        }
    }

    pub fn new() -> Self {
        Self::idle()
    }

    /// Update animation timers.
    pub fn update_timer(&mut self, delta: Duration) {
        self.timer.tick(delta);
        if !self.timer.finished() {
            return;
        }
        match self.state {
            PlayerAnimationState::Idle => {
                self.frame = (self.frame + 1) % Self::IDLE_FRAMES;
            }
            PlayerAnimationState::Walking => {
                self.frame = (self.frame + 1) % Self::WALKING_FRAMES;
            }
        }
    }

    /// Update animation state if it changes.
    pub fn update_state(&mut self, state: PlayerAnimationState) {
        if self.state != state {
            match state {
                PlayerAnimationState::Idle => *self = Self::idle(),
                PlayerAnimationState::Walking => *self = Self::walking(),
            }
        }
    }

    /// Whether animation changed this tick.
    pub fn changed(&self) -> bool {
        self.timer.finished()
    }

    /// Return sprite index in the atlas.
    pub fn get_atlas_index(&self) -> usize {
        match self.state {
            PlayerAnimationState::Idle => self.frame,
            PlayerAnimationState::Walking => 6 + self.frame,
        }
    }
}
