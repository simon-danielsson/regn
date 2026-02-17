use std::time::Duration;

use crate::{ProgState, Regn};
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
