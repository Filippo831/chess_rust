//! The simplest possible example that does something.
#![allow(clippy::unnecessary_wraps)]

mod evaluate_movements;
mod event_handler;
mod fen_functions;
mod gen_board;
mod movements;
mod vis_board;

use ggez::{
    conf::WindowMode,
    event,
    glam::*,
    graphics::{self, Color, DrawParam, Drawable},
    mint::Point2,
    Context, GameResult,
};

const SCREEN_SIZE: f32 = 400.0;

struct MainState {
    board_vec: Vec<graphics::Mesh>,
    board_test: Vec<Vec<char>>,
    pieces_vec: Vec<(graphics::Text, DrawParam)>,
    fen: String,
    moving_piece: movements::MovingStruct,
}

impl MainState {
    fn create_board(ctx: &mut Context) -> GameResult<MainState> {
        let fen = "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR".to_string();
        let board_test: Vec<Vec<char>> = fen_functions::from_fen(&fen);
        let board_vec = gen_board::generate_board(SCREEN_SIZE, ctx);
        let pieces_vec = vis_board::board_to_vis(&board_test, ctx, SCREEN_SIZE);
        let moving_piece = movements::MovingStruct::new(true, '0');

        Ok(MainState {
            board_vec,
            board_test,
            pieces_vec,
            fen,
            moving_piece,
        })
    }
}
impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        //get mouse position
        let mouse_position: Point2<f32> = _ctx.mouse.position();
        self.moving_piece.set_position(mouse_position);
        let is_clicked: bool = _ctx.mouse.button_just_pressed(event::MouseButton::Left);
        if is_clicked {
            //when the mouse is clicked see if you have a piece moving or not, if true pick the
            //piece you selected otherwise place the selected piece in that position
            event_handler::handle_click(
                mouse_position,
                &mut self.board_test,
                &mut self.moving_piece,
                &mut self.fen,
                &mut self.board_vec,
                &mut self.pieces_vec,
                _ctx,
            );
        }
        if self.moving_piece.fetch_is_moving() {}
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));

        //draw the board
        for sq in &self.board_vec {
            canvas.draw(sq, Vec2::new(0.0, 0.0));
        }
        //draw pieces
        for piece in &self.pieces_vec {
            canvas.draw(&piece.0, piece.1);
        }

        let moving_piece_mesh: &(graphics::Text, graphics::DrawParam) =
            &self.moving_piece.get_piece_mesh();

        canvas.draw(&moving_piece_mesh.0, moving_piece_mesh.1);

        canvas.finish(ctx)?;

        Ok(())
    }
}

pub fn main() -> GameResult {
    //set dimension
    let window_mode = WindowMode::default()
        .dimensions(SCREEN_SIZE, SCREEN_SIZE)
        .resizable(false);
    let mut conf = ggez::conf::Conf::new();

    //change window title
    conf.window_setup.title = "chesso".to_owned();
    // settings, directory of the resources
    let cb = ggez::ContextBuilder::new("super_simple", "ggez")
        .add_resource_path("/home/filippo/Desktop/projects/chess_rust/sprites")
        .default_conf(conf)
        .window_mode(window_mode);

    let (mut ctx, event_loop) = cb.build()?;
    let state = MainState::create_board(&mut ctx)?;
    event::run(ctx, event_loop, state)
}
