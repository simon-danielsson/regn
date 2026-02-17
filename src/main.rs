use std::{
    io::{self, Stdout, Write, stdout},
    thread,
    time::Duration,
};

use crossterm::{
    QueueableCommand,
    cursor::MoveTo,
    style::{Color, Print, SetForegroundColor},
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
const CLEAR_ANIM_FPS_DIV: i32 = 13;
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

#[derive(Clone)]
struct WeatherFrame {
    lines: Vec<String>,
    pos: Pos,
    height: u16,
    width: u16,
    border: Vec<char>,
}
impl WeatherFrame {
    fn new() -> Self {
        Self {
            lines: Vec::new(),
            pos: Pos::new_e(),
            height: 0,
            width: 0,
            border: vec!['╭', '─', '╮', '│', '╯', '╰'],
        }
    }

    fn find_optimal_width_and_height(&mut self) {
        let account_for_borders_w = 8;
        let account_for_borders_h = 4;
        let mut lines_sort_by_len = self.lines.clone();

        lines_sort_by_len.sort_by_key(|s| s.len());
        let longest = lines_sort_by_len.last().unwrap().chars().count();

        self.width = longest as u16 + account_for_borders_w;
        self.height = self.lines.len() as u16 + account_for_borders_h;
    }

    fn make_centered(&mut self, vp_cols: u16, vp_rows: u16) {
        let center_of_vp: (u16, u16) = (vp_cols / 2, vp_rows / 2);
        self.pos.col = center_of_vp.0 - (self.width / 2);
        self.pos.row = center_of_vp.1 - (self.height / 2);
    }
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
    fn new_e() -> Self {
        Self { col: 0, row: 0 }
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
    // rainy, snowy and clear weather
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
        for line in self.format_weather_data() {
            println!("{}", line);
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

    fn clear_animation(&mut self) -> io::Result<()> {
        let droplets_to_gen_each_frame: i32 = self.columns as i32 / 7;
        let max_amt_of_droplets: usize = self.columns as usize / 3;

        let drop_chars: Vec<&str> = vec![",", ".", "*"];
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
                    pos: Pos::new(rand_column, self.rows),
                    color: drop_colors[rand_color],
                    char: drop_chars[rand_char].to_string(),
                });
            }
        }

        // movement each frame
        // let col_mv_each_frame: u16 = 1; // left drift
        let row_mv_each_frame: u16 = 6; // fly up

        // move + draw
        for drop in self.precipitation.iter_mut() {
            // erase old position
            self.sout.queue(MoveTo(drop.pos.col, drop.pos.row))?;
            self.sout.write_all(b" ")?;

            // update position
            let rand_upward_speed = rng.random_range(..50);
            drop.pos.row = drop.pos.row.saturating_sub(
                row_mv_each_frame.saturating_sub(rand_upward_speed),
            );
            // drop.pos.col = drop.pos.col.saturating_sub(col_mv_each_frame);

            // if visible, draw
            if drop.pos.col < self.columns && drop.pos.row > 0 {
                let rand_char = rng.random_range(..drop_chars.len());
                drop.char = drop_chars[rand_char].to_string();
                self.sout.queue(SetForegroundColor(drop.color))?;
                self.sout.queue(MoveTo(drop.pos.col, drop.pos.row))?;
                self.sout.write_all(drop.char.as_bytes())?;
            }
            self.sout.queue(SetForegroundColor(Color::Reset))?;
        }

        // remove out-of-frame particles
        self.precipitation
            .retain(|d| d.pos.col < self.columns && d.pos.row > 0);

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

    fn format_weather_data(&mut self) -> Vec<String> {
        let mut s = Vec::new();

        s.push(format!("{time}", time = self.weather.location.localtime));

        s.push(format!(
            "{city}, {country}",
            city = self.weather.location.name,
            country = self.weather.location.country,
        ));

        s.push(format!(
            "{temp}°C, {cond}",
            temp = self.weather.current_temp_c,
            cond = self.weather.current_condition_as_str
        ));

        s.push(format!(
            "\n{}-Day Forecast:",
            self.weather.forecast_days.len()
        ));

        for day in self.weather.forecast_days.iter() {
            s.push(format!(
                "{}: {}°C / {}°C ({})",
                day.date,
                day.day.maxtemp_c,
                day.day.mintemp_c,
                day.day.condition.text.trim()
            ));
        }
        s
    }

    fn weather_frame(&mut self) -> io::Result<()> {
        let mut f = WeatherFrame::new();
        for line in self.format_weather_data() {
            f.lines.push(line);
        }
        f.find_optimal_width_and_height();
        f.make_centered(self.columns, self.rows);
        self.w_rect(&f)?;
        self.w_text(f)?;
        Ok(())
    }

    fn w_text(&mut self, f: WeatherFrame) -> io::Result<()> {
        let init_pos: Pos = Pos {
            col: f.pos.col + 4,
            row: f.pos.row + 2,
        };
        for (i, line) in f.lines.iter().enumerate() {
            self.sout
                .queue(MoveTo(init_pos.col, init_pos.row + i as u16))?;
            self.sout.write(line.as_bytes())?;
        }

        Ok(())
    }

    fn w_rect(&mut self, r: &WeatherFrame) -> io::Result<()> {
        // if nothing
        if r.width == 0 || r.height == 0 {
            return Ok(());
        }

        let x0 = r.pos.col;
        let y0 = r.pos.row;
        let w = r.width as u16;
        let h = r.height as u16;

        // 1x1: just a corner char (pick top-left)
        if w == 1 && h == 1 {
            self.sout.queue(MoveTo(x0, y0))?;
            self.sout.queue(Print(r.border[0]))?;
            return Ok(());
        }

        // repeat horizontal segment count times
        let horiz_len = w.saturating_sub(2) as usize;
        let horiz = r.border[1].to_string().repeat(horiz_len);

        // top row
        self.sout.queue(MoveTo(x0, y0))?;
        if w == 1 {
            self.sout.queue(Print(r.border[0]))?;
        } else {
            self.sout
                .queue(Print(r.border[0]))?
                .queue(Print(&horiz))?
                .queue(Print(r.border[2]))?;
        }

        // verticals
        let mid_rows = h.saturating_sub(2);
        for dy in 0..mid_rows {
            let yy = y0 + 1 + dy;
            self.sout.queue(MoveTo(x0, yy))?.queue(Print(r.border[3]))?;
            if w > 1 {
                self.sout
                    .queue(MoveTo(x0 + w - 1, yy))?
                    .queue(Print(r.border[3]))?;
            }
        }

        // bottom row
        if h > 1 {
            let yb = y0 + h - 1;
            self.sout.queue(MoveTo(x0, yb))?;
            if w == 1 {
                self.sout.queue(Print(r.border[5]))?;
            } else {
                self.sout
                    .queue(Print(r.border[5]))?
                    .queue(Print(&horiz))?
                    .queue(Print(r.border[4]))?;
            }
        }
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

            CurrentCondition::Clear => {
                if self.anim_frame_counter >= CLEAR_ANIM_FPS_DIV {
                    self.anim_frame_counter = 0;
                    self.clear_animation()?;
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
        self.weather_frame()?;

        Ok(())
    }
}
