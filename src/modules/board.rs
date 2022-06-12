use crate::BOARD_SIZE;

pub fn init_board() -> [i32; BOARD_SIZE] {
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
