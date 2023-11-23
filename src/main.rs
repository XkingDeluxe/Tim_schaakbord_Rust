use sdl2::{
    rect::Rect,
    pixels::Color,
    event::Event
};

mod view;
use view::board_view;

mod model;
use model::game::make_blank_board;
use model::game::GameState;

fn main() -> Result<(), String>{
    
    let screen_width = 640;
    let screen_height = 360;

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem.window("IDK", screen_width, screen_height)
        .build()
        .unwrap();

        let mut canvas = window.into_canvas()
        .build()
        .unwrap();

    let board_view: board_view::Renderer = board_view::Renderer { 
        screen_area: Rect::new(0, 0, screen_width, screen_height), 
        clear_color: Color::RGB(45, 145, 145),
    };

    let mut game_state = GameState { board: make_blank_board()};

    game_state.print_board();
    game_state.start_position();
    game_state.print_board();

    let mut running = true;
    let mut event_queue = sdl_context.event_pump().unwrap();

    while running {
        
        for event in event_queue.poll_iter() {
            
            match event {
                Event::Quit { .. } => {
                    running = false;
                },
                _ => {}
            }
        }

        board_view.render(&mut canvas);
        canvas.present();
    }

    Ok(())
}
