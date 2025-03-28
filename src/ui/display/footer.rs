use miette::{IntoDiagnostic, Result};
use std::io::Write;

use crossterm::{
    cursor, execute,
    style::{Color, Print, ResetColor, SetForegroundColor, SetBackgroundColor},
    terminal::Clear, terminal::ClearType,
};

/// Renders the footer section of the UI
/// 
/// # Arguments
/// * `stdout` - The output stream to write to
/// * `current_row` - The current row position for the footer
/// * `terminal_width` - Width of the terminal
/// 
/// # Returns
/// A Result indicating success or an error
pub fn render_footer<W: Write>(
    stdout: &mut W, 
    current_row: u16, 
    _terminal_width: usize
) -> Result<()> {
    execute!(
        stdout,
        cursor::MoveTo(0, current_row + 1),
        SetBackgroundColor(Color::DarkBlue),
        Clear(ClearType::CurrentLine),
        SetForegroundColor(Color::White),
        Print(format!(" Lengths to be converted to points in future updates!")),
        ResetColor
    ).into_diagnostic()?;
    
    Ok(())
}
