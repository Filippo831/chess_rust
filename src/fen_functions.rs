pub fn to_fen(board: &Vec<Vec<char>>) -> String {
    let mut fen_string: String = "".to_string();
    for line in board {
        let mut blank_counter: u8 = 0;
        for e in line {
            if *e == '0' {
                blank_counter += 1;
            } else {
                if blank_counter != 0 {
                    fen_string.push_str(&blank_counter.to_string());
                    fen_string.push(*e);
                    blank_counter = 0;
                } else {
                    fen_string.push(*e);
                }
            }
        }
        if blank_counter != 0 {
            fen_string.push_str(&blank_counter.to_string());
        }
        fen_string.push('/');
    }

    return fen_string;
}

pub fn from_fen(fen_string: &String) -> Vec<Vec<char>> {
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
