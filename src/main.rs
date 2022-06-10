const WINNING_SQUARE: i32 = 100;
const BOARD_SIZE: usize = WINNING_SQUARE as usize + 13;
const PLAYERCOUNT: usize = 4;
const MAX_TURNS: i32 = 136;
const GAMES: i32 = 1000;

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

    fn print(&self, player_nr: usize) {
        println!("Player {} has won {} times ({:.2}%) with an average of {} turns. He got hit {} times.", player_nr, self.wins, (self.wins as f32) / (GAMES as f32) * 100., self.mean, self.times_hit);
    }
}

fn main() {
    let board = init_board();

    let mut players: [Player; PLAYERCOUNT] = [Player::new(); PLAYERCOUNT];

    for _ in 0..GAMES {
        play_game(&mut players, &board);
    }

    for index in 0..players.len() {
        players[index].print(index);
    }
}

fn dice(faces: u32) -> i32 {
    (rand::random::<u32>() % faces) as i32
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

    println!("Board is initialized.");

    board
}

fn ladder(board: &mut [i32; BOARD_SIZE], pos: i32, jump: i32) {
    board[pos as usize] = jump-pos;
}

fn snake(board: &mut [i32; BOARD_SIZE], pos: i32, fall: i32) {
    board[pos as usize] = fall-pos;
}

fn play_game(players: &mut [Player; PLAYERCOUNT], board: &[i32; BOARD_SIZE]) {
    let mut pos: [i32; PLAYERCOUNT] = [0; PLAYERCOUNT];

    for turn in 0..MAX_TURNS {
        for i in 0..PLAYERCOUNT {
            print!("Player {} moves to", i);
            pos[i] = make_turn(pos[i], &board);
            println!("{}", pos[i]);

            if pos[i] >= WINNING_SQUARE {
                players[i].has_won(turn);
                return;
            }

            for j in 0..PLAYERCOUNT {
                if pos[i] == pos[j] && i != j {
                    players[j].got_hit();
                    println!("Oh no, Player {} got hit!", j);
                    pos[j] = 0;
                }
            }
        }
    }
}

fn make_turn(pos: i32, board: &[i32; BOARD_SIZE]) -> i32 {
    let result = pos + dice(6) + dice(6);
    result + board[result as usize]
}
