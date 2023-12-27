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

type GameField = [[char; 10]; 20];

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

struct Piece {
    x: i32,
    y: i32,
    rotation: i32,
    literal: char,
}

impl Piece {
    fn new(literal: char) -> Piece {
        return Piece {
            x: 5,
            y: 1,
            rotation: 0,
            literal: literal,
        }
    }

    fn body(&self) -> [[char; 4]; 4] {
        let lit = self.literal;

        return [
            [' ', ' ', ' ', ' '],
            [' ', lit, lit, ' '],
            [' ', lit, lit, ' '],
            [' ', ' ', ' ', ' '],
        ]
    }
}

pub fn main() {
    // const
    let framerate = 60;
    let window_width = 7 + 30*10 + 7 + 30*4 + 7;
    let window_height = 7 + 30*20 + 7;
    let bg_color = Color::RGB(0, 0, 0);
    let fg_color = Color::RGB(204, 204, 204);
    let hl_color = Color::RGB(204, 255, 136);

    // mut
    let mut tick_once_per_frames = 50;
    let mut frames_to_tick = 0;
    let mut state = GameState::menu;
    let mut field: GameField = [[' '; 10]; 20];
    let mut piece = Piece::new('Y');

    field[3][4] = 'Y';
    field[4][4] = 'Y';
    field[5][4] = 'Y';
    field[5][5] = 'N';

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
    let yellow_piece_texture = load_texture(&texture_creator, "/yellow.png");
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
                        canvas.copy(&deep_purple_piece_texture, None, Some(Rect::new(7+x*30, 7+y*30, 30, 30))).unwrap();
                    }
                }

                canvas.copy(&green_piece_texture, None, Some(Rect::new(7+10*30+7 + 0*30, 7+5*30, 30, 30))).unwrap();
                canvas.copy(&green_piece_texture, None, Some(Rect::new(7+11*30+7 + 0*30, 7+5*30, 30, 30))).unwrap();
                canvas.copy(&green_piece_texture, None, Some(Rect::new(7+12*30+7 + 0*30, 7+5*30, 30, 30))).unwrap();
                canvas.copy(&green_piece_texture, None, Some(Rect::new(7+12*30+7 + 0*30, 7+4*30, 30, 30))).unwrap();

                set_text(&mut canvas, &font, &texture_creator, fg_color, "level: 1", Rect::new(7+10*30+7, 200+0*30, 4*30, 30));
                set_text(&mut canvas, &font, &texture_creator, fg_color, "score: 1215", Rect::new(7+10*30+7, 200+1*30, 4*30, 30));
                set_text(&mut canvas, &font, &texture_creator, fg_color, "lines: 12", Rect::new(7+10*30+7, 200+2*30, 4*30, 30));
                set_text(&mut canvas, &font, &texture_creator, fg_color, "time: 12:15", Rect::new(7+10*30+7, 200+3*30, 4*30, 30));
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
            GameState::play => {
                canvas.set_draw_color(Color::RGB(30, 30, 30));
                canvas.fill_rect(Rect::new(7+10*30+7, 0, 30*4 + 7, window_height));

                if frames_to_tick <= 0 {
                    frames_to_tick = tick_once_per_frames;

                    piece.y += 1;

                    // put piece on a field
                    for y in 0..field.len() {
                        for x in 0..field[y].len() {
                            if field[y][x] != 'N' {
                                field[y][x] = ' '
                            }
                        }
                    }
                    for y in 0..4 {
                        for x in 0..4 {
                            let body = piece.body();
                            let fx = piece.x + x - 2;
                            let fy = piece.y + y - 2;

                            if fx < 0 || fx >= 10 || fy < 0 || fy >= 20 || body[y as usize][x as usize] == ' ' {
                                continue;
                            }

                            field[fy as usize][fx as usize] = body[y as usize][x as usize];
                        }
                    }
                } else {
                    frames_to_tick -= 1;
                }

                for y in 0..field.len() {
                    for x in 0..field[y].len() {
                        match field[y][x] {
                            'Y' => {
                                canvas.copy(&yellow_piece_texture, None, Some(Rect::new(7+x as i32 * 30, 7+y as i32 * 30, 30, 30))).unwrap();
                            },
                            'N' => {
                                canvas.copy(&gray_piece_texture, None, Some(Rect::new(7+x as i32 * 30, 7+y as i32 * 30, 30, 30))).unwrap();
                            },
                            _ => (),
                        }
                    }
                }
            },
            _ => (),
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / framerate));
    }
}
