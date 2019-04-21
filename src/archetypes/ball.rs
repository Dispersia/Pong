use crate::components::{ball::Ball, velocity::Velocity};
use crate::states::pong_state::{ARENA_HEIGHT, ARENA_WIDTH};
use amethyst::core::transform::Transform;
use amethyst::ecs::{Entity, World};
use amethyst::prelude::*;
use amethyst::renderer::{SpriteRender, SpriteSheetHandle};

const BALL_VELOCITY_X: f32 = 75.0;
const BALL_VELOCITY_Y: f32 = 50.0;
const BALL_RADIUS: f32 = 2.0;

pub struct BallArchetype;

impl BallArchetype {
    pub fn new(world: &mut World, sprite_sheet_handle: &SpriteSheetHandle) -> Entity {
        let mut transform = Transform::default();
        let x = ARENA_WIDTH / 2. - BALL_RADIUS / 2.;
        let y = ARENA_HEIGHT / 2. - BALL_RADIUS / 2.;

        transform.set_translation_xyz(x, y, 0.);

        let sprite_sheet = SpriteRender {
            sprite_sheet: (*sprite_sheet_handle).clone(),
            sprite_number: 1,
        };

        world
            .create_entity()
            .with(sprite_sheet)
            .with(Ball {
                radius: BALL_RADIUS,
            })
            .with(Velocity {
                x: BALL_VELOCITY_X,
                y: BALL_VELOCITY_Y,
            })
            .with(transform)
            .build()
    }
}
