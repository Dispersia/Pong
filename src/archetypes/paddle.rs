use crate::components::paddle;
use crate::states::pong_state::{ARENA_HEIGHT, ARENA_WIDTH};
use amethyst::core::transform::Transform;
use amethyst::ecs::{Entity, World};
use amethyst::prelude::*;
use amethyst::renderer::{SpriteRender, SpriteSheetHandle};

pub struct PaddleArchetype;

impl PaddleArchetype {
    pub fn new(world: &mut World, sprite_sheet: &SpriteSheetHandle, side: paddle::Side) -> Entity {
        let mut transform = Transform::default();

        let x: f32;

        if side == paddle::Side::Left {
            x = 5.;
        } else {
            x = 4.;
        }

        let sprite_render = SpriteRender {
            sprite_sheet: sprite_sheet.clone(),
            sprite_number: 0,
        };

        transform.set_translation_xyz(x, ARENA_HEIGHT / 2., 0.);

        world
            .create_entity()
            .with(sprite_render)
            .with(paddle::Paddle::new(side))
            .with(transform)
            .build()
    }
}
