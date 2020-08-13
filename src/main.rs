extern crate rand;
extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};

use sdl2::hint::set_video_minimize_on_focus_lost;
use std::fs::File;
use std::io::{self, Read, Write};
use std::thread::sleep;
use std::time::{Duration, SystemTime};

const TETRIS_HEIGHT: usize = 40;
const HIGH_SCORE_FILE: &'static str = "scores.txt";
const LEVEL_TIMES: [u32; 10] = [1000, 850, 700, 600, 500, 400, 300, 250, 221, 190];
const LEVEL_LINES: [u32; 10] = [20, 40, 60, 80, 100, 120, 140, 160, 180, 200];
const NB_HIGH_SCORES: usize = 5;

type Piece = Vec<Vec<u8>>;
type States = Vec<Piece>;

trait TetriminoGenerator {
    fn new() -> Tetrimino;
}

struct TetriminoI;

impl TetriminoGenerator for TetriminoI {
    fn new() -> Tetrimino {
        Tetrimino {
            states: vec![
                vec![
                    vec![1, 1, 1, 1],
                    vec![0, 0, 0, 0],
                    vec![0, 0, 0, 0],
                    vec![0, 0, 0, 0],
                ],
                vec![
                    vec![0, 1, 0, 0],
                    vec![0, 1, 0, 0],
                    vec![0, 1, 0, 0],
                    vec![0, 1, 0, 0],
                ],
            ],
            x: 4,
            y: 0,
            current_state: 0,
        }
    }
}

struct TetriminoJ;

impl TetriminoGenerator for TetriminoJ {
    fn new() -> Tetrimino {
        Tetrimino {
            states: vec![
                vec![
                    vec![2, 2, 2, 0],
                    vec![2, 0, 0, 0],
                    vec![0, 0, 0, 0],
                    vec![0, 0, 0, 0],
                ],
                vec![
                    vec![2, 2, 0, 0],
                    vec![0, 2, 0, 0],
                    vec![0, 2, 0, 0],
                    vec![0, 0, 0, 0],
                ],
                vec![
                    vec![0, 0, 2, 0],
                    vec![2, 2, 2, 0],
                    vec![0, 0, 0, 0],
                    vec![0, 0, 0, 0],
                ],
                vec![
                    vec![2, 0, 0, 0],
                    vec![2, 0, 0, 0],
                    vec![2, 2, 0, 0],
                    vec![0, 0, 0, 0],
                ],
            ],
            x: 4,
            y: 0,
            current_state: 0,
        }
    }
}

struct TetriminoL;

impl TetriminoGenerator for TetriminoL {
    fn new() -> Tetrimino {
        Tetrimino {
            states: vec![
                vec![
                    vec![3, 3, 3, 0],
                    vec![0, 0, 3, 0],
                    vec![0, 0, 0, 0],
                    vec![0, 0, 0, 0],
                ],
                vec![
                    vec![0, 3, 0, 0],
                    vec![0, 3, 0, 0],
                    vec![3, 3, 0, 0],
                    vec![0, 0, 0, 0],
                ],
                vec![
                    vec![3, 0, 0, 0],
                    vec![3, 3, 3, 0],
                    vec![0, 0, 0, 0],
                    vec![0, 0, 0, 0],
                ],
                vec![
                    vec![3, 3, 0, 0],
                    vec![3, 0, 0, 0],
                    vec![3, 0, 0, 0],
                    vec![0, 0, 0, 0],
                ],
            ],
            x: 4,
            y: 0,
            current_state: 0,
        }
    }
}

struct TetriminoO;

impl TetriminoGenerator for TetriminoO {
    fn new() -> Tetrimino {
        Tetrimino {
            states: vec![vec![
                vec![4, 4, 0, 0],
                vec![4, 4, 0, 0],
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 0],
            ]],
            x: 5,
            y: 0,
            current_state: 0,
        }
    }
}

struct TetriminoS;

impl TetriminoGenerator for TetriminoS {
    fn new() -> Tetrimino {
        Tetrimino {
            states: vec![
                vec![
                    vec![0, 5, 5, 0],
                    vec![5, 5, 0, 0],
                    vec![0, 0, 0, 0],
                    vec![0, 0, 0, 0],
                ],
                vec![
                    vec![0, 5, 0, 0],
                    vec![0, 5, 5, 0],
                    vec![0, 0, 5, 0],
                    vec![0, 0, 0, 0],
                ],
            ],
            x: 4,
            y: 0,
            current_state: 0,
        }
    }
}

struct TetrimimoZ;

impl TetriminoGenerator for TetrimimoZ {
    fn new() -> Tetrimino {
        Tetrimino {
            states: vec![
                vec![
                    vec![6, 6, 0, 0],
                    vec![0, 6, 6, 0],
                    vec![0, 0, 0, 0],
                    vec![0, 0, 0, 0],
                ],
                vec![
                    vec![0, 0, 6, 0],
                    vec![0, 6, 6, 0],
                    vec![0, 6, 0, 0],
                    vec![0, 0, 0, 0],
                ],
            ],
            x: 4,
            y: 0,
            current_state: 0,
        }
    }
}

struct TetriminoT;

impl TetriminoGenerator for TetriminoT {
    fn new() -> Tetrimino {
        Tetrimino {
            states: vec![
                vec![
                    vec![7, 7, 7, 0],
                    vec![0, 7, 0, 0],
                    vec![0, 0, 0, 0],
                    vec![0, 0, 0, 0],
                ],
                vec![
                    vec![0, 7, 0, 0],
                    vec![7, 7, 0, 0],
                    vec![0, 7, 0, 0],
                    vec![0, 0, 0, 0],
                ],
                vec![
                    vec![0, 7, 0, 0],
                    vec![7, 7, 7, 0],
                    vec![0, 0, 0, 0],
                    vec![0, 0, 0, 0],
                ],
                vec![
                    vec![0, 7, 0, 0],
                    vec![0, 7, 7, 0],
                    vec![0, 7, 0, 0],
                    vec![0, 0, 0, 0],
                ],
            ],
            x: 4,
            y: 0,
            current_state: 0,
        }
    }
}

struct Tetrimino {
    states: States,
    x: isize,
    y: usize,
    current_state: u8,
}

impl Tetrimino {
    fn rotate(&mut self, game_map: &[Vec<u8>]) {
        let mut tmp_state = self.current_state + 1;
        if tmp_state as usize >= self.states.len() {
            tmp_state = 0;
        }
        let x_pos = [0, -1, 1, -2, 2, -3];
        for x in x_pos.iter() {
            if self.test_position(game_map, tmp_state as usize, self.x + x, self.y) == true {
                self.current_state = tmp_state;
                self.x += *x;
                break;
            }
        }
    }

    fn test_position(&self, game_map: &[Vec<u8>], tmp_state: usize, x: isize, y: usize) -> bool {
        for shift_y in 0..4 {
            for shift_x in 0..4 {
                let x = x + shift_x;
                if self.states[tmp_state][shift_y][shift_x as usize] != 0
                    && (y + shift_y >= game_map.len()
                        || x < 0
                        || x as usize >= game_map[y + shift_y].len()
                        || game_map[y + shift_y][x as usize] != 0)
                {
                    return false;
                }
            }
        }
        return true;
    }

    fn test_current_position(&self, game_map: &[Vec<u8>]) -> bool {
        self.test_position(game_map, self.current_state as usize, self.x, self.y)
    }

    fn change_position(&mut self, game_map: &[Vec<u8>], new_x: isize, new_y: usize) -> bool {
        if self.test_position(game_map, self.current_state as usize, new_x, new_y) == true {
            self.x = new_x;
            self.y = new_y;
            true
        } else {
            false
        }
    }
}

struct Tetris {
    game_map: Vec<Vec<u8>>,
    current_level: u32,
    score: u32,
    nb_lines: u32,
    current_piece: Option<Tetrimino>,
}

impl Tetris {
    fn new() -> Tetris {
        let mut game_map = Vec::new();
        for _ in 0..16 {
            game_map.push(vec![0, 0, 0, 0, 0, 0, 0, 0, 0]);
        }
        Tetris {
            game_map,
            current_level: 1,
            score: 0,
            nb_lines: 0,
            current_piece: None,
        }
    }

    fn update_score(&mut self, to_add: u32) {
        self.score += to_add;
    }

    fn increase_level(&mut self) {
        self.current_level += 1;
    }

    fn increase_line(&mut self) {
        self.nb_lines += 1;
        if self.nb_lines > LEVEL_LINES[self.current_level as usize - 1] {
            self.increase_level();
        }
    }

    fn check_lines(&mut self) {
        let mut y = 0;
        let mut score_add = 0;

        while y < self.game_map.len() {
            let mut complete = true;

            for x in &self.game_map[y] {
                if *x == 0 {
                    complete = false;
                    break;
                }
            }
            if complete == true {
                score_add += self.current_level;
                self.game_map.remove(y);
                y -= 1;
            }
            y += 1;
        }
        if self.game_map.len() == 0 {
            score_add += 1000;
        }
        self.update_score(score_add);
        while self.game_map.len() < 16 {
            self.increase_line();
            self.game_map.insert(0, vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        }
    }
    fn create_new_tetrimino(&self) -> Tetrimino {
        static mut PREV: u8 = 7;
        let mut rand_nb = rand::random::<u8>() % 7;
        if unsafe { PREV } == rand_nb {
            rand_nb = rand::random::<u8>() % 7;
        }
        unsafe {
            PREV = rand_nb;
        }
        match rand_nb {
            0 => TetriminoI::new(),
            1 => TetriminoJ::new(),
            2 => TetriminoL::new(),
            3 => TetriminoO::new(),
            4 => TetriminoS::new(),
            5 => TetrimimoZ::new(),
            6 => TetriminoT::new(),
            _ => unreachable!(),
        }
    }

    fn make_permanent(&mut self) {
        let mut to_add = 0;
        if let Some(ref mut piece) = self.current_piece {
            let mut shift_y = 0;

            while shift_y < piece.states[piece.current_state as usize].len()
                && piece.y + shift_y < self.game_map.len()
            {
                let mut shift_x = 0;
                while shift_x < piece.states[piece.current_state as usize][shift_y].len()
                    && (piece.x + shift_x as isize)
                        < self.game_map[piece.y + shift_y].len() as isize
                {
                    if piece.states[piece.current_state as usize][shift_y][shift_x] != 0 {
                        let x = piece.x + shift_x as isize;
                        self.game_map[piece.y + shift_y][x as usize] =
                            piece.states[piece.current_state as usize][shift_y][shift_x];
                    }
                    shift_x += 1;
                }
                shift_y += 1;
            }
            to_add += self.current_level;
        }
        self.update_score(to_add);
        self.check_lines();
        self.current_piece = None;
    }
}

fn handle_events(
    tetris: &mut Tetris,
    quit: &mut bool,
    timer: &mut SystemTime,
    event_pump: &mut sdl2::EventPump,
) -> bool {
    let mut make_permanent = false;
    if let Some(ref mut piece) = tetris.current_piece {
        let mut tmp_x = piece.x;
        let mut tmp_y = piece.y;

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    *quit = true;
                    break;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    *timer = SystemTime::now();
                    tmp_y += 1;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    tmp_x += 1;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    tmp_x -= 1;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    piece.rotate(&tetris.game_map);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => {
                    let x = piece.x;
                    let mut y = piece.y;
                    while piece.change_position(&tetris.game_map, x, y + 1) == true {
                        y += 1;
                    }
                    make_permanent = true;
                }
                _ => {}
            }
        }
        if !make_permanent {
            if piece.change_position(&tetris.game_map, tmp_x, tmp_y) == false && tmp_y != piece.y {
                make_permanent = true;
            }
        }
    }
    if make_permanent {
        tetris.make_permanent();
        *timer = SystemTime::now();
    }
    make_permanent
}

fn write_into_file(content: &str, file_name: &str) -> io::Result<()> {
    let mut f = File::create(file_name)?;
    f.write_all(content.as_bytes())
}

fn read_from_file(file_name: &str) -> io::Result<String> {
    let mut f = File::open(file_name)?;
    let mut content = String::new();
    f.read_to_string(&mut content)?;
    Ok(content)
}

fn slice_to_string(slice: &[u32]) -> String {
    slice
        .iter()
        .map(|high_score| high_score.to_string())
        .collect::<Vec<String>>()
        .join(" ")
}

fn save_high_scores_and_lines(high_scores: &[u32], number_of_lines: &[u32]) -> bool {
    let s_high_scores = slice_to_string(high_scores);
    let s_number_of_lines = slice_to_string(number_of_lines);
    write_into_file(
        &format!("{}\n{}\n", s_high_scores, s_number_of_lines),
        HIGH_SCORE_FILE,
    )
    .is_ok()
}

fn line_to_slice(line: &str) -> Vec<u32> {
    line.split(" ")
        .filter_map(|nb| nb.parse::<u32>().ok())
        .collect()
}

fn load_high_scores_and_lines() -> Option<(Vec<u32>, Vec<u32>)> {
    if let Ok(content) = read_from_file(HIGH_SCORE_FILE) {
        let mut lines = content
            .splitn(2, "\n")
            .map(|line| line_to_slice(line))
            .collect::<Vec<_>>();
        if lines.len() == 2 {
            let (lines_sent, high_scores) = (lines.pop().unwrap(), lines.pop().unwrap());
            Some((high_scores, lines_sent))
        } else {
            None
        }
    } else {
        None
    }
}

fn update_vec(v: &mut Vec<u32>, value: u32) -> bool {
    if v.len() < NB_HIGH_SCORES {
        v.push(value);
        true
    } else {
        for entry in v.iter_mut() {
            if value > *entry {
                *entry = value;
                return true;
            }
        }
        false
    }
}
fn print_game_information(tetris: &Tetris) {
    let mut new_highest_high_score = true;
    let mut new_highest_lines_sent = true;
    if let Some((mut high_scores, mut lines_sent)) = load_high_scores_and_lines() {
        new_highest_high_score = update_vec(&mut high_scores, tetris.score);
        new_highest_lines_sent = update_vec(&mut lines_sent, tetris.score);
        if new_highest_high_score || new_highest_lines_sent {
            save_high_scores_and_lines(&high_scores, &lines_sent);
        }
    } else {
        save_high_scores_and_lines(&[tetris.score], &[tetris.nb_lines]);
    }
    println!("Game over..");
    println!(
        "Score:      {}{}",
        tetris.score,
        if new_highest_high_score {
            "[NEW HIGH SCORE]"
        } else {
            ""
        }
    );
    println!(
        "Number of lines: {}{}",
        tetris.nb_lines,
        if new_highest_lines_sent {
            "[NEW HIGH SCORE]"
        } else {
            ""
        }
    );
    println!("Current level: {}", tetris.current_level);
}

fn is_time_over(tetris: &Tetris, timer: &SystemTime) -> bool {
    match timer.elapsed() {
        Ok(elapsed) => {
            let millis = elapsed.as_secs() as u32 * 1000 + elapsed.subsec_nanos() / 1_000_000;
            millis > LEVEL_TIMES[tetris.current_level as usize - 1]
        }
        Err(_) => false,
    }
}

fn main() {
    let sdl_context = sdl2::init().expect("SDL initialization failed");
    let video_subsystem = sdl_context
        .video()
        .expect("Couldn't get SDL video subsystem");
    let width = 600;
    let height = 800;
    let mut tetris = Tetris::new();
    let mut timer = SystemTime::now();

    let mut event_pump = sdl_context
        .event_pump()
        .expect("Failed to get SDL event pump");

    let grid_x = (width - TETRIS_HEIGHT as u32 * 10) as i32 / 2;
    let grid_y = (height - TETRIS_HEIGHT as u32 * 16) as i32 / 2;

    let window = video_subsystem
        .window("Tetris", width, height)
        .position_centered()
        .build()
        .expect("Failed to create window");

    let mut canvas = window
        .into_canvas()
        .target_texture()
        .present_vsync()
        .build()
        .expect("Couldn't get window's canvas");

    let texture_creator: TextureCreator<_> = canvas.texture_creator();
    let grid = create_texture_rect(
        &mut canvas,
        &texture_creator,
        0,
        0,
        0,
        TETRIS_HEIGHT as u32 + 10,
        TETRIS_HEIGHT as u32 * 16,
    )
    .expect("Failed to create a texture");

    let border = create_texture_rect(
        &mut canvas,
        &texture_creator,
        255,
        255,
        255,
        TETRIS_HEIGHT as u32 * 10 + 20,
        TETRIS_HEIGHT as u32 * 16 + 20,
    )
    .expect("Failed to create a texture");

    macro_rules! texture {
        ($r:expr, $g:expr, $b:expr) => {
            create_texture_rect(
                &mut canvas,
                &texture_creator,
                $r,
                $g,
                $b,
                TETRIS_HEIGHT as u32,
                TETRIS_HEIGHT as u32,
            )
            .unwrap()
        };
    }
    let textures = [
        texture!(255, 69, 69),
        texture!(255, 220, 69),
        texture!(237, 150, 37),
        texture!(171, 99, 237),
        texture!(77, 159, 239),
        texture!(39, 218, 225),
        texture!(45, 216, 47),
    ];

    loop {
        if is_time_over(&tetris, &timer) {
            let mut make_permanent = false;
            if let Some(ref mut piece) = tetris.current_piece {
                let x = piece.x;
                let y = piece.y + 1;
                make_permanent = !piece.change_position(&tetris.game_map, x, y);
            }
            if make_permanent {
                tetris.make_permanent();
            }
            timer = SystemTime::now();
        }

        canvas.set_draw_color(Color::RGB(255, 0, 0));
        canvas.clear();

        canvas
            .copy(
                &border,
                None,
                Rect::new(
                    (width - TETRIS_HEIGHT as u32 * 10) as i32 / 2 - 10,
                    (height - TETRIS_HEIGHT as u32 * 16) as i32 / 2 * 10,
                    TETRIS_HEIGHT as u32 * 10 + 20,
                    TETRIS_HEIGHT as u32 * 16 + 20,
                ),
            )
            .expect("Couldn't copy texture into window");
        canvas
            .copy(
                &grid,
                None,
                Rect::new(
                    (width - TETRIS_HEIGHT as u32 * 10) as i32 / 2,
                    (height - TETRIS_HEIGHT as u32 * 16) as i32 / 2,
                    TETRIS_HEIGHT as u32 * 10,
                    TETRIS_HEIGHT as u32 * 16,
                ),
            )
            .expect("Couldn't copy texture into new window");

        if tetris.current_piece.is_none() {
            let current_piece = tetris.create_new_tetrimino();
            if !current_piece.test_current_position(&tetris.game_map) {
                print_game_information(&tetris);
                break;
            }
            tetris.current_piece = Some(current_piece);
        }
        let mut quit = false;
        if !handle_events(&mut tetris, &mut quit, &mut timer, &mut event_pump) {
            if let Some(ref mut piece) = tetris.current_piece {
                for (line_nb, line) in piece.states[piece.current_state as usize]
                    .iter()
                    .enumerate()
                {
                    for (case_nb, case) in line.iter().enumerate() {
                        if *case == 0 {
                            continue;
                        }
                        canvas
                            .copy(
                                &textures[*case as usize - 1],
                                None,
                                Rect::new(
                                    grid_x
                                        + (piece.x + case_nb as isize) as i32
                                            * TETRIS_HEIGHT as i32,
                                    grid_y + (piece.y + line_nb) as i32 + TETRIS_HEIGHT as i32,
                                    TETRIS_HEIGHT as u32,
                                    TETRIS_HEIGHT as u32,
                                ),
                            )
                            .expect("Couldn't copy texture into window");
                    }
                }
            }
        }
        if quit {
            print_game_information(&tetris);
            break;
        }
        for (line_nb, line) in tetris.game_map.iter().enumerate() {
            for (case_nb, case) in line.iter().enumerate() {
                if *case == 0 {
                    continue;
                }
                canvas
                    .copy(
                        &textures[*case as usize - 1],
                        None,
                        Rect::new(
                            grid_x + case_nb as i32 * TETRIS_HEIGHT as i32,
                            grid_y + line_nb as i32 * TETRIS_HEIGHT as i32,
                            TETRIS_HEIGHT as u32,
                            TETRIS_HEIGHT as u32,
                        ),
                    )
                    .expect("Couldn't copy texture into window");
            }
        }
        canvas.present();
        sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn create_texture_rect<'a>(
    canvas: &mut Canvas<Window>,
    texture_creator: &'a TextureCreator<WindowContext>,
    color_r: u8,
    color_g: u8,
    color_b: u8,
    width: u32,
    height: u32,
) -> Option<Texture<'a>> {
    if let Ok(mut square_texture) = texture_creator.create_texture_target(None, width, height) {
        canvas
            .with_texture_canvas(&mut square_texture, |texture| {
                texture.set_draw_color(Color::RGB(color_r, color_g, color_b));
                texture.clear();
            })
            .expect("Failed to color a texture");
        Some(square_texture)
    } else {
        None
    }
}
