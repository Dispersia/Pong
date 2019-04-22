use amethyst::ecs::{Component, VecStorage};

#[derive(PartialEq, Eq)]
pub enum PaddleSide {
    Left,
    Right,
}

impl Component for PaddleSide {
    type Storage = VecStorage<Self>;
}