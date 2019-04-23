use crate::components::{paddle::PaddleSide, velocity::Velocity};
use amethyst::{
    ecs::{Read, ReadStorage, System, WriteStorage},
    input::InputHandler,
};

const PADDLE_SPEED: f32 = 75.0;

pub struct PongInputSystem;

impl<'s> System<'s> for PongInputSystem {
    type SystemData = (
        ReadStorage<'s, PaddleSide>,
        WriteStorage<'s, Velocity>,
        Read<'s, InputHandler<String, String>>,
    );

    fn run(&mut self, (paddles, mut velocities, input): Self::SystemData) {
        use amethyst::ecs::Join;

        for (paddle, velocity) in (&paddles, &mut velocities).join() {
            let movement: Option<f64> = match paddle {
                PaddleSide::Left => input.axis_value("left_paddle"),
                PaddleSide::Right => input.axis_value("right_paddle"),
            };

            if let Some(movement_amount) = movement {
                velocity.y = movement_amount as f32 * PADDLE_SPEED;
            } else {
                velocity.y = 0.0;
            }
        }
    }
}
