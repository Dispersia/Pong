use crate::{
    archetypes::{ball::BallArchetype, camera::CameraArchetype, paddle::PaddleArchetype},
    components::{paddle::PaddleSide, team::Team},
    resources::spritesheet,
};
use amethyst::prelude::*;

pub const ARENA_HEIGHT: f32 = 100.0;
pub const ARENA_WIDTH: f32 = 100.0;

pub struct PongState;

impl SimpleState for PongState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        world.register::<Team>();

        let sprite_sheet_handle = spritesheet::load_sprite_sheet(world);

        PaddleArchetype::new(world, &sprite_sheet_handle, PaddleSide::Left);
        PaddleArchetype::new(world, &sprite_sheet_handle, PaddleSide::Right);

        BallArchetype::new(world, &sprite_sheet_handle);

        CameraArchetype::new(world);
    }
}
