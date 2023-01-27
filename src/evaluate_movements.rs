use ggez::mint::Point2;

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
}
