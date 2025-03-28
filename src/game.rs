use crate::swimmer::Swimmer;
use crate::ui;
use miette::{IntoDiagnostic, Result};
use std::thread;
use std::time::Duration;

use crossterm::{
    event::{self, read, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};

pub struct Game {
    swimmers: Vec<Swimmer>,
    selected_index: usize,
}

impl Game {
    pub fn new() -> Self {
        // Create swimmers with different swimming styles
        let swimmers = vec![
            Swimmer::new("Freestyle Swimmer", 1),
            Swimmer::new("Butterfly Swimmer", 2),
            Swimmer::new("Backstroke Swimmer", 1),
        ];

        Self {
            swimmers,
            selected_index: 0,
        }
    }

    pub fn run(&mut self) -> Result<()> {
        // Enable raw mode
        enable_raw_mode().into_diagnostic()?;

        // Show welcome message
        ui::show_welcome_message()?;
        thread::sleep(Duration::from_secs(3));

        let mut quit: bool = false;
        while !quit {
            // Update swimmers' progress
            for swimmer in &mut self.swimmers {
                swimmer.swim();
            }

            // Display the UI
            ui::display_ui(&self.swimmers, self.selected_index)?;

            // Handle user input
            if event::poll(Duration::from_millis(100)).into_diagnostic()? {
                if let Event::Key(event) = read().into_diagnostic()? {
                    match event.code {
                        KeyCode::Char('q') => {
                            quit = true;
                        }
                        KeyCode::Char(' ') => {
                            // Upgrade the selected swimmer
                            let swimmer = &mut self.swimmers[self.selected_index];
                            let success = swimmer.upgrade();
                            ui::show_upgrade_message(swimmer, success)?;
                            thread::sleep(Duration::from_millis(800));
                        }
                        KeyCode::Up => {
                            // Select previous swimmer
                            if self.selected_index > 0 {
                                self.selected_index -= 1;
                            }
                        }
                        KeyCode::Down => {
                            // Select next swimmer
                            if self.selected_index < self.swimmers.len() - 1 {
                                self.selected_index += 1;
                            }
                        }
                        _ => {}
                    }
                }
            }

            thread::sleep(Duration::from_millis(100));
        }

        // Show goodbye message
        ui::show_goodbye_message()?;

        // Wait for final keypress
        read().into_diagnostic()?;

        // Disable raw mode before exiting
        disable_raw_mode().into_diagnostic()?;
        Ok(())
    }
}
