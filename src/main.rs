use std::{
    io::{self, Stdout, Write, stdout},
    thread,
    time::Duration,
};

use crossterm::{
    ExecutableCommand, QueueableCommand,
    cursor::{self, MoveTo},
    style::{Color, SetForegroundColor},
    terminal::{
        self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen,
        disable_raw_mode, enable_raw_mode,
    },
};
use rand::{RngExt, rng};

mod controls;

const FPS: f64 = 60.0;
const RAIN_ANIM_FPS_DIV: i32 = 4;
const SNOW_ANIM_FPS_DIV: i32 = 13;

fn main() -> io::Result<()> {
    let sout = stdout();
    let mut r = Regn::new(sout);

    r.util_setup()?;

    while r.prog_state != ProgState::Quit {
        r.controls()?;
        r.main_loop()?;
        r.sout.flush()?;
        thread::sleep(r.fps);
    }

    r.util_quit()?;

    Ok(())
}

#[derive(Clone, PartialEq)]
struct Pos {
    col: u16,
    row: u16,
}
impl Pos {
    fn new(col: u16, row: u16) -> Self {
        Self { col, row }
    }
}

struct WeatherParticle {
    pos: Pos,
    color: Color,
    char: String,
}

#[derive(PartialEq)]
enum ProgState {
    Main,
    Quit,
}

#[derive(PartialEq)]
enum CurrentWeather {
    Rain,
    Snow,
}

struct Regn {
    columns: u16,
    rows: u16,
    sout: Stdout,
    current_weather: CurrentWeather,
    prog_state: ProgState,
    fps: Duration,
    anim_frame_counter: i32,
    api_key: String,
    // rain_animation
    weather_particles: Vec<WeatherParticle>,
}

impl Regn {
    fn new(sout: Stdout) -> Self {
        Self {
            sout,
            columns: 0,
            rows: 0,
            current_weather: CurrentWeather::Rain,
            prog_state: ProgState::Main,
            fps: get_fps(FPS),
            anim_frame_counter: 0,
            api_key: String::new(),
            // rain_animation
            weather_particles: Vec::new(),
        }
    }

    fn snow_animation(&mut self) -> io::Result<()> {
        let droplets_to_gen_each_frame: i32 = self.columns as i32 / 10;
        let max_amt_of_droplets: usize = self.columns as usize;
        let mut rng = rng();

        let drop_chars: Vec<&str> = vec!["*", "o", "."];

        let drop_colors: Vec<Color> = vec![
            Color::DarkGrey,
            Color::Grey,
            Color::Reset,
            Color::Black,
            Color::White,
        ];

        // generate droplets
        if self.weather_particles.len() < max_amt_of_droplets {
            for _ in 0..=droplets_to_gen_each_frame.max(0) {
                let rand_column = rng.random_range(..self.columns);
                let rand_color = rng.random_range(..drop_colors.len());
                let rand_char = rng.random_range(..drop_chars.len());
                self.weather_particles.push(WeatherParticle {
                    pos: Pos::new(rand_column, 0),
                    color: drop_colors[rand_color],
                    char: drop_chars[rand_char].to_string(),
                });
            }
        }

        for drop in self.weather_particles.iter_mut() {
            // erase old position
            self.sout.queue(MoveTo(drop.pos.col, drop.pos.row))?;
            self.sout.write_all(b" ")?;

            // update position
            // (random left to right)
            let new_direction = rng.random_bool(1.0 / 2.0);
            if new_direction {
                match rng.random_bool(1.0 / 2.0) {
                    true => drop.pos.col.saturating_add(1),
                    false => drop.pos.col.saturating_sub(1),
                };
            }

            let rand_downward_speed = rng.random_range(0..10);
            drop.pos.row = drop.pos.row.saturating_add(rand_downward_speed);

            // if visible, draw
            if drop.pos.col < self.columns && drop.pos.row < self.rows {
                self.sout.queue(SetForegroundColor(drop.color))?;
                self.sout.queue(MoveTo(drop.pos.col, drop.pos.row))?;
                self.sout.write_all(drop.char.as_bytes())?;
            }
            self.sout.queue(SetForegroundColor(Color::Reset))?;
        }

        // remove out-of-frame particles
        self.weather_particles
            .retain(|d| d.pos.col < self.columns && d.pos.row < self.rows);

        Ok(())
    }

    fn rain_animation(&mut self) -> io::Result<()> {
        let droplets_to_gen_each_frame: i32 = self.columns as i32 / 7;
        let max_amt_of_droplets: usize = self.columns as usize;

        let drop_chars: Vec<&str> = vec!["/", "."];
        let drop_colors: Vec<Color> = vec![
            Color::DarkGrey,
            Color::Grey,
            Color::Reset,
            Color::Black,
            Color::White,
        ];

        let mut rng = rng();

        // generate droplets
        if self.weather_particles.len() < max_amt_of_droplets {
            for _ in 0..=droplets_to_gen_each_frame.max(0) {
                let rand_column = rng.random_range(..self.columns);
                let rand_color = rng.random_range(..drop_colors.len());
                let rand_char = rng.random_range(..drop_chars.len());
                self.weather_particles.push(WeatherParticle {
                    pos: Pos::new(rand_column, 0),
                    color: drop_colors[rand_color],
                    char: drop_chars[rand_char].to_string(),
                });
            }
        }

        // movement each frame
        let col_mv_each_frame: u16 = 1; // left drift
        let row_mv_each_frame: u16 = 1; // fall down

        // move + draw
        for drop in self.weather_particles.iter_mut() {
            // erase old position
            self.sout.queue(MoveTo(drop.pos.col, drop.pos.row))?;
            self.sout.write_all(b" ")?;

            // update position
            let rand_downward_speed = rng.random_range(0..10);
            drop.pos.row =
                drop.pos.row
                    .saturating_add(row_mv_each_frame + rand_downward_speed);
            drop.pos.col = drop.pos.col.saturating_sub(col_mv_each_frame);

            // if visible, draw
            if drop.pos.col < self.columns && drop.pos.row < self.rows {
                self.sout.queue(SetForegroundColor(drop.color))?;
                self.sout.queue(MoveTo(drop.pos.col, drop.pos.row))?;
                self.sout.write_all(drop.char.as_bytes())?;
            }
            self.sout.queue(SetForegroundColor(Color::Reset))?;
        }

        // remove out-of-frame particles
        self.weather_particles
            .retain(|d| d.pos.col < self.columns && d.pos.row < self.rows);

        Ok(())
    }

    fn cloud_animation(&mut self) -> io::Result<()> {
        Ok(())
    }

    fn sun_animation(&mut self) -> io::Result<()> {
        Ok(())
    }

    fn main_loop(&mut self) -> io::Result<()> {
        // weather animation
        match self.current_weather {
            CurrentWeather::Rain => {
                if self.anim_frame_counter >= RAIN_ANIM_FPS_DIV {
                    self.anim_frame_counter = 0;
                    self.rain_animation()?;
                } else {
                    self.anim_frame_counter += 1;
                }
            }
            CurrentWeather::Snow => {
                if self.anim_frame_counter >= SNOW_ANIM_FPS_DIV {
                    self.anim_frame_counter = 0;
                    self.snow_animation()?;
                } else {
                    self.anim_frame_counter += 1;
                }
            }
        }
        Ok(())
    }

    fn util_clear_screen(&mut self) -> io::Result<()> {
        self.sout.queue(Clear(ClearType::All))?;
        Ok(())
    }

    /// accuweather api key
    fn load_api_key(&mut self) -> io::Result<String> {
        std::fs::read_to_string("src/key.txt")
    }

    fn util_setup(&mut self) -> io::Result<()> {
        // load api key and quit if it's not there
        match self.load_api_key() {
            Ok(key) => {
                let k: String = key.trim().to_string();
                if k.is_empty() {
                    panic!(
                    "ERROR: No API key was supplied in \"key.txt\". Please add your accuweather key."
                )
                } else {
                    self.api_key = k;
                }
            }
            _ => {
                panic!(
                "ERROR: \"key.txt\" does not exist in src directory. Create this file and add your accuweather API key inside it."
            )
            }
        };

        enable_raw_mode()?;
        (self.columns, self.rows) = terminal::size()?;
        self.sout.execute(EnterAlternateScreen)?;
        self.sout.queue(cursor::SavePosition)?;
        self.sout.queue(cursor::Hide)?;
        Ok(())
    }

    pub fn util_quit(&mut self) -> io::Result<()> {
        disable_raw_mode()?;
        self.sout.execute(LeaveAlternateScreen)?;
        self.sout.queue(cursor::RestorePosition)?;
        self.sout.queue(cursor::Show)?;
        Ok(())
    }
}

pub fn get_fps(fps: f64) -> Duration {
    Duration::from_secs_f64(1.0 / fps)
}
