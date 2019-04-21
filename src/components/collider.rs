use amethyst::ecs::{Component, DenseVecStorage};

pub enum Collider {
    Circle(f32),
    Rectangle(f32, f32),
}

impl Component for Collider {
    type Storage = DenseVecStorage<Self>;
}
