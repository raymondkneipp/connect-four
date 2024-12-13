mod game;
use game::{Game, Player};

fn main() {
    let players = vec![Player::new("Raymond", 'x'), Player::new("Anaïs", 'o')];

    let mut game = Game::new(5, 5, 4, players);

    game.start();
}
