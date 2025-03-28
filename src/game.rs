use crate::swimmer::Swimmer;
use crate::ui;
use crate::utils::{generate_random_name, load_name_data, NameData};
use miette::{IntoDiagnostic, Result};
use std::thread;
use std::time::{Duration, Instant};

use crossterm::{
    event::{self, read, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};

pub struct Game {
    swimmers: Vec<Swimmer>,
    selected_index: usize,
    name_data: NameData,
    new_swimmer_cost: usize,
}

impl Game {
    /// Creates a new game with initial state
    ///
    /// # Returns
    /// A Result containing the new Game or an error
    pub fn new() -> Result<Self> {
        // Load name data
        let name_data: NameData = load_name_data()?;

        // Generate a random name for the first swimmer
        let first_name: String = generate_random_name(&name_data);

        // Start with just one swimmer with reduced base speed of 0.7
        let swimmers: Vec<Swimmer> = vec![Swimmer::new(&first_name, 0.7_f64)];

        Ok(Self {
            swimmers,
            selected_index: 0_usize,
            name_data,
            new_swimmer_cost: 25_usize, // Initial cost to add a new swimmer
        })
    }

    /// Adds a new swimmer if player has enough lengths
    ///
    /// # Returns
    /// `true` if successful, `false` otherwise
    fn add_new_swimmer(&mut self) -> bool {
        // Check if player has enough lengths to add a new swimmer
        let total_lengths: i32 = self.swimmers.iter().map(|s: &Swimmer| s.lengths).sum();

        if total_lengths >= self.new_swimmer_cost as i32 {
            // Deduct the cost from the first swimmer (could distribute this differently)
            self.swimmers[0].lengths -= self.new_swimmer_cost as i32;

            // Generate a random name
            let new_name: String = generate_random_name(&self.name_data);

            // Create a new swimmer with slower base speed of 0.7 (changed from 1)
            let mut new_swimmer: Swimmer = Swimmer::new(&new_name, 0.7_f64);

            // Set a basic upgrade cost
            new_swimmer.upgrade_cost = 10_usize;

            // Add the new swimmer
            self.swimmers.push(new_swimmer);

            // Increase the cost for the next swimmer
            self.new_swimmer_cost = (self.new_swimmer_cost as f64 * 1.5_f64) as usize;

            return true;
        }

        false
    }

    /// Runs the main game loop
    ///
    /// # Returns
    /// A Result indicating success or failure
    pub fn run(&mut self) -> Result<()> {
        // Enable raw mode
        enable_raw_mode().into_diagnostic()?;

        // Display the initial UI before entering the game loop
        ui::display_ui(&self.swimmers, self.selected_index, self.new_swimmer_cost)?;

        let mut quit: bool = false;
        // Use a fixed time step for smoother animation
        let frame_duration: Duration = Duration::from_millis(33_u64); // ~30 FPS
        let mut last_update: Instant = Instant::now();

        // Use a fixed time step for rendering to avoid excessive updates
        let render_duration: Duration = Duration::from_millis(100_u64); // ~10 FPS
        let mut last_render: Instant = Instant::now();

        while !quit {
            let now: Instant = Instant::now();
            let elapsed: Duration = now.duration_since(last_update);

            // Only update the game state at fixed intervals
            if elapsed >= frame_duration {
                for swimmer in &mut self.swimmers {
                    swimmer.swim();
                }
                last_update = now;
            }

            // Only render the UI at fixed intervals
            if now.duration_since(last_render) >= render_duration {
                ui::display_ui(&self.swimmers, self.selected_index, self.new_swimmer_cost)?;
                last_render = now;
            }

            // Handle user input with a shorter timeout to be responsive
            if event::poll(Duration::from_millis(10_u64)).into_diagnostic()? {
                if let Event::Key(event) = read().into_diagnostic()? {
                    match event.code {
                        KeyCode::Char('q') => {
                            quit = true;
                        }
                        KeyCode::Char(' ') => {
                            let swimmer: &mut Swimmer = &mut self.swimmers[self.selected_index];
                            let success: bool = swimmer.upgrade();
                            ui::show_upgrade_message(swimmer, success)?;
                            thread::sleep(Duration::from_millis(800_u64));
                        }
                        KeyCode::Char('n') => {
                            let success: bool = self.add_new_swimmer();
                            if success {
                                ui::show_new_swimmer_message(
                                    &self.swimmers.last().unwrap(),
                                    true,
                                    None,
                                )?;
                            } else {
                                ui::show_new_swimmer_message(
                                    &self.swimmers[0],
                                    false,
                                    Some(self.new_swimmer_cost),
                                )?;
                            }
                            thread::sleep(Duration::from_millis(800_u64));
                        }
                        KeyCode::Up => {
                            if self.selected_index > 0_usize {
                                self.selected_index -= 1_usize;
                            }
                        }
                        KeyCode::Down => {
                            if self.selected_index < self.swimmers.len() - 1_usize {
                                self.selected_index += 1_usize;
                            }
                        }
                        _ => {}
                    }
                }
            } else {
                thread::sleep(Duration::from_millis(1_u64));
            }
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
