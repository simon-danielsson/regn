use std::{io, time::Duration};

use crossterm::{
    ExecutableCommand, QueueableCommand, cursor,
    terminal::{
        self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen,
        disable_raw_mode, enable_raw_mode,
    },
};

use crate::Regn;

impl Regn {
    pub fn util_clear_screen(&mut self) -> io::Result<()> {
        self.sout.queue(Clear(ClearType::All))?;
        Ok(())
    }

    /// helper: util_get_api_key
    pub fn util_get_key_file(&mut self) -> io::Result<String> {
        let h: String = home::home_dir().unwrap().display().to_string();
        let d: String = format!("{}/.regn", h);
        std::fs::read_to_string(d)
    }

    /// load api key and quit if it's not there
    pub fn util_get_api_key(&mut self) {
        match self.util_get_key_file() {
            Ok(key) => {
                let k: String = key.trim().to_string();
                if k.is_empty() {
                    panic!(
                    "ERROR: No API key was supplied in \"~/.regn\". Please add your accuweather key."
                )
                } else {
                    self.api_key = k;
                }
            }
            _ => {
                panic!(
                "ERROR: \".regn\" does not exist in your home directory. Create this file (\"~/.regn\") and supply your accuweather API key inside it."
            )
            }
        };
    }

    pub fn util_setup(&mut self) -> io::Result<()> {
        self.util_get_api_key();
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
