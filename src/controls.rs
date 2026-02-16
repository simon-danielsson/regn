use std::time::Duration;

use crate::{CurrentWeather, ProgState, Regn};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers, poll};

impl Regn {
    pub fn controls(&mut self) -> std::io::Result<()> {
        if poll(Duration::ZERO)? {
            match self.prog_state {
                ProgState::Main => {
                    if let Event::Key(KeyEvent {
                        code, modifiers, ..
                    }) = event::read()?
                    {
                        match (code, modifiers) {
                            // quit
                            (KeyCode::Esc, _) => {
                                self.prog_state = ProgState::Quit;
                            }

                            (KeyCode::Char('t'), _) => {
                                if self.weather.current
                                == CurrentWeather::Rain
                                {
                                    self.weather_particles
                                        .clear();
                                    self.util_clear_screen()?;
                                    self.weather
                                        .current = CurrentWeather::Snow;
                                } else {
                                    self.weather_particles
                                        .clear();
                                    self.util_clear_screen()?;
                                    self.weather
                                        .current = CurrentWeather::Rain;
                                }
                            }

                            (
                                KeyCode::Char('c'),
                                KeyModifiers::CONTROL,
                            ) => {
                                self.prog_state = ProgState::Quit;
                            }
                            _ => {}
                        }
                    }
                }

                ProgState::Quit => {}
            }
        }
        Ok(())
    }
}
