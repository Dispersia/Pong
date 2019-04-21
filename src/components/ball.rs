use amethyst::ecs::prelude::{Component, NullStorage};

#[derive(Default)]
pub struct Ball;

impl Component for Ball {
    type Storage = NullStorage<Self>;
}
