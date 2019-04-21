use amethyst::ecs::{Component, DenseVecStorage};

#[derive(PartialEq, Eq)]
pub enum Side {
    Left,
    Right,
}


pub struct Paddle {
    pub side: Side,
}

impl Component for Paddle {
    type Storage = DenseVecStorage<Self>;
}