use ggez::graphics::DrawParam;
use ggez::mint::Point2;
use ggez::{graphics, Context};

use crate::movements::MovingStruct;
use crate::SCREEN_SIZE;
use crate::{fen_functions, gen_board, movements, vis_board};

pub fn handle_click(
    mouse_position: Point2<f32>,
    board_test: &mut Vec<Vec<char>>,
    moving_piece: &mut MovingStruct,
    fen: &mut String,
    board_vec: &mut Vec<graphics::Mesh>,
    pieces_vec: &mut Vec<(graphics::Text, DrawParam)>,
    _ctx: &mut Context,
) {
    //when the mouse is clicked see if you have a piece moving or not, if true pick the
    //piece you selected otherwise place the selected piece in that position
    if moving_piece.fetch_is_moving() {
        movements::get_piece(mouse_position, board_test, SCREEN_SIZE / 8.0, moving_piece);
    } else {
        unsafe {
            movements::put_piece(mouse_position, board_test, SCREEN_SIZE / 8.0, moving_piece);
        }
    }
    *fen = fen_functions::to_fen(board_test);
    *board_vec = gen_board::generate_board(SCREEN_SIZE, _ctx);
    *pieces_vec = vis_board::board_to_vis(board_test, _ctx, SCREEN_SIZE);
}
