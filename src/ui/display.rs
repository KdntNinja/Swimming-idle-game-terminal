use crate::swimmer::Swimmer;
use crate::ui::utils::center_padding;
use miette::{IntoDiagnostic, Result};
use std::io::{stdout, Write};
use terminal_size::{terminal_size, Height, Width};

use crossterm::{
    cursor, execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{Clear, ClearType},
};

pub fn display_ui(swimmers: &[Swimmer], selected_index: usize) -> Result<()> {
    let mut stdout = stdout();
    execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0, 0)).into_diagnostic()?;

    // Get terminal dimensions
    let terminal_dimensions =
        terminal_size().ok_or_else(|| miette::miette!("Failed to get terminal size"))?;
    let (Width(width), Height(_)) = terminal_dimensions;
    let terminal_width = width as usize;

    // Game title with color
    let title = "Swimming Idle Game";
    let title_padding = center_padding(title.len(), terminal_width);
    execute!(
        stdout,
        cursor::MoveTo(title_padding as u16, 0),
        SetForegroundColor(Color::Cyan),
        Print(format!("{}\n\n", title)),
        ResetColor
    )
    .into_diagnostic()?;

    // Instructions
    let instructions = "Controls: [↑/↓] Select Swimmer | [Space] Upgrade | [q] Quit";
    let instructions_padding = center_padding(instructions.len(), terminal_width);
    execute!(
        stdout,
        cursor::MoveTo(instructions_padding as u16, 2),
        Print(format!("{}\n\n", instructions))
    )
    .into_diagnostic()?;

    // Lane width calculation
    let lane_width = (terminal_width).saturating_sub(10);

    // Display swimmer lanes and positions
    let mut current_row = 4;
    for (i, swimmer) in swimmers.iter().enumerate() {
        let is_selected = i == selected_index;

        // Swimmer stats
        let stats = format!(
            "{} {} | Speed: {} | Lengths: {} | Next Upgrade: {} lengths",
            if is_selected { ">" } else { " " },
            swimmer.name,
            swimmer.speed,
            swimmer.lengths,
            swimmer.upgrade_cost
        );
        let stats_padding = center_padding(stats.len(), terminal_width);

        // Highlight selected swimmer
        if is_selected {
            execute!(stdout, SetForegroundColor(Color::Yellow)).into_diagnostic()?;
        }

        execute!(
            stdout,
            cursor::MoveTo(stats_padding as u16, current_row),
            Print(format!("{}\n", stats))
        )
        .into_diagnostic()?;
        current_row += 1;

        // Calculate swimmer position in the lane
        let position_in_lane = swimmer.position * lane_width / 100;

        // Create the lane string first
        let mut lane = String::with_capacity(lane_width + 2);
        lane.push('|'); // Left wall

        for pos in 0..lane_width {
            // Always add water for consistent length
            lane.push('~');
        }

        lane.push('|'); // Right wall

        // Calculate lane padding before using it
        let lane_padding = center_padding(lane.len(), terminal_width);

        // Draw the base lane
        execute!(
            stdout,
            cursor::MoveTo(lane_padding as u16, current_row),
            SetForegroundColor(Color::Blue),
            Print(&lane),
            ResetColor
        )
        .into_diagnostic()?;

        // Now draw the swimmer at the correct position
        execute!(
            stdout,
            cursor::MoveTo((lane_padding + position_in_lane + 1) as u16, current_row),
            SetForegroundColor(Color::Red),
            Print(if swimmer.direction { "→" } else { "←" }),
            ResetColor
        )
        .into_diagnostic()?;

        current_row += 2;

        if is_selected {
            execute!(stdout, ResetColor).into_diagnostic()?;
        }
    }

    stdout.flush().into_diagnostic()?;
    Ok(())
}
