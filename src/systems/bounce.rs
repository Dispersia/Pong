use crate::components::{ball::Ball, velocity::Velocity};
use crate::states::pong_state::ARENA_HEIGHT;
use amethyst::{
    core::transform::Transform,
    ecs::{Join, ReadStorage, System, WriteStorage},
};

pub struct BounceSystem;

impl<'s> System<'s> for BounceSystem {
    type SystemData = (
        ReadStorage<'s, Ball>,
        ReadStorage<'s, Transform>,
        WriteStorage<'s, Velocity>,
    );

    fn run(&mut self, (balls, transforms, mut velocities): Self::SystemData) {
        for (ball, transform, velocity) in (&balls, &transforms, &mut velocities).join() {
            if transform.translation().y + ball.radius > ARENA_HEIGHT
                || transform.translation().y + ball.radius <= 0.
            {
                velocity.y = -velocity.y;
            }
        }
    }
}
