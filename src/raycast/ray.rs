use crate::util::math::Vec2;
use crate::raycast::boundary::Boundary;

struct Ray {

    pos: Vec2,
    dir: Vec2,

    angle: f32

}

impl Ray {

    pub fn new(pos: Vec2, angle: f32) -> Ray {
        Ray {
            pos: pos,
            dir: Vec2::from_angle(angle),
            angle: angle
        }
    }

    pub fn set_angle(&mut self, angle: f32) {
        self.dir = Vec2::from_angle(angle);
        self.angle = angle
    }

    pub fn look_at(&mut self, x: f32, y: f32) {
        self.dir.set(x - self.pos.x, y - self.pos.y);
        self.dir.normalize()
    }

    pub fn cast(&self, bd: Boundary) -> Vec2 {
        
        let x1 = bd.a.x;
        let y1 = bd.a.y;
        let x2 = bd.b.x;
        let y2 = bd.b.y;

        let x3 = self.pos.x;
        let y3 = self.pos.y;
        let x4 = self.pos.x + self.dir.x;
        let y4 = self.pos.y + self.dir.y;

        let den = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);
      
        if den == 0 {
            return Vec2::new(0.0, 0.0)
        }

        let t = ((x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4)) / den;
        let u = -((x1 - x2) * (y1 - y3) - (y1 - y2) * (x1 - x3)) / den;

        if t > 0 && t < 1 && u > 0 {
            let pt = Vec2::new(0.0, 0.0);
            pt.x = x1 + t * (x2 - x1);
            pt.y = y1 + t * (y2 - y1);
            return pt;
        } else {
            return Vec2::new(0.0, 0.0)
        }

    }

}