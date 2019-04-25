use amethyst::ecs::{Component, VecStorage};

pub struct Team {
    pub id: u8,
    pub points: u32,
}

impl Component for Team {
    type Storage = VecStorage<Self>;
}
