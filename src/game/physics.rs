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
pub(crate) struct Velocity(pub(crate) Vec3);

/// The actual transform of the player in the physics simulation.
/// This is separate from the `Transform`, which is merely a visual
/// representation.
#[derive(Debug, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
pub(crate) struct PhysicalTransform(pub(crate) Transform);

/// Make sure that this component is always initialized
/// with the same value as the `Transform`
impl Component for PhysicalTransform {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    fn register_component_hooks(hooks: &mut ComponentHooks) {
        hooks.on_add(|mut world, entity, _component_id| {
            let rendered_transform = *world.get::<Transform>(entity).unwrap();
            let mut physical_transform = world.get_mut::<PhysicalTransform>(entity).unwrap();
            physical_transform.0 = rendered_transform;
        });
    }
}

/// Advance the physics simulation by one fixed timestep. This may run zero or
/// multiple times per frame.
///
/// Note that since this runs in `FixedUpdate`, `Res<Time>` would be
/// `Res<Time<Fixed>>` automatically. We are being explicit here for clarity.
fn advance_physics(
    fixed_time: Res<Time<Fixed>>,
    mut query: Query<(&mut PhysicalTransform, &Velocity)>,
) {
    for (mut physical_transform, velocity) in query.iter_mut() {
        physical_transform.translation += velocity.0 * fixed_time.delta_seconds();
    }
}
