use crate::components::paddle;
use crate::states::pong_state::{ARENA_HEIGHT, ARENA_WIDTH};
use amethyst::core::transform::Transform;
use amethyst::ecs::{Entity, World};
use amethyst::prelude::*;
use amethyst::renderer::{Flipped, SpriteRender, SpriteSheetHandle};

pub struct PaddleArchetype;

impl PaddleArchetype {
    pub fn new(
        world: &mut World,
        sprite_sheet_handle: &SpriteSheetHandle,
        side: paddle::Side,
    ) -> Entity {
        use crate::components::paddle::{PADDLE_HEIGHT, PADDLE_WIDTH};

        let mut transform = Transform::default();

        let is_right = side == paddle::Side::Right;

        let x = if is_right {
            ARENA_WIDTH - PADDLE_WIDTH * 0.5
        } else {
            PADDLE_WIDTH * 0.5
        };

        let y = (ARENA_HEIGHT - PADDLE_HEIGHT) / 2.0;

        transform.set_translation_xyz(x, y, 0.0);

        let sprite_render = SpriteRender {
            sprite_sheet: (*sprite_sheet_handle).clone(),
            sprite_number: 0,
        };

        let mut entity = world
            .create_entity()
            .with(sprite_render.clone())
            .with(paddle::Paddle::new(side))
            .with(transform);

        if is_right {
            entity = entity.with(Flipped::Horizontal);
        }

        entity.build()
    }
}
