use amethyst::ecs::{System, Join, ReadStorage, WriteStorage};
use crate::components::{
    ball::Ball,
    velocity::Velocity,
};

pub struct BounceSystem;

impl<'s> System<'s> for BounceSystem {
    type SystemData = (
        ReadStorage<'s, Ball>,
        WriteStorage<'s, Velocity>
    );

    fn run(&mut self, (balls, mut velocities): Self::SystemData) {
        for (ball, velocity) in (&balls, &mut velocities).join() {
            //if ball.x
        }
    }
}