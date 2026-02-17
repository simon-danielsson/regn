use std::{
    io::{self, Stdout, Write, stdout},
    thread,
    time::Duration,
};

use crossterm::{
    QueueableCommand,
    cursor::MoveTo,
    style::{Color, SetForegroundColor},
};
use rand::{RngExt, rng};

mod api;
mod arg;
mod controls;
mod help;
mod utils;

use crate::{
    api::api_main::{CurrentCondition, WeatherAPI},
    arg::{Arguments, parse_args},
    utils::get_fps,
};

const FPS: f64 = 60.0;
const RAIN_ANIM_FPS_DIV: i32 = 4;
const SNOW_ANIM_FPS_DIV: i32 = 13;

fn main() -> io::Result<()> {
    // get commandline argument launch
    let args: Arguments = parse_args();

    // fetch weather data from API
    let weather: WeatherAPI = api::api_main::api_main(&args.location, &args.forecast);

    let sout = stdout();
    let mut r = Regn::new(sout, weather, args);

    // if -t
    if r.args.no_tui {
        r.f_stdout_direct()?;
        return Ok(());
    }

    // if help
    if r.args.help {
        r.print_help();
        return Ok(());
    }

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

struct Precipitation {
    pos: Pos,
    color: Color,
    char: String,
}

#[derive(PartialEq)]
enum ProgState {
    Main,
    Quit,
}

struct Regn {
    columns: u16,
    rows: u16,
    sout: Stdout,
    weather: WeatherAPI,
    prog_state: ProgState,
    fps: Duration,
    args: Arguments,
    anim_frame_counter: i32,
    // rain and snow vector
    precipitation: Vec<Precipitation>,
}

impl Regn {
    fn new(sout: Stdout, weather: WeatherAPI, args: Arguments) -> Self {
        Self {
            sout,
            columns: 0,
            rows: 0,
            weather,
            args,
            prog_state: ProgState::Main,
            fps: get_fps(FPS),
            anim_frame_counter: 0,
            // rain_animation
            precipitation: Vec::new(),
        }
    }

    fn f_stdout_direct(&mut self) -> io::Result<()> {
        println!(
            "City: {}\nCountry: {}\nTime: {}",
            self.weather.location.name,
            self.weather.location.country,
            self.weather.location.localtime
        );

        println!("Current temp: {}°C", self.weather.current_temp_c);
        println!("Condition: {}", self.weather.current_condition_as_str);

        println!("\n{}-Day Forecast:", self.weather.forecast_days.len());
        for day in self.weather.forecast_days.iter() {
            println!(
                "{} → {}°C / {}°C ({})",
                day.date,
                day.day.maxtemp_c,
                day.day.mintemp_c,
                day.day.condition.text.trim()
            );
        }
        Ok(())
    }

    fn snow_animation(&mut self) -> io::Result<()> {
        let droplets_to_gen_each_frame: i32 = self.columns as i32 / 10;
        let max_amt_of_droplets: usize = self.columns as usize;
        let mut rng = rng();

        let drop_chars: Vec<&str> = vec!["*", "o", "."];

        let drop_colors: Vec<Color> =
        vec![Color::DarkGrey, Color::Grey, Color::Reset, Color::White];

        // generate droplets
        if self.precipitation.len() < max_amt_of_droplets {
            for _ in 0..=droplets_to_gen_each_frame.max(0) {
                let rand_column = rng.random_range(..self.columns);
                let rand_color = rng.random_range(..drop_colors.len());
                let rand_char = rng.random_range(..drop_chars.len());
                self.precipitation.push(Precipitation {
                    pos: Pos::new(rand_column, 0),
                    color: drop_colors[rand_color],
                    char: drop_chars[rand_char].to_string(),
                });
            }
        }

        for drop in self.precipitation.iter_mut() {
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
        self.precipitation
            .retain(|d| d.pos.col < self.columns && d.pos.row < self.rows);

        Ok(())
    }

    fn rain_animation(&mut self) -> io::Result<()> {
        let droplets_to_gen_each_frame: i32 = self.columns as i32 / 7;
        let max_amt_of_droplets: usize = self.columns as usize;

        let drop_chars: Vec<&str> = vec!["/", "."];
        let drop_colors: Vec<Color> =
        vec![Color::DarkGrey, Color::Grey, Color::Reset, Color::White];

        let mut rng = rng();

        // generate droplets
        if self.precipitation.len() < max_amt_of_droplets {
            for _ in 0..=droplets_to_gen_each_frame.max(0) {
                let rand_column = rng.random_range(..self.columns);
                let rand_color = rng.random_range(..drop_colors.len());
                let rand_char = rng.random_range(..drop_chars.len());
                self.precipitation.push(Precipitation {
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
        for drop in self.precipitation.iter_mut() {
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
        self.precipitation
            .retain(|d| d.pos.col < self.columns && d.pos.row < self.rows);

        Ok(())
    }

    /// todo
    fn cloud_animation(&mut self) -> io::Result<()> {
        self.util_clear_screen()?;
        Ok(())
    }

    /// todo
    fn sun_animation(&mut self) -> io::Result<()> {
        self.util_clear_screen()?;
        Ok(())
    }

    fn main_loop(&mut self) -> io::Result<()> {
        // weather animation
        match self.weather.current_condition {
            CurrentCondition::Rain => {
                if self.anim_frame_counter >= RAIN_ANIM_FPS_DIV {
                    self.anim_frame_counter = 0;
                    self.rain_animation()?;
                } else {
                    self.anim_frame_counter += 1;
                }
            }

            CurrentCondition::Snow => {
                if self.anim_frame_counter >= SNOW_ANIM_FPS_DIV {
                    self.anim_frame_counter = 0;
                    self.snow_animation()?;
                } else {
                    self.anim_frame_counter += 1;
                }
            }

            CurrentCondition::Sun => {
                self.sun_animation()?;
            }

            CurrentCondition::Cloud => {
                self.cloud_animation()?;
            }

            // add animations for thunder, clear and fog
            _ => {
                if self.anim_frame_counter >= RAIN_ANIM_FPS_DIV {
                    self.anim_frame_counter = 0;
                    self.rain_animation()?;
                } else {
                    self.anim_frame_counter += 1;
                }
            }
        }
        Ok(())
    }
}
