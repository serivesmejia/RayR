use sdl2::keyboard::Keycode;

use crate::player::Player;

pub struct Game {

    pub player: Player

}

impl Game {

    pub fn new() -> Game {

        Game {
            player: Player::new()
        }

    }

    pub fn update(&mut self) {
    
    }

    pub fn key_pressed(&mut self, key_code: Keycode) {


    }

    pub fn key_released(&mut self, key_code: Keycode) {


    }

}