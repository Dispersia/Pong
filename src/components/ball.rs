use amethyst::ecs::prelude::{Component, DenseVecStorage};

pub struct Ball {
    pub radius: f32,
}

impl Component for Ball {
    type Storage = DenseVecStorage<Self>;
}
