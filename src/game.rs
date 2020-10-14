use crate::player::Player;

pub struct Game {

    player: Player

}

impl Game {

    pub fn new() -> Game {

        Game {
            player: Player::new()
        }

    }

}