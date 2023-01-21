pub fn to_fen(board: Vec<Vec<&str>>) -> &str {
    let mut fen_string: &str = "";

    return fen_string;
}

pub fn from_fen(fen_string: &str) -> Vec<Vec<char>> {
    let mut board: Vec<Vec<char>> = vec![vec!['0'; 8]; 8];
    let fen_blocks: Vec<&str> = fen_string.split("/").collect();

    for (y, line) in fen_blocks.iter().enumerate() {
        let mut index = 0;
        for (x, c) in line.chars().enumerate() {
            if c.is_numeric() {
                index += c.to_digit(10).unwrap() as usize;
            } else {
                board[y][index] = c;
                index += 1;
            }
        }
    }

    return board;
}
