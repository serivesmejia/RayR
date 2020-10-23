extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use ray_r::handler::key_handler;
use ray_r::game::Game;

fn main() -> Result<(), String> {

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window("Raycasting", 800, 600)
                .position_centered()
                .build()
                .expect("Could not initialize video subsystem");

    let mut canvas = window.into_canvas().build()
                    .expect("Could not make a canvas");

    canvas.set_draw_color(Color::RGB(0, 0, 0));

    let mut game = Game::new();

    let mut handler_key = key_handler::KeyHandler::new();

    let mut events = sdl_context.event_pump()?;

    'running: loop {

        canvas.clear();

        for event in events.poll_iter() {
            if let Event::Quit {..} = event {
                break 'running;
            };
        }

        let keys = events.keyboard_state().pressed_scancodes().filter_map(Keycode::from_scancode).collect();
        handler_key.handle_keys(keys, &mut game);

        game.update();
        game.player.update();

        canvas.present();

    }

    Ok(())

}