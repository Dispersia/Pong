use amethyst::{
    core::transform::Transform,
    ecs::{Component, DenseVecStorage},
};

pub enum Collider {
    Circle(f32),
    Rectangle(f32, f32),
}

impl Component for Collider {
    type Storage = DenseVecStorage<Self>;
}

impl Collider {
    // This will treat both as rectangles for now.
    pub fn collides_with(
        &self,
        transform: &Transform,
        other_collider: &Collider,
        other_transform: &Transform,
    ) -> bool {
        let (height, width) = match *self {
            Collider::Circle(radius) => (radius, radius),
            Collider::Rectangle(height, width) => (height, width),
        };

        let (other_height, other_width) = match *other_collider {
            Collider::Circle(radius) => (radius, radius),
            Collider::Rectangle(height, width) => (height, width),
        };

        let transform = transform.translation();
        let other_transform = other_transform.translation();

        self.point_in_rect(
            transform.x,
            transform.y,
            other_transform.x - width,
            other_transform.y - height,
            other_transform.x + width + other_width,
            other_transform.y + height + other_height,
        )
    }

    fn point_in_rect(&self, x: f32, y: f32, left: f32, bottom: f32, right: f32, top: f32) -> bool {
        x >= left && x <= right && y >= bottom && y <= top
    }
}
