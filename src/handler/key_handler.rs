use std::collections::HashSet;

use sdl2::keyboard::Keycode;

use crate::game::Game;

pub struct KeyHandler {

    prev_keys: HashSet<Keycode>,

}

impl KeyHandler {

    pub fn new() -> KeyHandler {

        KeyHandler {
            prev_keys: HashSet::new()
        }

    }

    pub fn handle_keys(&mut self, keys: HashSet<Keycode>, game: &mut Game) {

        let new_keys = &keys - &self.prev_keys;
        let old_keys = &self.prev_keys - &keys;
    
        for key in new_keys {
            game.key_pressed(key);
            game.player.key_pressed(key)
        }
    
        for key in old_keys {
            game.key_released(key);
            game.player.key_pressed(key)
        }
    
        self.prev_keys = keys;

    }

}