use amethyst::ecs::{World, Entity};
use amethyst::prelude::*;
use crate::components::ball::Ball;

pub const BALL_VELOCITY_X: f32 = 75.0;
pub const BALL_VELOCITY_Y: f32 = 50.0;
pub const BALL_RADIUS: f32 = 2.0;

pub struct BallArchetype;

impl BallArchetype {
    pub fn new(world: &mut World) -> Entity {
        world
            .create_entity()
            .with(Ball {
                velocity: [BALL_VELOCITY_X, BALL_VELOCITY_Y],
                radius: BALL_RADIUS
            })
            .build()
    }
}