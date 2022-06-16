


















































/*
use crate::player::*;
use crate::*;

pub const WINNING_SQUARE: i32 = 100;
pub const BOARD_SIZE: usize = WINNING_SQUARE as usize + 14;
pub const MAX_TURNS: i32 = 136;

pub trait Playable {
    fn play_game(&mut self);
}

pub struct Game {
    args: Args,
    board: [i32; BOARD_SIZE],
    players: Vec<Player>,
}

impl Game {
    pub fn new(args: Args) -> Game {
        let board: [i32; BOARD_SIZE] = init_board();

        if args.verbose { println!("Board initialized:\n {:?}", board) }

        let players: Vec<Player> = vec!(Player::new(); args.playercount);

        if args.verbose { println!("Players initialized") }

        Game {args, board, players }
    }

    fn make_turn(start: i32, board: [i32; BOARD_SIZE]) -> i32 {
        let result = start + dice() + dice();
        // if board[result as usize] != 0 { println!("Uiuiui")}
        result+ board[result as usize]
    }

    pub fn print_players(&self, gamecounter: u32) {
        for index in 0..self.args.playercount {
            self.players[index].print(index, gamecounter);
        }
    }
}

impl Playable for Game {
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
}

fn dice() -> i32 {
    let rng: u32 = rand::random();
    (rng & 6) as i32 + 1
}

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

*/