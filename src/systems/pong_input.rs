use crate::components::paddle::PaddleSide;
use amethyst::{
    ecs::{Read, ReadStorage, System},
    input::InputHandler,
};

pub struct PongInputSystem;

impl<'s> System<'s> for PongInputSystem {
    type SystemData = (
        ReadStorage<'s, PaddleSide>,
        Read<'s, InputHandler<String, String>>,
    );

    fn run(&mut self, (paddles, input): Self::SystemData) {
        use amethyst::ecs::Join;

        for paddle in (&paddles).join() {
            let movement = match paddle {
                PaddleSide::Left => input.axis_value("left_paddle"),
                PaddleSide::Right => input.axis_value("right_paddle"),
            };

            if let Some(movement_amount) = movement {
                if movement_amount != 0.0 {
                    let side_name = match paddle {
                        PaddleSide::Left => "left",
                        PaddleSide::Right => "right",
                    };

                    println!("Side {:?} moving {}", side_name, movement_amount);
                }
            }
        }
    }
}
