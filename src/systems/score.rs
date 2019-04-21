use crate::{
    components::{ball::Ball, collider::Collider, velocity::Velocity},
    states::pong_state::{ARENA_HEIGHT, ARENA_WIDTH},
};
use amethyst::{
    core::transform::Transform,
    ecs::{Join, ReadStorage, System, WriteStorage},
};

use rand::Rng;

pub struct ScoreSystem;

impl<'s> System<'s> for ScoreSystem {
    type SystemData = (
        ReadStorage<'s, Ball>,
        ReadStorage<'s, Collider>,
        WriteStorage<'s, Velocity>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, (_, colliders, mut velocities, mut transforms): Self::SystemData) {
        for (collider, velocity, transform) in (&colliders, &mut velocities, &mut transforms).join()
        {
            let transform_pos = transform.translation();

            if let Collider::Circle(radius) = collider {
                if transform_pos.x + radius > ARENA_WIDTH || transform_pos.x <= 0. {
                    transform.set_translation_x(ARENA_WIDTH / 2.);
                    transform.set_translation_y(ARENA_HEIGHT / 2.);

                    let mut rng = rand::thread_rng();
                    let invert_x = rng.gen_range(0, 2);
                    let invert_y = rng.gen_range(0, 2);

                    if invert_x == 0 {
                        velocity.x = -velocity.x;
                    }

                    if invert_y == 0 {
                        velocity.y = -velocity.y;
                    }
                }
            }
        }
    }
}
