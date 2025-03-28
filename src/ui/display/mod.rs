//! Display module for handling the game UI rendering

mod header;
mod swimmers;
mod footer;

use crate::swimmer::Swimmer;
use miette::{IntoDiagnostic, Result};
use std::io::{stdout, Write};
use terminal_size::{terminal_size, Height, Width};

use crossterm::{
    cursor, execute,
    terminal::{Clear, ClearType},
};

/// Displays the main game UI with all swimmers and game information
/// 
/// # Arguments
/// * `swimmers` - Slice of swimmer objects to display
/// * `selected_index` - Index of the currently selected swimmer
/// * `new_swimmer_cost` - Cost to add a new swimmer
/// 
/// # Returns
/// A Result indicating success or an error
pub fn display_ui(swimmers: &[Swimmer], selected_index: usize, new_swimmer_cost: usize) -> Result<()> {
    let mut stdout = stdout();
    
    // Ensure the terminal is completely cleared before each redraw
    execute!(
        stdout, 
        Clear(ClearType::All), 
        cursor::MoveTo(0, 0)
    ).into_diagnostic()?;
    
    // Hide cursor during rendering
    execute!(stdout, cursor::Hide).into_diagnostic()?;
    
    // Get terminal dimensions
    let terminal_dimensions =
        terminal_size().ok_or_else(|| miette::miette!("Failed to get terminal size"))?;
    let (Width(width), Height(_)) = terminal_dimensions;
    let terminal_width: usize = width as usize;

    // Define a consistent header height
    let header_height: u16 = 7; // Fixed header size

    // Render header section
    header::render_header(&mut stdout, terminal_width, swimmers, new_swimmer_cost)?;
    
    // Render all swimmers
    let current_row = swimmers::render_swimmers(&mut stdout, swimmers, selected_index, terminal_width, header_height)?;
    
    // Render footer
    footer::render_footer(&mut stdout, current_row, terminal_width)?;

    // Show cursor again
    execute!(stdout, cursor::Show).into_diagnostic()?;
    
    // Flush all changes to ensure immediate display
    stdout.flush().into_diagnostic()?;
    
    // Ensure the terminal is properly refreshed
    execute!(stdout, cursor::MoveTo(0, 0)).into_diagnostic()?;
    
    Ok(())
}
