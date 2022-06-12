#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use clap::Parser;

/// A simple snakes-and-ladders that plays itself
#[derive(Parser, Default, Debug)]
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

const WINNING_SQUARE: i32 = 100;
const BOARD_SIZE: usize = WINNING_SQUARE as usize + 13;
const MAX_TURNS: i32 = 136;

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

struct SnakesAndLadders {
    verbose: bool,
    board: [i32; BOARD_SIZE],
    players: Vec<Player>,
    playercount: usize,
}

impl SnakesAndLadders {
    fn new(verbose: bool, playercount: usize) -> SnakesAndLadders {

        let board = SnakesAndLadders::init_board();

        if verbose { println!("Board is initialized"); }

        let mut players: Vec<Player> = vec!(Player::new(); playercount);

        if verbose { println!("Players are initialized"); }

        SnakesAndLadders { verbose, board, players, playercount }
    }

    fn dice(faces: u32) -> i32 {
        (rand::random::<u32>() % faces) as i32
    }
    
    fn init_board() -> [i32; BOARD_SIZE] {
        let mut board: [i32; BOARD_SIZE] = [0; BOARD_SIZE];
    
        SnakesAndLadders::ladder(&mut board, 1, 38);
        SnakesAndLadders::ladder(&mut board, 4, 14);
        SnakesAndLadders::ladder(&mut board, 9, 31);
        SnakesAndLadders::snake(&mut board, 17, 7);
        SnakesAndLadders::ladder(&mut board, 21, 42);
        SnakesAndLadders::ladder(&mut board, 28, 84);
        SnakesAndLadders::ladder(&mut board, 51, 67);
        SnakesAndLadders::snake(&mut board, 54, 34);
        SnakesAndLadders::snake(&mut board, 62, 19);
        SnakesAndLadders::snake(&mut board, 64, 60);
        SnakesAndLadders::ladder(&mut board, 72, 91);
        SnakesAndLadders::ladder(&mut board, 80, 99);
        SnakesAndLadders::snake(&mut board, 87, 36);
        SnakesAndLadders::snake(&mut board, 93, 73);
        SnakesAndLadders::snake(&mut board, 94, 75);
        SnakesAndLadders::snake(&mut board, 98, 79);
    
        board
    }
    
    fn ladder(board: &mut [i32; BOARD_SIZE], pos: i32, jump: i32) {
        board[pos as usize] = jump-pos;
    }
    
    fn snake(board: &mut [i32; BOARD_SIZE], pos: i32, fall: i32) {
        board[pos as usize] = fall-pos;
    }
    
    fn play_game(&self) {
        let playercount = self.players.len();
        let mut pos: Vec<i32> = vec!(0; playercount);
    
        for turn in 0..MAX_TURNS {
            for i in 0..playercount {
                print!("Player {} moves to", i);
                pos[i] = SnakesAndLadders::make_turn(pos[i], &self.board);
                println!("{}", pos[i]);
    
                if pos[i] >= WINNING_SQUARE {
                    self.players[i].has_won(turn);
                    return;
                }
    
                for j in 0..playercount {
                    if pos[i] == pos[j] && i != j {
                        self.players[j].got_hit();
                        println!("Oh no, Player {} got hit!", j);
                        pos[j] = 0;
                    }
                }
            }
        }
    }
    
    fn make_turn(pos: i32, board: &[i32; BOARD_SIZE]) -> i32 {
        let result = pos + SnakesAndLadders::dice(6) + SnakesAndLadders::dice(6);
        result + board[result as usize]
    }
}

fn main() {
    let args = Args::parse();

    if args.verbose { println!("{:?}", args);}

    let game: SnakesAndLadders = SnakesAndLadders::new(args.verbose, args.playercount);

    for _ in 0..args.games {
        game.play_game(&mut players, &board);
    }

    for index in 0..players.len() {
        players[index].print(index, args.games);
    }
}
