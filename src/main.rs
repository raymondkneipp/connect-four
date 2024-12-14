mod game;
use game::{Game, Player};

fn main() {
    let players = vec![Player::new("Whippo", 'x'), Player::new("Ray", 'o')];

    let mut game = Game::new(5, 7, 4, players);

    game.start();
}
