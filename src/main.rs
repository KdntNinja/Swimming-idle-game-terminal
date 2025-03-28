mod game;
mod swimmer;
mod ui;
mod utils;

use game::Game;
use miette::Result;

/// The entry point of the application
///
/// # Returns
/// A Result indicating success or failure
fn main() -> Result<()> {
    let mut game: Game = Game::new()?;
    game.run()?;
    Ok(())
}
