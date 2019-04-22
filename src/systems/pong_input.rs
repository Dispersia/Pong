use crate::components::paddle;
use amethyst::{
    ecs::prelude::{ReadStorage, System, Read},
    input::InputHandler,
};

pub struct PongInputSystem;

impl<'s> System<'s> for PongInputSystem {
    type SystemData = (
        ReadStorage<'s, paddle::PaddleSide>,
        Read<'s, InputHandler<String, String>>,
    );

    fn run(&mut self, _paddles: Self::SystemData) {}
}