use sdl2::{
    rect::{Rect, Point},
    pixels::Color,
    render::Canvas,
    video::Window,
};

pub struct Renderer {
    pub screen_area: Rect,
    pub clear_color: Color,
}

impl Renderer {
    
    pub fn render(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(self.clear_color);
        canvas.fill_rect(self.screen_area).ok().unwrap_or_default();
        

        canvas.set_draw_color(Color::RGB(0, 0, 0));

        let square_amount = 99;

        let square_size: f64;
        let board_size: f64;
        
        let board_offset: f64 = 40.0;
        let board_offset_x: f64;
        let board_offset_y: f64;

        let screen_width = self.screen_area.w as f64;
        let screen_height = self.screen_area.h as f64;

        let square_width = (screen_width - 2.0 * board_offset) / square_amount as f64;
        let square_height = (screen_height as f64 - 2.0 * board_offset) / square_amount as f64;

        if screen_height > screen_width {
            square_size = square_width;
            board_size = square_size * square_amount as f64;
            board_offset_x = board_offset;
            board_offset_y = board_offset - (screen_height - screen_width) / 2.0;
        } else {
            square_size = square_height;
            board_size = square_size * square_amount as f64;
            board_offset_x = board_offset - (screen_height - screen_width) / 2.0;
            board_offset_y = board_offset;
        }

        let top_left = Point::new(board_offset_x as i32, board_offset_y as i32);
        let top_right = Point::new(board_offset_x as i32, board_offset_y as i32 + board_size as i32);
        let bottom_left = Point::new(board_offset_x as i32 + board_size as i32, board_offset_y as i32);
        let bottom_right = Point::new(board_offset_x as i32 + board_size as i32, board_offset_y as i32 + board_size as i32);


        fn draw_square(size: f64, xstart: f64, ystart: f64, canvas: &mut Canvas<Window>) {
            for i in 0..size.ceil() as i32{

                let yend = (ystart + size).floor() as i32;
    
                canvas.draw_line(
                    Point::new(xstart.ceil() as i32 + i, ystart.floor() as i32),
                    Point::new(xstart.ceil() as i32 + i, yend)
                ).ok().unwrap_or_default();
            }
        }
        

        for i in 0..square_amount {

            for j in 0..square_amount{

                if ((j + i) % 2) == 0 {
                    canvas.set_draw_color(Color::RGB(255, 243, 214));
                } else {
                    canvas.set_draw_color(Color::RGB(87, 48, 10));
                }
                
                draw_square(square_size, 
                            board_offset_x + square_size * i as f64, 
                            board_offset_y + square_size * j as f64, 
                            canvas
                        );              
            }
        } 

        canvas.set_draw_color(Color::RGB(0, 0, 0));

        canvas.draw_line(top_right, top_left).ok().unwrap_or_default();
        canvas.draw_line(top_right, bottom_right).ok().unwrap_or_default();
        canvas.draw_line(bottom_left, top_left).ok().unwrap_or_default();
        canvas.draw_line(bottom_left, bottom_right).ok().unwrap_or_default();
    }

}