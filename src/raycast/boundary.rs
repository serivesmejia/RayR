use crate::util::math::Vec2;

struct Boundary {
    a: Vec2,
    b: Vec2
}

impl Boundary {

    pub fn new(x1: f32, y1: f32, x2: f32, y2: f32) -> Boundary {
        Boundary {
            a: Vec2::new(x1, y1),
            b: Vec2::new(x2, y2)
        }
    }

}