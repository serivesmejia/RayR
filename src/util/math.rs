pub struct Vec2 {
    
    pub x: f32,
    pub y: f32

}

impl Vec2 {

    pub fn new(x_pos: f32, y_pos: f32) -> Vec2 {

        Vec2 {
            x: x_pos,
            y: y_pos
        }

    }

    pub fn plus(&mut self, other: Vec2) {

        self.x += other.x;
        self.y += other.y;

    }

    pub fn minus(&mut self, other: Vec2) {

        self.x -= other.x;
        self.y -= other.y;

    }

    pub fn times(&mut self, other: Vec2) {

        self.x *= other.x;
        self.y *= other.y;

    }

    pub fn div(&mut self, other: Vec2) {

        self.x /= other.x;
        self.y /= other.y;

    }

    pub fn clone(&self) -> Vec2 {

        Vec2::new(self.x, self.y)

    }

}