use crate::swimmer::Swimmer;
use crate::ui::utils::center_padding;
use miette::{IntoDiagnostic, Result};
use std::io::Write;

use crossterm::{
    cursor, execute,
    style::{Color, Print, ResetColor, SetForegroundColor, SetBackgroundColor, Attribute, SetAttribute},
    terminal::Clear, terminal::ClearType,
};

/// Renders the header section of the UI
/// 
/// # Arguments
/// * `stdout` - The output stream to write to
/// * `terminal_width` - Width of the terminal
/// * `swimmers` - Slice of swimmer objects for stats calculations
/// * `new_swimmer_cost` - Cost to add a new swimmer
/// 
/// # Returns
/// A Result indicating success or an error
pub fn render_header<W: Write>(
    stdout: &mut W, 
    terminal_width: usize, 
    swimmers: &[Swimmer],
    new_swimmer_cost: usize
) -> Result<()> {
    // Draw header background for the entire header area
    for y in 0..7_u16 {
        execute!(
            stdout,
            cursor::MoveTo(0, y),
            SetBackgroundColor(if y == 0 { Color::DarkBlue } else { Color::Reset }),
            Clear(ClearType::CurrentLine)
        ).into_diagnostic()?;
    }
    
    // Game title with enhanced styling - always on first line
    let title: &str = "🏊 Swimming Idle Game 🏊";
    let title_padding: usize = center_padding(title.len(), terminal_width);
    execute!(
        stdout,
        cursor::MoveTo(title_padding as u16, 0),
        SetForegroundColor(Color::White),
        SetAttribute(Attribute::Bold),
        Print(title),
        SetAttribute(Attribute::Reset),
        ResetColor
    ).into_diagnostic()?;
    
    // Reset background
    execute!(stdout, ResetColor).into_diagnostic()?;
    
    // Total lengths collected - always on line 2
    let total_lengths: i32 = swimmers.iter().map(|s| s.lengths).sum();
    let total_swimmers: usize = swimmers.len();
    
    execute!(
        stdout,
        cursor::MoveTo(1, 2),
        SetForegroundColor(Color::Cyan),
        Print(format!("Total Lengths: {}", total_lengths)),
        ResetColor
    ).into_diagnostic()?;
    
    // Swimmers count - always on line 2, right side
    let swimmers_text = format!("Swimmers: {}", total_swimmers);
    execute!(
        stdout,
        cursor::MoveTo((terminal_width as u16).saturating_sub(swimmers_text.len() as u16 + 1), 2),
        SetForegroundColor(Color::Cyan),
        Print(swimmers_text),
        ResetColor
    ).into_diagnostic()?;

    // Enhanced instructions with better formatting - always on line 3
    let instructions: &str = "Controls: [↑/↓] Select | [Space] Upgrade | [n] New Swimmer | [q] Quit";
    let instructions_padding: usize = center_padding(instructions.len(), terminal_width);
    execute!(
        stdout,
        cursor::MoveTo(instructions_padding as u16, 3),
        SetForegroundColor(Color::Yellow),
        Print(instructions),
        ResetColor
    ).into_diagnostic()?;
    
    // New swimmer cost with box styling - always on line 5
    let new_swimmer_info: String = format!("[ New Swimmer Cost: {} lengths ]", new_swimmer_cost);
    let info_padding: usize = center_padding(new_swimmer_info.len(), terminal_width);
    execute!(
        stdout,
        cursor::MoveTo(info_padding as u16, 5),
        SetForegroundColor(Color::Green),
        SetAttribute(Attribute::Bold),
        Print(&new_swimmer_info),
        SetAttribute(Attribute::Reset),
        ResetColor
    ).into_diagnostic()?;
    
    // Draw a separator line - always on line 6
    let separator: String = "─".repeat(terminal_width);
    execute!(
        stdout,
        cursor::MoveTo(0, 6),
        SetForegroundColor(Color::DarkGrey),
        Print(&separator),
        ResetColor
    ).into_diagnostic()?;
    
    Ok(())
}
