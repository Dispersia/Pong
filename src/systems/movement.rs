use crate::components::velocity::Velocity;

use amethyst::{
    core::{timing::Time, transform::Transform},
    ecs::{ReadStorage, System, WriteStorage, Read},
};

pub struct MovementSystem;

impl<'s> System<'s> for MovementSystem {
    type SystemData = (
        ReadStorage<'s, Velocity>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(&mut self, _data: Self::SystemData) {}
}