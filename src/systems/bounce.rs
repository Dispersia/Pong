use crate::components::{
    ball::Ball,
    velocity::Velocity,
    collider::Collider,
};
use crate::states::pong_state::ARENA_HEIGHT;
use amethyst::{
    core::transform::Transform,
    ecs::{Join, ReadStorage, System, WriteStorage},
};

pub struct BounceSystem;

impl<'s> System<'s> for BounceSystem {
    type SystemData = (
        ReadStorage<'s, Ball>,
        ReadStorage<'s, Collider>,
        ReadStorage<'s, Transform>,
        WriteStorage<'s, Velocity>,
    );

    fn run(&mut self, (_, colliders, transforms, mut velocities): Self::SystemData) {
        for (collider, transform, velocity) in (&colliders, &transforms, &mut velocities).join() {
            let transform = transform.translation();

            if let Collider::Circle(radius) = collider {
                if transform.y + radius > ARENA_HEIGHT
                    || transform.y + radius <= 0.0 {
                    velocity.y = -velocity.y;
                }
            }
        }
    }
}
