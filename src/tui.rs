use std::{
    io::{self, Stdout},
    time::Duration,
};

use anyhow::Result;
use ratatui::{self, prelude::CrosstermBackend, Terminal};

use crossterm::{
    event::{self, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use crate::{app::App, component::Component, event::AppEvent};

pub struct TuiApp {
    terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl TuiApp {
    pub fn build() -> io::Result<Self> {
        let terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;
        Ok(Self { terminal })
    }

    fn setup_terminal(&mut self) -> io::Result<()> {
        enable_raw_mode()?;
        execute!(
            self.terminal.backend_mut(),
            EnterAlternateScreen,
            EnableMouseCapture
        )?;
        Ok(())
    }
    fn restore_terminal(&mut self) -> io::Result<()> {
        disable_raw_mode()?;
        execute!(self.terminal.backend_mut(), LeaveAlternateScreen)?;
        self.terminal.show_cursor()
    }
    fn internal_run(&mut self) -> Result<()> {
        self.setup_terminal()?;
        let mut app = App::build()?;
        loop {
            self.terminal
                .draw(|frame| app.render(frame, frame.size(), &mut None))?;
            if event::poll(Duration::from_millis(250))? {
                if let Event::Key(key) = event::read()? {
                    if KeyModifiers::CONTROL == key.modifiers && KeyCode::Char('c') == key.code {
                        app.on_event(&AppEvent::Quit, &mut None);
                        break;
                    }
                    if KeyCode::Char('q') == key.code {
                        app.on_event(&AppEvent::Quit, &mut None);
                        break;
                    }
                    app.on_event(&AppEvent::Key(key), &mut None);
                }
            }
        }
        Ok(())
    }

    //Wrap the internal_run to restore the terminal if
    //an error ocurred
    pub fn run(mut self) -> Result<()> {
        let run_res = self.internal_run();
        self.restore_terminal()?;
        run_res
    }
}
