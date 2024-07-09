//! Run a very simple physics simulation.

use bevy::{
    ecs::component::{ComponentHooks, StorageType},
    prelude::*,
};

pub(super) fn plugin(app: &mut App) {
    // `FixedUpdate` runs before `Update`, so the physics simulation is advanced
    // before the player's visual representation is updated.
    app.add_systems(FixedUpdate, advance_physics);
}

/// How many units per second the player should move.
#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
pub struct Velocity(pub Vec3);

/// The actual transform of the player in the physics simulation.
/// This is separate from the `Transform`, which is merely a visual
/// representation.
/// The reason for this separation is that physics simulations
/// want to run at a fixed timestep, while rendering should run
/// as fast as possible. The rendering will then interpolate between
/// the previous and current physical translation to get a smooth
/// visual representation of the player.
#[derive(Debug, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
pub struct PhysicalTransform(pub Transform);

/// The value that [`PhysicalTranslation`] had in the last fixed timestep.
/// Used for interpolation when rendering.
#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
pub struct PreviousPhysicalTransform(pub Transform);

/// When adding a [`PhysicalTransform`]:
/// - make sure it is always initialized with the same value as the
///   [`Transform`]
/// - add a [`PreviousPhysicalTransform`] as well
impl Component for PhysicalTransform {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    fn register_component_hooks(hooks: &mut ComponentHooks) {
        hooks.on_add(|mut world, entity, _component_id| {
            let rendered_transform = *world.get::<Transform>(entity).unwrap();
            let mut physical_transform = world.get_mut::<PhysicalTransform>(entity).unwrap();
            physical_transform.0 = rendered_transform;
            world
                .commands()
                .entity(entity)
                .insert(PreviousPhysicalTransform(rendered_transform));
        });
    }
}

/// Advance the physics simulation by one fixed timestep. This may run zero or
/// multiple times per frame.
fn advance_physics(
    fixed_time: Res<Time>,
    mut query: Query<(
        &mut PhysicalTransform,
        &mut PreviousPhysicalTransform,
        &Velocity,
    )>,
) {
    for (mut current_physical_transform, mut previous_physical_transform, velocity) in
        query.iter_mut()
    {
        previous_physical_transform.0 = current_physical_transform.0;
        current_physical_transform.translation += velocity.0 * fixed_time.delta_seconds();
    }
}
