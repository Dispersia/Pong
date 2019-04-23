use crate::{
    components::{
        ball::Ball, collider::Collider, paddle::PaddleSide, team::Team, velocity::Velocity,
    },
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
        ReadStorage<'s, PaddleSide>,
        WriteStorage<'s, Team>,
        WriteStorage<'s, Velocity>,
        WriteStorage<'s, Transform>,
    );

    fn run(
        &mut self,
        (balls, colliders, paddles, mut teams, mut velocities, mut transforms): Self::SystemData,
    ) {
        for (_, collider, velocity, transform) in
            (&balls, &colliders, &mut velocities, &mut transforms).join()
        {
            let transform_pos = transform.translation();

            if let Collider::Square(length) = collider {
                let exited_right_side = transform_pos.x + length >= ARENA_WIDTH;
                let exited_left_side = transform_pos.x <= 0.;

                if exited_left_side || exited_right_side {
                    transform.set_translation_x(ARENA_WIDTH / 2.);
                    transform.set_translation_y(ARENA_HEIGHT / 2.);

                    let mut rng = rand::thread_rng();
                    let invert_x: i32 = rng.gen_range(0, 2);
                    let invert_y: i32 = rng.gen_range(0, 2);

                    if invert_x == 0 {
                        velocity.x = -velocity.x;
                    }

                    if invert_y == 0 {
                        velocity.y = -velocity.y;
                    }

                    if exited_left_side {
                        for (_, team) in (&paddles, &mut teams).join() {
                            if team.id == 0 {
                                team.points += 1;

                                println!("Team {}: {}", team.id, team.points);
                                break;
                            }
                        }
                    }

                    if exited_right_side {
                        for (_, team) in (&paddles, &mut teams).join() {
                            if team.id == 1 {
                                team.points += 1;

                                println!("Team {}: {}", team.id, team.points);
                                break;
                            }
                        }
                    }
                }
            }
        }
    }
}
