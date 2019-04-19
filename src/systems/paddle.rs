use crate::components::paddle;
use amethyst::{
    ecs::prelude::{ReadStorage, System, Read},
    input::InputHandler,
};

pub struct PaddleSystem;

impl<'s> System<'s> for PaddleSystem {
    type SystemData = (
        ReadStorage<'s, paddle::Paddle>,
        Read<'s, InputHandler<String, String>>,
    );

    fn run(&mut self, _paddles: Self::SystemData) {}
}