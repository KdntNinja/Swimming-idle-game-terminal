mod game;
mod swimmer;
mod ui;

use game::Game;
use miette::Result;

fn main() -> Result<()> {
    let mut game = Game::new();
    game.run()?;
    Ok(())
}
