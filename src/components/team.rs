use amethyst::ecs::{Component, DenseVecStorage};

pub struct Team {
    pub id: u8,
    pub points: u32,
}

impl Component for Team {
    type Storage = DenseVecStorage<Self>;
}
