use crate::modules::board::init_board;
use crate::*;
use crate::modules::Player;

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

    pub fn play_game(&mut self) {
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

    fn dice(faces: u32) -> i32 {
        (rand::random::<u32>() % faces) as i32
    }

    fn make_turn(start: i32, board: [i32; BOARD_SIZE]) -> i32 {
        let result = start + Game::dice(6) + Game::dice(6);
        // if board[result as usize] != 0 { println!("Uiuiui")}
        result + board[result as usize]
    }

    pub fn print_players(&self, gamecounter: u32) {
        for index in 0..self.args.playercount {
            self.players[index].print(index, gamecounter);
        }
    }
}