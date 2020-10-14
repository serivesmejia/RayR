use sdl2::keyboard::Keycode;

use crate::util::math::Vec2;

pub struct Player {
    pub fov: f32,
    pub pos: Vec2
}

impl Player {

    pub fn new() -> Player {
        Player {
            fov: 65.0,
            pos: Vec2::new(0.0, 0.0)
        }
    }

    pub fn key_pressed(&mut self, key_code: Keycode) {

        println!("Pressed: {}", key_code.name())

    }

    pub fn key_released(&mut self, key_code: Keycode) {

        println!("Released: {}", key_code.name())

    }

}