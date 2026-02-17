use std::time::Duration;

use crate::{CurrentCondition, ProgState, Regn};
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
                                if self.weather.current_condition
                                == CurrentCondition::Rain
                                {
                                    self.precipitation.clear();
                                    self.util_clear_screen()?;
                                    self.weather
                                        .current_condition = CurrentCondition::Snow;
                                } else {
                                    self.precipitation.clear();
                                    self.util_clear_screen()?;
                                    self.weather
                                        .current_condition = CurrentCondition::Rain;
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
