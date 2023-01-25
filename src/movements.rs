use ggez::{graphics, mint::Point2};

pub struct MovingStruct {
    is_moving: bool,
    piece: char,
    piece_mesh: graphics::Text,
}
impl MovingStruct {
    pub fn toggle_is_moving(&mut self) {
        self.is_moving = !self.is_moving;
    }

    fn set_piece(&mut self, new_piece: char) {
        self.piece = new_piece;
    }
    pub fn new(is_moving: bool, piece: char) -> MovingStruct {
        let piece_mesh: graphics::Text = graphics::Text::new("");
        MovingStruct {
            is_moving,
            piece,
            piece_mesh,
        }
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
    // get piece by the coordinates
    let x_nth: i32 = (coordinates.x / sq_dim).floor() as i32;
    let y_nth: i32 = (coordinates.y / sq_dim).floor() as i32;

    // pick up the piece if you select a piece on the board
    if board[y_nth as usize][x_nth as usize] != '0' {
        moving_piece.toggle_is_moving();
        moving_piece.set_piece(board[y_nth as usize][x_nth as usize]);
        moving_piece.piece_mesh = graphics::Text::new(moving_piece.fetch_piece());
        board[y_nth as usize][x_nth as usize] = '0';
    }
}

pub unsafe fn put_piece(
    coordinates: Point2<f32>,
    board: &mut Vec<Vec<char>>,
    sq_dim: f32,
    moving_piece: &mut MovingStruct,
) {
    // get piece by the coordinates
    let x_nth: i32 = (coordinates.x / sq_dim).floor() as i32;
    let y_nth: i32 = (coordinates.y / sq_dim).floor() as i32;
    let board_caracter: *mut char = &mut board[y_nth as usize][x_nth as usize];

    // check whether the destintion is empty or occupied by opponents pieces
    if (*board_caracter == '0')
        || (board_caracter.as_ref().unwrap().is_uppercase()
            != moving_piece.fetch_piece().is_uppercase())
    {
        // if true put the moved piece in that position
        *board_caracter = moving_piece.fetch_piece();
        moving_piece.set_piece('0');
        moving_piece.toggle_is_moving();
        moving_piece.piece_mesh = graphics::Text::new(moving_piece.fetch_piece());
    }
}
