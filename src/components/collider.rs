use amethyst::{
    core::transform::Transform,
    ecs::{Component, VecStorage},
};

pub enum Collider {
    Square(f32),
    Rectangle(f32, f32),
}

impl Component for Collider {
    type Storage = VecStorage<Self>;
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
            Collider::Square(length) => (length, length),
            Collider::Rectangle(height, width) => (height, width),
        };

        let (other_height, other_width) = match *other_collider {
            Collider::Square(length) => (length, length),
            Collider::Rectangle(height, width) => (height, width),
        };

        let ball_transform = transform.translation();
        let paddle_transform = other_transform.translation();

        self.point_in_rect(
            ball_transform.x,
            ball_transform.x + width,
            ball_transform.y,
            ball_transform.y + height,
            paddle_transform.x,
            paddle_transform.x + other_width,
            paddle_transform.y,
            paddle_transform.y + other_height,
        )
    }

    fn point_in_rect(
        &self,
        ball_left: f32,
        ball_right: f32,
        ball_top: f32,
        ball_bottom: f32,
        paddle_left: f32,
        paddle_right: f32,
        paddle_top: f32,
        paddle_bottom: f32,
    ) -> bool {
        ball_left < paddle_right
            && paddle_left < ball_right
            && ball_top < paddle_bottom
            && paddle_top < ball_bottom
    }
}
