extern crate sdl2;

use std::path::Path;
use std::time::Duration;
use sdl2::render::{TextureCreator, Texture, Canvas};
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::image::LoadTexture;
use sdl2::video::{Window, WindowContext};
use sdl2::ttf::Font;
use rand;

type GameField = [[char; 10]; 20];

enum GameState {
    Menu,
    Play,
    Death,
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
            canvas.copy(texture, None, Some(Rect::new(20+x*20, 20, 20, 20))).unwrap();
        }
        if !vec![0, 2, 3, 5, 6, 7, 9, 10, 12, 13, 15, 16, 18].iter().any(|&i| i == x) {
            canvas.copy(texture, None, Some(Rect::new(20+x*20, 40, 20, 20))).unwrap();
        }
        if !vec![0, 2, 3, 6, 7, 9, 10, 12, 13, 15, 18].iter().any(|&i| i == x) {
            canvas.copy(texture, None, Some(Rect::new(20+x*20, 60, 20, 20))).unwrap();
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
    fn new() -> Piece {
        let lit = match rand::random::<u8>() % 6 {
            0 => 'Y',
            1 => 'C',
            2 => 'P',
            3 => 'D',
            4 => 'R',
            5 => 'O',
            6 => 'G',
            _ => 'N',
        };
        return Piece {
            x: 5,
            y: match lit {
                'C' => 0,
                _ => 1,
            },
            rotation: 0,
            literal: lit,
        }
    }

    // return true if moved, false if something blocks move
    fn move_x(&mut self, delta: i32, field: &GameField) -> bool {
        let body = self.body();
        match delta {
            d if d < 0 => {
                for y in 0..4 {
                    for x in 0..4 {
                        if body[y][x] != ' ' {
                            let left_cell_i = self.x + x as i32 - 2 - 1;
                            let cy = self.y - 2 + y as i32;

                            if left_cell_i < 0 {
                                return false;
                            }
                            if cy < 0 {
                                continue;
                            }
                            if field[cy as usize][left_cell_i as usize] == 'N' {
                                return false;
                            }
                            break;
                        }
                    }
                }
            },
            d if d > 0 => {
                for y in 0..4 {
                    for x in (0..4).rev() {
                        if body[y][x] != ' ' {
                            let right_cell_i = self.x + x as i32 - 2 + 1;
                            let cy = self.y - 2 + y as i32;

                            if right_cell_i >= 10 {
                                return false;
                            }
                            if cy < 0 {
                                continue;
                            }
                            if field[cy as usize][right_cell_i as usize] == 'N' {
                                return false;
                            }
                            break;
                        }
                    }
                }
            },
            _ => (),
        }
        self.x += delta;
        return true
    }

    fn is_move_down_awailable(&self, field: &GameField) -> bool {
        let body = self.body();
        for x in 0..4 {
            for y in (0..4).rev() {
                if body[y][x] != ' ' {
                    let below_cell_i = self.y + y as i32 - 2 + 1;
                    let cx = self.x - 2 + x as i32;
                    if cx < 0 || below_cell_i < 0 {
                        continue;
                    }
                    if below_cell_i >= 20 || field[below_cell_i as usize][cx as usize] == 'N' {
                        return false;
                    }
                    break;
                }
            }
        }
        return true;
    }

    fn force_move_y(&mut self, delta: i32) {
        self.y += delta;
    }

    fn drop_down(&mut self, field: &GameField) {
        while self.is_move_down_awailable(&field) {
            self.force_move_y(1);
        }
    }

    fn rotate(&mut self, field: &GameField, direction: i32) {
        let rot_backup = self.rotation;
        self.rotation = (self.rotation + direction) % 4;
        if self.rotation < 0 {
            self.rotation = 3;
        }
        let body = self.body();
        self.rotation = rot_backup;

        for y in 0..4 {
            let mut x = -1;
            while x < 3 {
                x += 1;

                let fx = self.x + x - 2;
                let fy = self.y + y - 2;

                if body[y as usize][x as usize] == ' ' {
                    continue;
                }

                if fx < 0 {
                    x -= 1;
                    self.x += 1;
                    continue;
                }
                if fx >= 10 {
                    x -= 1;
                    self.x -= 1;
                    continue;
                }

                if fy < 0 {
                    continue
                }

                if fy >= 20 || field[fy as usize][fx as usize] == 'N' {
                    return;
                }
            }
        }

        self.rotation = (self.rotation + direction) % 4;
        if self.rotation < 0 {
            self.rotation = 3;
        }
    }

    fn put_on_a_field(&self, field: &mut GameField, force_gray: bool) {
        let body = self.body();

        for y in 0..field.len() {
            for x in 0..field[y].len() {
                if field[y][x] != 'N' {
                    field[y][x] = ' '
                }
            }
        }

        for y in 0..4 {
            for x in 0..4 {
                let fx = self.x + x - 2;
                let fy = self.y + y - 2;

                if fx < 0 || fx >= 10 || fy < 0 || fy >= 20 || body[y as usize][x as usize] == ' ' {
                    continue;
                }

                field[fy as usize][fx as usize] = match force_gray {
                    true => 'N',
                    false => body[y as usize][x as usize],
                }
            }
        }
    }

    fn body(&self) -> [[char; 4]; 4] {
        let lit = self.literal;
        let rot = self.rotation;

        let default = [
            [' ', ' ', ' ', ' '],
            [' ', lit, lit, ' '],
            [' ', lit, lit, ' '],
            [' ', ' ', ' ', ' '],
        ];

        return match lit {
            'Y' => [
                [' ', ' ', ' ', ' '],
                [' ', lit, lit, ' '],
                [' ', lit, lit, ' '],
                [' ', ' ', ' ', ' '],
            ],
            'C' => match rot {
                0 => [
                    [' ', ' ', ' ', ' '],
                    [lit, lit, lit, lit],
                    [' ', ' ', ' ', ' '],
                    [' ', ' ', ' ', ' '],
                ],
                1 => [
                    [' ', ' ', lit, ' '],
                    [' ', ' ', lit, ' '],
                    [' ', ' ', lit, ' '],
                    [' ', ' ', lit, ' '],
                ],
                2 => [
                    [' ', ' ', ' ', ' '],
                    [' ', ' ', ' ', ' '],
                    [lit, lit, lit, lit],
                    [' ', ' ', ' ', ' '],
                ],
                3 => [
                    [' ', lit, ' ', ' '],
                    [' ', lit, ' ', ' '],
                    [' ', lit, ' ', ' '],
                    [' ', lit, ' ', ' '],
                ],
                _ => default,
            },
            'P' => match rot {
                0 => [
                    [' ', lit, ' ', ' '],
                    [lit, lit, lit, ' '],
                    [' ', ' ', ' ', ' '],
                    [' ', ' ', ' ', ' '],
                ],
                1 => [
                    [' ', lit, ' ', ' '],
                    [' ', lit, lit, ' '],
                    [' ', lit, ' ', ' '],
                    [' ', ' ', ' ', ' '],
                ],
                2 => [
                    [' ', ' ', ' ', ' '],
                    [lit, lit, lit, ' '],
                    [' ', lit, ' ', ' '],
                    [' ', ' ', ' ', ' '],
                ],
                3 => [
                    [' ', lit, ' ', ' '],
                    [lit, lit, ' ', ' '],
                    [' ', lit, ' ', ' '],
                    [' ', ' ', ' ', ' '],
                ],
                _ => default,
            },
            'D' => match rot {
                0 => [
                    [lit, ' ', ' ', ' '],
                    [lit, lit, lit, ' '],
                    [' ', ' ', ' ', ' '],
                    [' ', ' ', ' ', ' '],
                ],
                1 => [
                    [' ', lit, lit, ' '],
                    [' ', lit, ' ', ' '],
                    [' ', lit, ' ', ' '],
                    [' ', ' ', ' ', ' '],
                ],
                2 => [
                    [' ', ' ', ' ', ' '],
                    [lit, lit, lit, ' '],
                    [' ', ' ', lit, ' '],
                    [' ', ' ', ' ', ' '],
                ],
                3 => [
                    [' ', lit, ' ', ' '],
                    [' ', lit, ' ', ' '],
                    [lit, lit, ' ', ' '],
                    [' ', ' ', ' ', ' '],
                ],
                _ => default,
            },
            'R' => match rot {
                0 => [
                    [lit, lit, ' ', ' '],
                    [' ', lit, lit, ' '],
                    [' ', ' ', ' ', ' '],
                    [' ', ' ', ' ', ' '],
                ],
                1 => [
                    [' ', ' ', lit, ' '],
                    [' ', lit, lit, ' '],
                    [' ', lit, ' ', ' '],
                    [' ', ' ', ' ', ' '],
                ],
                2 => [
                    [' ', ' ', ' ', ' '],
                    [lit, lit, ' ', ' '],
                    [' ', lit, lit, ' '],
                    [' ', ' ', ' ', ' '],
                ],
                3 => [
                    [' ', lit, ' ', ' '],
                    [lit, lit, ' ', ' '],
                    [lit, ' ', ' ', ' '],
                    [' ', ' ', ' ', ' '],
                ],
                _ => default,
            },
            'O' => match rot {
                0 => [
                    [' ', ' ', lit, ' '],
                    [lit, lit, lit, ' '],
                    [' ', ' ', ' ', ' '],
                    [' ', ' ', ' ', ' '],
                ],
                1 => [
                    [' ', lit, ' ', ' '],
                    [' ', lit, ' ', ' '],
                    [' ', lit, lit, ' '],
                    [' ', ' ', ' ', ' '],
                ],
                2 => [
                    [' ', ' ', ' ', ' '],
                    [lit, lit, lit, ' '],
                    [lit, ' ', ' ', ' '],
                    [' ', ' ', ' ', ' '],
                ],
                3 => [
                    [lit, lit, ' ', ' '],
                    [' ', lit, ' ', ' '],
                    [' ', lit, ' ', ' '],
                    [' ', ' ', ' ', ' '],
                ],
                _ => default,
            },
            'G' => match rot {
                0 => [
                    [' ', lit, lit, ' '],
                    [lit, lit, ' ', ' '],
                    [' ', ' ', ' ', ' '],
                    [' ', ' ', ' ', ' '],
                ],
                1 => [
                    [' ', lit, ' ', ' '],
                    [' ', lit, lit, ' '],
                    [' ', ' ', lit, ' '],
                    [' ', ' ', ' ', ' '],
                ],
                2 => [
                    [' ', ' ', ' ', ' '],
                    [' ', lit, lit, ' '],
                    [lit, lit, ' ', ' '],
                    [' ', ' ', ' ', ' '],
                ],
                3 => [
                    [lit, ' ', ' ', ' '],
                    [lit, lit, ' ', ' '],
                    [' ', lit, ' ', ' '],
                    [' ', ' ', ' ', ' '],
                ],
                _ => default,
            },
            _ => default,
        }
    }
}

fn restart(field: &mut GameField, piece: &mut Piece, score: &mut i32, lines: &mut i32, seconds: &mut f64, preview_piece: &mut Piece) {
    for y in 0..field.len() {
        for x in 0..field[y].len() {
            field[y][x] = ' ';
        }
    }
    *piece = Piece::new();
    *preview_piece = Piece::new();
    *score = 0;
    *lines = 0;
    *seconds = 0.;
}

pub fn main() {
    // const
    let framerate = 60;
    let window_width = 7 + 30*10 + 7 + 30*4 + 7;
    let window_height = 7 + 30*20 + 7;
    let bg_color = Color::RGB(0, 0, 0);
    let fg_color = Color::RGB(204, 204, 204);
    let hl_color = Color::RGB(204, 255, 136);
    let at_color = Color::RGB(204, 40, 40);

    // mut
    let mut tick_once_per_frames;
    let mut frames_to_tick = 0;
    let mut state = GameState::Menu;
    let mut field: GameField = [[' '; 10]; 20];
    let mut preview_piece = Piece::new();
    let mut piece = Piece::new();
    let mut level = 1;
    let mut score = 0;
    let mut lines = 0;
    let mut seconds = 0.;

    // sdl stuff
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let mut window = video_subsystem.window("tetris", window_width, window_height);
    window.resizable();
    let window_builder = window.build().unwrap();
    let mut canvas = window_builder.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();
    let ttf_context = sdl2::ttf::init().unwrap();

    // loading font
    let font_path: &Path = Path::new(&"./assets/ShadowsIntoLight-Regular.ttf");
    let font = ttf_context.load_font(font_path, 128).unwrap();

    // loading textures
    let grid_texture = load_texture(&texture_creator, "/grid.png");
    let gray_piece_texture = load_texture(&texture_creator, "/none.png");
    let yellow_piece_texture = load_texture(&texture_creator, "/yellow.png");
    let cyan_piece_texture = load_texture(&texture_creator, "/cyan.png");
    let purple_piece_texture = load_texture(&texture_creator, "/purple.png");
    let deep_purple_piece_texture = load_texture(&texture_creator, "/deep_purple.png");
    let red_piece_texture = load_texture(&texture_creator, "/red.png");
    let orange_piece_texture = load_texture(&texture_creator, "/orange.png");
    let green_piece_texture = load_texture(&texture_creator, "/green.png");

    // select texture function
    let get_texture = |ch: char| match ch {
        'N' => &gray_piece_texture,
        'Y' => &yellow_piece_texture,
        'C' => &cyan_piece_texture,
        'P' => &purple_piece_texture,
        'D' => &deep_purple_piece_texture,
        'R' => &red_piece_texture,
        'O' => &orange_piece_texture,
        'G' => &green_piece_texture,
        _ => &gray_piece_texture,
    };

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        // process controls
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Num4), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::Num1), .. } => {
                    match state {
                        GameState::Death => (),
                        _ => state = GameState::Play,
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::Num2), .. } => {
                    restart(&mut field, &mut piece, &mut score, &mut lines, &mut seconds, &mut preview_piece);
                    state = GameState::Play;
                },
                Event::KeyDown { keycode: Some(Keycode::Num3), .. } => {
                    match state {
                        GameState::Death => (),
                        _ => state = GameState::Menu,
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::Z), .. } => {
                    match state {
                        GameState::Play => piece.rotate(&field, -1),
                        _ => (),
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::X), .. } => {
                    match state {
                        GameState::Play => piece.rotate(&field, 1),
                        _ => (),
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                    match state {
                        GameState::Play => {
                            piece.move_x(-1, &field);
                        },
                        _ => (),
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                    match state {
                        GameState::Play => {
                            piece.move_x(1, &field);
                        },
                        _ => (),
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                    match state {
                        GameState::Play => frames_to_tick = 1,
                        _ => (),
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                    match state {
                        GameState::Play => {
                            piece.drop_down(&field);
                            frames_to_tick = 1;
                        },
                        _ => (),
                    }
                },
                _ => {}
            }
        }

        // clear canvas
        canvas.set_draw_color(bg_color);
        canvas.clear();

        match state {
            GameState::Menu => {
                write_tetris_by_textures(&mut canvas, &green_piece_texture);

                set_text(&mut canvas, &font, &texture_creator, hl_color, "1: play", Rect::new(7, 140, window_width - 14, 60));
                set_text(&mut canvas, &font, &texture_creator, fg_color, "2: restart", Rect::new(7, 140+60, window_width - 14, 60));
                set_text(&mut canvas, &font, &texture_creator, fg_color, "3: menu", Rect::new(7, 140+120, window_width - 14, 60));
                set_text(&mut canvas, &font, &texture_creator, fg_color, "4: exit", Rect::new(7, 140+180, window_width - 14, 60));

            },
            GameState::Death => {
                set_text(&mut canvas, &font, &texture_creator, at_color, "Death", Rect::new(7, 40, window_width - 14, 60));
                set_text(&mut canvas, &font, &texture_creator, fg_color, "level: 1", Rect::new((window_width / 2 - 80) as i32, 140+0*30, 140, 30));
                set_text(&mut canvas, &font, &texture_creator, fg_color, &format!("score: {}", score), Rect::new((window_width / 2 - 80) as i32, 140+1*30, 140, 30));
                set_text(&mut canvas, &font, &texture_creator, fg_color, &format!("lines: {}", lines), Rect::new((window_width / 2 - 80) as i32, 140+2*30, 140, 30));
                set_text(&mut canvas, &font, &texture_creator, fg_color, &format!("time: {:.1}", seconds), Rect::new((window_width / 2 - 80) as i32, 140+3*30, 140, 30));
                set_text(&mut canvas, &font, &texture_creator, hl_color, "2: restart", Rect::new(7, 300+60, window_width - 14, 60));
                set_text(&mut canvas, &font, &texture_creator, fg_color, "4: exit", Rect::new(7, 300+120, window_width - 14, 60));
            },
            GameState::Play => {
                // side panel
                canvas.set_draw_color(Color::RGB(30, 30, 30));
                canvas.fill_rect(Rect::new(7+10*30+7, 0, 30*4 + 7, window_height)).unwrap();

                canvas.copy(&grid_texture, None, Rect::new(7, 7, 30*10, 30*20));

                set_text(&mut canvas, &font, &texture_creator, fg_color, &format!("level: {}", level), Rect::new(7+10*30+7, 200+0*30, 4*30, 30));
                set_text(&mut canvas, &font, &texture_creator, fg_color, &format!("score: {}", score), Rect::new(7+10*30+7, 200+1*30, 4*30, 30));
                set_text(&mut canvas, &font, &texture_creator, fg_color, &format!("lines: {}", lines), Rect::new(7+10*30+7, 200+2*30, 4*30, 30));
                set_text(&mut canvas, &font, &texture_creator, fg_color, &format!("time: {:.1}", seconds), Rect::new(7+10*30+7, 200+3*30, 4*30, 30));

                let preview_body = preview_piece.body();
                for y in 0..4 {
                    for x in 0..4 {
                        if preview_body[y][x] != ' ' {
                            canvas.copy(get_texture(preview_body[y][x]), None, Some(Rect::new(7+(10 + x as i32) * 30+7, 7+(3 + y as i32)*30, 30, 30))).unwrap();
                        }
                    }
                }

                // game field
                level = lines / 30 + 1;
                tick_once_per_frames = 50 / level;

                if frames_to_tick <= 0 {
                    frames_to_tick = tick_once_per_frames;

                    // place piece if need
                    if !piece.is_move_down_awailable(&field) {
                        piece.put_on_a_field(&mut field, true);
                        piece = preview_piece;
                        preview_piece = Piece::new();
                    }

                    // remove filled lines
                    let mut filled_lines = 0;
                    for y in 0..field.len() {
                        let mut is_full = true;
                        for x in 0..field[y].len() {
                            if field[y][x] != 'N' {
                                is_full = false;
                                break;
                            }
                        }
                        if is_full {
                            for i in (1..y+1).rev() {
                                let temp = field[i];
                                field[i] = field[i-1];
                                field[i-1] = temp;
                            }
                            for i in 0..field[0].len() {
                                field[0][i] = ' ';
                            }
                            filled_lines += 1;
                        }
                    }

                    lines += filled_lines;
                    score += match filled_lines {
                        1 => 40,
                        2 => 100,
                        3 => 300,
                        4 => 1200,
                        _ => 0,
                    };

                    // die if reach top of game field
                    for x in 0..field[0].len() {
                        if field[0][x] == 'N' {
                            state = GameState::Death;
                        }
                    }

                    piece.force_move_y(1);
                } else {
                    frames_to_tick -= 1;
                }

                piece.put_on_a_field(&mut field, false);

                // disPlay field
                for y in 0..field.len() {
                    for x in 0..field[y].len() {
                        if field[y][x] == ' ' {
                            continue
                        }
                        let texture = get_texture(field[y][x]);

                        canvas.copy(texture, None, Some(Rect::new(7+x as i32 * 30, 7+y as i32 * 30, 30, 30))).unwrap();
                    }
                }

                seconds += 1./60.;
            },
        }


        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / framerate));
    }
}
