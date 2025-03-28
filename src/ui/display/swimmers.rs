use crate::swimmer::Swimmer;
use crate::ui::utils::center_padding;
use miette::{IntoDiagnostic, Result};
use std::io::Write;

use crossterm::{
    cursor, execute,
    style::{Color, Print, ResetColor, SetForegroundColor, SetBackgroundColor, Attribute, SetAttribute},
};

/// Renders all swimmers with their lanes and indicators
/// 
/// # Arguments
/// * `stdout` - The output stream to write to
/// * `swimmers` - Slice of swimmer objects to display
/// * `selected_index` - Index of the currently selected swimmer
/// * `terminal_width` - Width of the terminal
/// * `header_height` - Height of the header section
/// 
/// # Returns
/// Result with the current row position after rendering (for footer positioning)
pub fn render_swimmers<W: Write>(
    stdout: &mut W, 
    swimmers: &[Swimmer], 
    selected_index: usize, 
    terminal_width: usize,
    header_height: u16
) -> Result<u16> {
    // Lane width calculation
    let lane_width: usize = terminal_width.saturating_sub(10);

    // Display swimmer lanes and positions - starts exactly after header
    let mut current_row: u16 = header_height + 1;
    
    for (i, swimmer) in swimmers.iter().enumerate() {
        let is_selected: bool = i == selected_index;
        
        // Render swimmer stats bar
        render_swimmer_stats(stdout, swimmer, is_selected, terminal_width, current_row)?;
        current_row += 1;
        
        // Render swimmer lane
        render_swimmer_lane(stdout, swimmer, lane_width, terminal_width, current_row)?;
        current_row += 1;
        
        // Render divider
        render_lane_divider(stdout, terminal_width, current_row)?;
        current_row += 1;
    }
    
    Ok(current_row)
}

/// Renders the stats for a single swimmer
fn render_swimmer_stats<W: Write>(
    stdout: &mut W,
    swimmer: &Swimmer,
    is_selected: bool,
    terminal_width: usize,
    row: u16
) -> Result<()> {
    // Box styling for selected swimmer
    if is_selected {
        let box_width: usize = terminal_width.saturating_sub(6);
        execute!(
            stdout,
            cursor::MoveTo(3, row),
            SetBackgroundColor(Color::DarkGrey),
            Print(" ".repeat(box_width)),
            ResetColor
        ).into_diagnostic()?;
    }
    
    // Swimmer stats with better formatting
    let stats: String = format!(
        "{} {} | Speed: {} | Lengths: {} | Next Upgrade: {} lengths",
        if is_selected { "➤" } else { " " },
        swimmer.name,
        swimmer.display_speed(),
        swimmer.lengths,
        swimmer.upgrade_cost
    );
    let stats_padding: usize = center_padding(stats.len(), terminal_width);

    // Highlight selected swimmer
    if is_selected {
        execute!(stdout, SetForegroundColor(Color::White), SetAttribute(Attribute::Bold)).into_diagnostic()?;
    } else {
        execute!(stdout, SetForegroundColor(Color::Grey)).into_diagnostic()?;
    }

    execute!(
        stdout,
        cursor::MoveTo(stats_padding as u16, row),
        Print(&stats),
        ResetColor
    ).into_diagnostic()?;
    
    Ok(())
}

/// Renders a swimming lane with the swimmer at the correct position
fn render_swimmer_lane<W: Write>(
    stdout: &mut W,
    swimmer: &Swimmer,
    lane_width: usize,
    terminal_width: usize,
    row: u16
) -> Result<()> {
    // Calculate lane padding
    let lane_padding: usize = center_padding(lane_width + 2, terminal_width);
    
    // Create the lane string with better pool styling
    let mut lane: String = String::with_capacity(lane_width + 2);
    lane.push('│'); // Left wall
    for _ in 0..lane_width {
        lane.push('~');
    }
    lane.push('│'); // Right wall

    // Draw the base lane with water color
    execute!(
        stdout,
        cursor::MoveTo(lane_padding as u16, row),
        SetForegroundColor(Color::Blue),
        Print(&lane),
        ResetColor
    ).into_diagnostic()?;

    // Calculate swimmer position in the lane
    let position_in_lane: usize = swimmer.position * lane_width / 100;
    
    // Draw the swimmer with distinctive color based on speed
    let swimmer_color: Color = match swimmer.speed as usize {
        0..=1 => Color::Red,
        2..=3 => Color::Yellow,
        _ => Color::Green,
    };
    
    execute!(
        stdout,
        cursor::MoveTo((lane_padding + position_in_lane + 1) as u16, row),
        SetForegroundColor(swimmer_color),
        SetAttribute(Attribute::Bold),
        Print(if swimmer.direction { "-→" } else { "←-" }),
        SetAttribute(Attribute::Reset),
        ResetColor
    ).into_diagnostic()?;
    
    Ok(())
}

/// Renders a divider between swimmer lanes
fn render_lane_divider<W: Write>(
    stdout: &mut W,
    terminal_width: usize,
    row: u16
) -> Result<()> {
    let divider: &str = "· · · · · · · · · · · · · · · · · · · · · · · · · · · · · · ·";
    let divider_padding: usize = center_padding(divider.len(), terminal_width);
    
    execute!(
        stdout,
        cursor::MoveTo(divider_padding as u16, row),
        SetForegroundColor(Color::DarkGrey),
        Print(divider),
        ResetColor
    ).into_diagnostic()?;
    
    Ok(())
}
