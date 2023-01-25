use ggez::mint::Point2;

pub struct MovingStruct {
    is_moving: bool,
    piece: char,
}
impl MovingStruct {
    pub fn toggle_is_moving(&mut self) {
        self.is_moving = !self.is_moving;
    }

    fn set_piece(&mut self, new_piece: char) {
        self.piece = new_piece;
    }
    pub fn new(is_moving: bool, piece: char) -> MovingStruct {
        MovingStruct { is_moving, piece }
    }

    pub fn fetch_piece(&mut self) -> char {
        return self.piece;
    }
    pub fn fetch_is_moving(&mut self) -> bool {
        return self.is_moving;
    }
}

pub fn get_piece(
    coordinates: Point2<f32>,
    board: &mut Vec<Vec<char>>,
    sq_dim: f32,
    moving_piece: &mut MovingStruct,
) {
    let x_nth: i32 = (coordinates.x / sq_dim).floor() as i32;
    let y_nth: i32 = (coordinates.y / sq_dim).floor() as i32;
    if board[y_nth as usize][x_nth as usize] != '0' {
        moving_piece.toggle_is_moving();
        moving_piece.set_piece(board[y_nth as usize][x_nth as usize]);
        board[y_nth as usize][x_nth as usize] = '0';
    }
}

pub fn put_piece(
    coordinates: Point2<f32>,
    board: &mut Vec<Vec<char>>,
    sq_dim: f32,
    moving_piece: &mut MovingStruct,
) {
    let x_nth: i32 = (coordinates.x / sq_dim).floor() as i32;
    let y_nth: i32 = (coordinates.y / sq_dim).floor() as i32;
    if board[y_nth as usize][x_nth as usize] == '0' {
        moving_piece.toggle_is_moving();
        board[y_nth as usize][x_nth as usize] = moving_piece.fetch_piece();
        moving_piece.set_piece('0');
    }
}
