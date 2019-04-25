use crate::{
    archetypes::{ball::BallArchetype, camera::CameraArchetype, paddle::PaddleArchetype},
    components::paddle::PaddleSide,
    resources::spritesheet,
};
use amethyst::{
    prelude::*,
    renderer::{DebugLines, DebugLinesParams},
};

pub const ARENA_HEIGHT: f32 = 100.0;
pub const ARENA_WIDTH: f32 = 100.0;

pub struct PongState;

impl SimpleState for PongState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        #[cfg(feature = "draw_debug")]
        self.add_debug_lines(world);

        let sprite_sheet_handle = spritesheet::load_sprite_sheet(world);

        PaddleArchetype::new(world, &sprite_sheet_handle, PaddleSide::Left);
        PaddleArchetype::new(world, &sprite_sheet_handle, PaddleSide::Right);

        BallArchetype::new(world, &sprite_sheet_handle);

        CameraArchetype::new(world);
    }
}

impl PongState {
    /// This function will be called to add depdencies only when draw_debug
    /// feature is enabled
    #[allow(dead_code)]
    fn add_debug_lines(&self, world: &mut World) {
        world.add_resource(DebugLines::new().with_capacity(500));
        world.add_resource(DebugLinesParams { line_width: 4.0 });
    }
}
