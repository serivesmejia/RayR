use std::collections::HashSet;
use sdl2::keyboard::Keycode;

use crate::player::Player;

pub struct KeyHandler {

    prev_keys: HashSet<Keycode>,

}

impl KeyHandler {

    pub fn new() -> KeyHandler {

        KeyHandler {
            prev_keys: HashSet::new()
        }

    }

    pub fn handle_keys(&mut self, keys: HashSet<Keycode>, player: &mut Player) {

        let new_keys = &keys - &self.prev_keys;
        let old_keys = &self.prev_keys - &keys;
    
        for key in new_keys {
            player.key_pressed(key)
        }
    
        for key in old_keys {
            player.key_released(key)
        }
    
        self.prev_keys = keys;

    }

}