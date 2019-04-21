use crate::{
    components::{ball::Ball, velocity::Velocity},
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
        WriteStorage<'s, Velocity>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, (balls, mut velocities, mut transforms): Self::SystemData) {
        for (ball, velocity, transform) in (&balls, &mut velocities, &mut transforms).join() {
            let transform_pos = transform.translation();

            if transform_pos.x + ball.radius > ARENA_WIDTH
                || transform_pos.x <= 0.
            {
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
