use crate::components::velocity::Velocity;

use amethyst::{
    core::{timing::Time, transform::Transform},
    ecs::{Read, ReadStorage, System, WriteStorage},
};

pub struct MovementSystem;

impl<'s> System<'s> for MovementSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Velocity>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut transforms, velocities, time): Self::SystemData) {
        use amethyst::ecs::Join;

        for (transform, velocity) in (&mut transforms, &velocities).join() {
            if velocity.x != 0.0 {
                transform.prepend_translation_x(velocity.x * time.delta_seconds());
            }

            if velocity.y != 0.0 {
                transform.prepend_translation_y(velocity.y * time.delta_seconds());
            }
        }
    }
}
