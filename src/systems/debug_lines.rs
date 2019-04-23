use crate::components::collider::Collider;
use amethyst::{
    core::{math::Point3, transform::Transform},
    ecs::{Join, ReadStorage, System, Write},
    renderer::{DebugLines, Rgba},
};

pub struct PongDebugLinesSystem;

impl<'s> System<'s> for PongDebugLinesSystem {
    type SystemData = (
        Write<'s, DebugLines>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Collider>,
    );

    fn run(&mut self, (mut debug_line_resource, transforms, colliders): Self::SystemData) {
        for (transform, collider) in (&transforms, &colliders).join() {
            self.print_debug_line_1(transform, collider, &mut debug_line_resource);
            self.print_debug_line_2(transform, collider, &mut debug_line_resource);
            self.print_debug_line_3(transform, collider, &mut debug_line_resource);
            self.print_debug_line_4(transform, collider, &mut debug_line_resource);
        }
    }
}

impl PongDebugLinesSystem {
    fn print_debug_line_1(
        &self,
        transform: &Transform,
        collider: &Collider,
        debug_line_resource: &mut DebugLines,
    ) {
        let transform_pos = transform.translation();

        let height = match *collider {
            Collider::Square(length) => length,
            Collider::Rectangle(height, _) => height,
        };

        let transform_start = Point3::new(transform_pos.x, transform_pos.y, 0.0);
        let transform_end = Point3::new(transform_pos.x, transform_pos.y + height, 0.0);
        let color_white = Rgba::WHITE;

        debug_line_resource.draw_line(transform_start, transform_end, color_white);
    }

    fn print_debug_line_2(
        &self,
        transform: &Transform,
        collider: &Collider,
        debug_line_resource: &mut DebugLines,
    ) {
        let transform_pos = transform.translation();

        let width = match *collider {
            Collider::Square(length) => length,
            Collider::Rectangle(_, width) => width,
        };

        let transform_start = Point3::new(transform_pos.x, transform_pos.y, 0.0);
        let transform_end = Point3::new(transform_pos.x + width, transform_pos.y, 0.0);
        let color_white = Rgba::WHITE;

        debug_line_resource.draw_line(transform_start, transform_end, color_white);
    }

    fn print_debug_line_3(
        &self,
        transform: &Transform,
        collider: &Collider,
        debug_line_resource: &mut DebugLines,
    ) {
        let transform_pos = transform.translation();

        let (height, width) = match *collider {
            Collider::Square(length) => (length, length),
            Collider::Rectangle(height, width) => (height, width),
        };

        let transform_start = Point3::new(transform_pos.x + width, transform_pos.y, 0.0);
        let transform_end = Point3::new(transform_pos.x + width, transform_pos.y + height, 0.0);
        let color_white = Rgba::WHITE;

        debug_line_resource.draw_line(transform_start, transform_end, color_white);
    }

    fn print_debug_line_4(
        &self,
        transform: &Transform,
        collider: &Collider,
        debug_line_resource: &mut DebugLines,
    ) {
        let transform_pos = transform.translation();

        let (height, width) = match *collider {
            Collider::Square(length) => (length, length),
            Collider::Rectangle(height, width) => (height, width),
        };

        let transform_start = Point3::new(transform_pos.x, transform_pos.y + height, 0.0);
        let transform_end = Point3::new(transform_pos.x + width, transform_pos.y + height, 0.0);
        let color_white = Rgba::WHITE;

        debug_line_resource.draw_line(transform_start, transform_end, color_white);
    }
}
