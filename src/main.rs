extern crate sdl2;

use std::path::Path;
use std::time::Duration;
use sdl2::render::TextureCreator;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::image::{InitFlag, LoadTexture};

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("tetris", 800, 600)
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let texture_creator = canvas.texture_creator();
    let ttf_context = sdl2::ttf::init().unwrap();
    let font_path: &Path = Path::new(&"./assets/ShadowsIntoLight-Regular.ttf");
    let mut font = ttf_context.load_font(font_path, 128).unwrap();

    let image_context = sdl2::image::init(InitFlag::PNG).unwrap();
    let chimp_texture = texture_creator.load_texture("./assets/chimp.png").unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        canvas.copy(&chimp_texture, None, None).unwrap();

        canvas.set_draw_color(Color::RGB(0, 255, 255));
        canvas.fill_rect(Rect::new(12, 15, 200, 200));

        let surface = font.render(&"hueta moya huetovaya").blended(Color::RGB(200, 0, 0)).unwrap();
        let texture = texture_creator.create_texture_from_surface(&surface).unwrap();
        let target = Rect::new(12, 15, 400, 100);
        canvas.copy(&texture, None, Some(target)).unwrap();



        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
