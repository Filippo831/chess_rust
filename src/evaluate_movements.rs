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
    if board[row][col].is_uppercase() != is_white {
        possible_movements[row][col] = true;
    };
    // right side
    row = (piece_position.0 + turn) as usize;
    col = (piece_position.1 + 1) as usize;
    // if on the right border don't go further
    if col == 8 {
        col = 7
    }
    if board[row][col].is_uppercase() != is_white {
        possible_movements[row][col] = true;
    };
}

fn evaluate_strong_piece(
    possible_movements: Vec<Vec<bool>>,
    piece_position: (i32, i32),
    board: &Vec<Vec<char>>,
    is_white: bool,
    piece_movements: Vec<i32>,
    number_steps: i32,
) {
    let turn: i32 = is_white as i32 * 2 - 1;

    let good: Vec<bool> = vec![true; piece_movements.len()];

    // a = multiply factor

    let initial_position = piece_position.0 * 8 + piece_position.1;

    for a in 0..number_steps {
        for (i, mov) in piece_movements.iter().enumerate() {
            if good[i] {
                let check_position: i32 = initial_position + a * mov;
                let value: &char = board.iter().flatten().nth(check_position as usize).unwrap();

                // no pieces in that square
                if *value == '0' {
                    possible_movements[(check_position / 8) as usize]
                        [(check_position % 8) as usize] = true;
                }
                // opponent piece
                else if value.is_uppercase() != is_white {
                    possible_movements[(check_position / 8) as usize]
                        [(check_position % 8) as usize] = true;
                    good[i] = false;
                }
                // my piece
                else {
                    good[i] = false;
                }
                // if is checking on the border stop that line
                if check_position / 8 == 0
                    || check_position / 8 == 7
                    || check_position % 8 == 0
                    || check_position % 8 == 7
                {
                    good[i] = false;
                }
            }
        }
    }
}
