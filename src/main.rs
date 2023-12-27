extern crate sdl2;

use std::path::Path;
use std::time::Duration;
use sdl2::render::{TextureCreator, Texture, Canvas};
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::video::{Window, WindowContext};
use sdl2::ttf::Font;

enum GameState {
    menu,
    play,
    pause,
    death,
    test,
}

fn load_texture<'a>(tc: &'a TextureCreator<WindowContext>, filename: &str) -> Texture<'a> {
    return tc.load_texture(format!("./assets/{}", filename)).unwrap();
}

fn set_text(canvas: &mut Canvas<Window>, font: &Font, tc: &TextureCreator<WindowContext>, color: Color, text: &str, position_rect: Rect) {
    let surface = font.render(text).blended(color).unwrap();
    let texture = tc.create_texture_from_surface(&surface).unwrap();
    canvas.copy(&texture, None, Some(position_rect)).unwrap();
}

fn write_tetris_by_textures(canvas: &mut Canvas<Window>, texture: &Texture) {
    for x in 0..19 {
        if !vec![3, 6, 10, 13, 15, 16].iter().any(|&i| i == x) {
            canvas.copy(texture, None, Some(Rect::new(15+x*15, 15, 15, 15))).unwrap();
        }
        if !vec![0, 2, 3, 5, 6, 7, 9, 10, 12, 13, 15, 16, 18].iter().any(|&i| i == x) {
            canvas.copy(texture, None, Some(Rect::new(15+x*15, 30, 15, 15))).unwrap();
        }
        if !vec![0, 2, 3, 6, 7, 9, 10, 12, 13, 15, 18].iter().any(|&i| i == x) {
            canvas.copy(texture, None, Some(Rect::new(15+x*15, 45, 15, 15))).unwrap();
        }
    }
}

pub fn main() {
    // const
    let window_width = 314;
    let window_height = 704;
    let bg_color = Color::RGB(0, 0, 0);
    let fg_color = Color::RGB(204, 204, 204);
    let hl_color = Color::RGB(204, 255, 136);

    // mut
    let mut state = GameState::menu;

    // sdl stuff
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("tetris", window_width, window_height).build().unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();
    let ttf_context = sdl2::ttf::init().unwrap();

    // loading font
    let font_path: &Path = Path::new(&"./assets/ShadowsIntoLight-Regular.ttf");
    let font = ttf_context.load_font(font_path, 128).unwrap();

    // loading textures
    let chimp_texture = texture_creator.load_texture("./assets/chimp.png").unwrap();

    let gray_piece_texture = load_texture(&texture_creator, "/none.png");
    let cyan_piece_texture = load_texture(&texture_creator, "/cyan.png");
    let purple_piece_texture = load_texture(&texture_creator, "/purple.png");
    let deep_purple_piece_texture = load_texture(&texture_creator, "/deep_purple.png");
    let red_piece_texture = load_texture(&texture_creator, "/red.png");
    let orange_piece_texture = load_texture(&texture_creator, "/orange.png");
    let green_piece_texture = load_texture(&texture_creator, "/green.png");

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        // process controls
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Num9), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::Num1), .. } => {
                    state = GameState::play;
                },
                Event::KeyDown { keycode: Some(Keycode::Num2), .. } => {
                    state = GameState::play;
                },
                Event::KeyDown { keycode: Some(Keycode::Num3), .. } => {
                    state = GameState::pause;
                },
                Event::KeyDown { keycode: Some(Keycode::Num4), .. } => {
                    state = GameState::menu;
                },
                Event::KeyDown { keycode: Some(Keycode::Num0), .. } => {
                    state = GameState::test;
                },
                _ => {}
            }
        }

        // clear canvas
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        // handle logic

        // draw

        match state {
            GameState::test => {
                for x in 0..10 {
                    for y in 0..20 {
                        canvas.copy(&deep_purple_piece_texture, None, Some(Rect::new(7+x*30, 97+y*30, 30, 30))).unwrap();
                    }
                }

                set_text(&mut canvas, &font, &texture_creator, fg_color, "score: 1215", Rect::new(134, 7, 173, 30));
                set_text(&mut canvas, &font, &texture_creator, hl_color, "time: 11:50", Rect::new(134, 37, 173, 30));

                write_tetris_by_textures(&mut canvas, &green_piece_texture);
            },
            GameState::menu => {
                write_tetris_by_textures(&mut canvas, &green_piece_texture);

                set_text(&mut canvas, &font, &texture_creator, hl_color, "1: play", Rect::new(7, 104, window_width - 14, 60));
                set_text(&mut canvas, &font, &texture_creator, fg_color, "2: restart", Rect::new(7, 104+60, window_width - 14, 60));
                set_text(&mut canvas, &font, &texture_creator, fg_color, "3: pause", Rect::new(7, 104+120, window_width - 14, 60));
                set_text(&mut canvas, &font, &texture_creator, fg_color, "4: menu", Rect::new(7, 104+180, window_width - 14, 60));
                set_text(&mut canvas, &font, &texture_creator, fg_color, "9: exit", Rect::new(7, 104+240, window_width - 14, 60));
                set_text(&mut canvas, &font, &texture_creator, fg_color, "0: test", Rect::new(7, 104+280, window_width - 14, 60));

            },
            _ => (),
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
