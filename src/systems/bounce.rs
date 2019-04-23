use crate::components::{ball::Ball, collider::Collider, paddle::PaddleSide, velocity::Velocity};
use crate::states::pong_state::ARENA_HEIGHT;
use amethyst::{
    core::transform::Transform,
    ecs::{Join, ReadStorage, System, WriteStorage},
};

pub struct BounceSystem;

impl<'s> System<'s> for BounceSystem {
    type SystemData = (
        ReadStorage<'s, Ball>,
        ReadStorage<'s, PaddleSide>,
        ReadStorage<'s, Collider>,
        ReadStorage<'s, Transform>,
        WriteStorage<'s, Velocity>,
    );

    fn run(&mut self, (balls, paddles, colliders, transforms, mut velocities): Self::SystemData) {
        for (_, collider, transform, velocity) in
            (&balls, &colliders, &transforms, &mut velocities).join()
        {
            let transform_pos = transform.translation();

            if let Collider::Square(length) = collider {
                if transform_pos.y + length > ARENA_HEIGHT || transform_pos.y <= 0.0 {
                    velocity.y = -velocity.y;
                    continue;
                }

                for (_, paddle_collider, paddle_transform) in
                    (&paddles, &colliders, &transforms).join()
                {
                    if collider.collides_with(transform, paddle_collider, paddle_transform) {
                        velocity.x = -velocity.x;
                        break;
                    }
                }
            }
        }
    }
}
