use crate::game::Game;

mod game;

pub fn main() { 
    let mut game = Game::default();
    game.initialize();
    game.run();
}
