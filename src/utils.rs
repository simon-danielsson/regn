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

    pub fn util_setup(&mut self) -> io::Result<()> {
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
