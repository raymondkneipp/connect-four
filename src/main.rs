use clap::Parser;

mod game;
use game::{Game, Player};

#[derive(Parser)]
pub struct Args {
    #[arg(short, long, value_parser, num_args = 2..=26, value_delimiter = ' ')]
    pub players: Vec<String>,

    #[arg(short, long, default_value = "5")]
    pub rows: usize,

    #[arg(short, long, default_value = "7")]
    pub cols: usize,

    #[arg(short, long, default_value = "4")]
    pub tokens_to_win: usize,
}

fn main() {
    let args = Args::parse();

    let players = args
        .players
        .iter()
        .map(|name| Player::new(name))
        .collect::<Vec<Player>>();

    let mut game = Game::new(args.rows, args.cols, args.tokens_to_win, players);

    game.start();
}
