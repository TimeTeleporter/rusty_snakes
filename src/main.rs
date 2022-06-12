#![allow(dead_code)]
#![allow(unused_variables)]

pub const WINNING_SQUARE: i32 = 100;
pub const BOARD_SIZE: usize = WINNING_SQUARE as usize + 13;
pub const MAX_TURNS: i32 = 136;

use clap::Parser;
use modules::Game;

/// A simple snakes-and-ladders that plays itself
#[derive(Parser, Debug)]
#[clap(author, about)]
pub struct Args {
    /// Number of players
    #[clap(short, long, default_value_t = 1)]
    playercount: usize,

    /// Number of games to play
    #[clap(short, long, default_value_t = 1)]
    games: u32,

    /// For a more imersive experience
    #[clap(short, long)]
    verbose: bool,

}

mod modules;

fn main() {
    let args = Args::parse();

    let games = args.games;

    let mut game: Game = Game::new(args);

    for _ in 0..games {
        game.play_game();
    }

    game.print_players(games);
}