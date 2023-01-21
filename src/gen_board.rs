use ggez::{graphics, Context};

pub fn generate_board(screen_size: f32, ctx: &mut Context) -> Vec<graphics::Mesh> {
    let mut board_vec: Vec<graphics::Mesh> = vec![];
    let square_size: f32 = screen_size / 8.0;
    for x in 0..8 {
        for y in 0..8 {
            let color: graphics::Color;
            if (x + y) % 2 == 0 {
                color = graphics::Color::from_rgb(115, 56, 26);
            } else {
                color = graphics::Color::from_rgb(191, 101, 55);
            }
            let square = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                graphics::Rect {
                    x: square_size * x as f32,
                    y: square_size * y as f32,
                    w: square_size,
                    h: square_size,
                },
                color,
            );
            board_vec.push(square.unwrap());
        }
    }
    return board_vec;
}
