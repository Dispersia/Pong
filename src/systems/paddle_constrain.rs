use crate::{
    components::{collider::Collider, paddle::PaddleSide},
    states::pong_state::ARENA_HEIGHT,
};
use amethyst::{
    core::transform::Transform,
    ecs::{Join, ReadStorage, System, WriteStorage},
};

pub struct PaddleConstrainSystem;

impl<'s> System<'s> for PaddleConstrainSystem {
    type SystemData = (
        ReadStorage<'s, PaddleSide>,
        ReadStorage<'s, Collider>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, (paddle, collider, mut transform): Self::SystemData) {
        for (_, collider, transform) in (&paddle, &collider, &mut transform).join() {
            let transform_pos = transform.translation();

            let height = match collider {
                Collider::Square(length) => length,
                Collider::Rectangle(height, _) => height,
            };

            if transform_pos.y + height >= ARENA_HEIGHT {
                transform.set_translation_y(ARENA_HEIGHT - height);
            } else if transform_pos.y <= 0.0 {
                transform.set_translation_y(0.0);
            }
        }
    }
}
