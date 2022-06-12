#![allow(dead_code)]
#![allow(unused_variables)]

const WINNING_SQUARE: i32 = 100;
pub const BOARD_SIZE: usize = WINNING_SQUARE as usize + 13;
const MAX_TURNS: i32 = 136;

use clap::Parser;

/// A simple snakes-and-ladders that plays itself
#[derive(Parser, Debug)]
#[clap(author, about)]
struct Args {
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

#[derive(Clone,Copy)]
struct Player {
    wins: i32,
    mean: f32,
    times_hit: i32,
}

impl Player {
    fn new() -> Player {
        Player {wins: 0, mean: 0., times_hit: 0}
    }

    fn has_won(&mut self, turns: i32) {
        let old_wins = self.wins as f32;
        self.wins += 1;
        self.mean = (old_wins * self.mean + turns as f32)/(old_wins+1.);
    }

    fn got_hit(&mut self) {
        self.times_hit += 1;
    }

    fn print(&self, player_nr: usize, games: u32) {
        println!("Player {} has won {} times ({:.2}%) with an average of {} turns. He got hit {} times.", 
                player_nr, 
                self.wins, 
                (self.wins as f32) / (games as f32) * 100., 
                self.mean, 
                self.times_hit
            );
    }
}

struct Game {
    args: Args,
    board: [i32; BOARD_SIZE],
    players: Vec<Player>,
}

fn dice(faces: u32) -> i32 {
    (rand::random::<u32>() % faces) as i32
}

impl Game {
    fn new(args: Args) -> Game {
        let board: [i32; BOARD_SIZE] = [0; BOARD_SIZE];

        if args.verbose { println!("Board initialized") }

        let players: Vec<Player> = vec!(Player::new(); args.playercount);

        if args.verbose { println!("Players initialized") }

        Game {args, board, players }
    }

    fn play_game(&mut self) {
        let playercount = self.args.playercount;
        let verbose = self.args.verbose;
        let mut pos: Vec<i32> = vec!(0; playercount);

        for turn in 0..MAX_TURNS {
            for i in 0..playercount {
                pos[i] = Game::make_turn(pos[i], self.board);
                if verbose { println!("Player {} moves to {}", i, pos[i]); }

                if pos[i] >= WINNING_SQUARE {
                    self.players[i].has_won(turn);
                    return;
                }
    
                for j in 0..playercount {
                    if pos[i] == pos[j] && i != j {
                        self.players[j].got_hit();
                        if verbose { println!("Oh no, Player {} got hit!", j); }
                        pos[j] = 0;
                    }
                }
            }
        }
    }

    fn make_turn(start: i32, board: [i32; BOARD_SIZE]) -> i32 {
        let result = start + dice(6) + dice(6);
        result + board[result as usize]
    }

    fn print_players(&self, gamecounter: u32) {
        for index in 0..self.args.playercount {
            self.players[index].print(index, gamecounter);
        }
    }
}

mod board {
    use crate::BOARD_SIZE;

    fn init_board() -> [i32; BOARD_SIZE] {
        let mut board: [i32; BOARD_SIZE] = [0; BOARD_SIZE];
    
        ladder(&mut board, 1, 38);
        ladder(&mut board, 4, 14);
        ladder(&mut board, 9, 31);
        snake(&mut board, 17, 7);
        ladder(&mut board, 21, 42);
        ladder(&mut board, 28, 84);
        ladder(&mut board, 51, 67);
        snake(&mut board, 54, 34);
        snake(&mut board, 62, 19);
        snake(&mut board, 64, 60);
        ladder(&mut board, 72, 91);
        ladder(&mut board, 80, 99);
        snake(&mut board, 87, 36);
        snake(&mut board, 93, 73);
        snake(&mut board, 94, 75);
        snake(&mut board, 98, 79);
    
        board
    }
    
    fn ladder(board: &mut [i32; BOARD_SIZE], pos: i32, jump: i32) {
        board[pos as usize] = jump-pos;
    }
    
    fn snake(board: &mut [i32; BOARD_SIZE], pos: i32, fall: i32) {
        board[pos as usize] = fall-pos;
    }
}

fn main() {
    let args = Args::parse();

    let games = args.games;

    let mut game: Game = Game::new(args);

    for _ in 0..games {
        game.play_game();
    }

    game.print_players(games);
}