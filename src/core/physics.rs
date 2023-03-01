use bevy::prelude::*;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(apply_velocity);
    }
}

#[derive(Component)]
pub struct Velocity(pub Vec2);

fn apply_velocity(mut qry: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in qry.iter_mut() {
        transform.translation += velocity.0.extend(0.);
    }
}
