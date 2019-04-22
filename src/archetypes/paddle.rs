use crate::components::{collider::Collider, paddle::PaddleSide, team::Team};
use crate::states::pong_state::{ARENA_HEIGHT, ARENA_WIDTH};
use amethyst::core::transform::Transform;
use amethyst::ecs::{Entity, World};
use amethyst::prelude::*;
use amethyst::renderer::{SpriteRender, SpriteSheetHandle};

const PADDLE_HEIGHT: f32 = 16.0;
const PADDLE_WIDTH: f32 = 4.0;

pub struct PaddleArchetype;

impl PaddleArchetype {
    pub fn new(
        world: &mut World,
        sprite_sheet_handle: &SpriteSheetHandle,
        side: PaddleSide,
    ) -> Entity {
        let mut transform = Transform::default();

        let is_right = side == PaddleSide::Right;
        let team_id: u8 = if !is_right { 0 } else { 1 };

        let x = if is_right {
            ARENA_WIDTH - PADDLE_WIDTH * 0.5
        } else {
            PADDLE_WIDTH * 0.5
        };

        let y = ARENA_HEIGHT / 2.0;

        transform.set_translation_xyz(x, y, 0.0);

        let sprite_render = SpriteRender {
            sprite_sheet: (*sprite_sheet_handle).clone(),
            sprite_number: 0,
        };

        world
            .create_entity()
            .with(sprite_render.clone())
            .with(side)
            .with(Team {
                id: team_id,
                points: 0,
            })
            .with(Collider::Rectangle(PADDLE_WIDTH, PADDLE_HEIGHT))
            .with(transform)
            .build()
    }
}
