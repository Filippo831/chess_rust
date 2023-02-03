use ggez::mint::Point2;
use num;

fn evaluate_piece(
    piece: char,
    piece_position: (i32, i32),
    board: &Vec<Vec<char>>,
) -> Vec<Vec<bool>> {
    let possible_movements: Vec<Vec<bool>> = vec![];
    if piece.to_lowercase().to_string() == "p" {}
    return possible_movements;
}

fn evaluate_pawn(
    possible_movements: Vec<Vec<bool>>,
    piece_position: (i32, i32),
    board: &Vec<Vec<char>>,
    is_white: bool,
) {
    // if is_white is true (1) * 2 == 2 / 2 == 1 esle if false (0) * 2 == 0 - 1 == -1
    let turn: i32 = is_white as i32 * 2 - 1;
    let mut row: usize;
    let mut col: usize;
    // front movement
    row = (piece_position.0 + turn) as usize;
    col = piece_position.1 as usize;
    if board[row][col] == '0' {
        possible_movements[row][col] = true;
    }
    //left side movement
    row = (piece_position.0 + turn) as usize;
    col = (piece_position.1 - 1).abs() as usize;
    if possible_movements[row][col] {
        possible_movements[row][col] = true;
    };
    // right side
    row = (piece_position.0 + turn) as usize;
    col = (piece_position.1 + 1) as usize;
    // if on the right border don't go further
    if col == 8 {
        col = 7
    }
    if possible_movements[row][col] {
        possible_movements[row][col] = true;
    };
}

fn evaluate_rook(
    possible_movements: Vec<Vec<bool>>,
    piece_position: (i32, i32),
    board: &Vec<Vec<char>>,
    is_white: bool,
) {
    let turn: i32 = is_white as i32 * 2 - 1;
    let mut row: usize;
    let mut col: usize;

    let rook_movements: Vec<i32> = vec![1, 8, -1, -8];
    let good: Vec<bool> = vec![true, true, true, true];

    // a = multiply factor
    // m = movement factor
    for a in 1..7 {
        for (i, m) in rook_movements.iter().enumerate() {
            row = (m % 8) as usize;
            col = (m / 8) as usize;
            if board[row][col] == '0' {
                board[row][col] = true;
            } else if board[row][col].is
        }
    }
}
