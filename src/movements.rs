use ggez::mint::Point2;

pub fn get_piece(coordinates: Point2<f32>, board: &mut Vec<Vec<char>>, sq_dim: f32) {
    let x_nth: i32 = (coordinates.x / sq_dim).floor() as i32;
    let y_nth: i32 = (coordinates.y / sq_dim).floor() as i32;
    board[y_nth as usize][x_nth as usize] = '0';
}
