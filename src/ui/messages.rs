use crate::swimmer::Swimmer;
use crate::ui::utils::center_padding;
use miette::{IntoDiagnostic, Result};
use std::io::{stdout, Write};
use terminal_size::{terminal_size, Height, Width};

use crossterm::{
    cursor, execute,
    style::{
        Attribute, Color, Print, ResetColor, SetAttribute, SetBackgroundColor, SetForegroundColor,
    },
    terminal::{Clear, ClearType},
};

/// Shows an upgrade message when a player attempts to upgrade a swimmer
///
/// # Arguments
/// * `swimmer` - Reference to the swimmer being upgraded
/// * `success` - Whether the upgrade was successful
///
/// # Returns
/// A Result indicating success or an error
pub fn show_upgrade_message(swimmer: &Swimmer, success: bool) -> Result<()> {
    let terminal_dimensions =
        terminal_size().ok_or_else(|| miette::miette!("Failed to get terminal size"))?;
    let (Width(width), Height(height)) = terminal_dimensions;
    let terminal_width: usize = width as usize;

    let message: String = if success {
        format!(
            "✅ {} upgraded to speed {}!",
            swimmer.name,
            swimmer.display_speed()
        )
    } else {
        format!(
            "❌ Not enough lengths! Need {} more for upgrade.",
            swimmer.upgrade_cost as i32 - swimmer.lengths
        )
    };

    let padding: usize = center_padding(message.len() + 6, terminal_width);
    let message_row: u16 = height.saturating_sub(3);

    // Create a box for the message
    execute!(
        stdout(),
        cursor::SavePosition,
        cursor::MoveTo(padding as u16, message_row),
        if success {
            SetBackgroundColor(Color::DarkGreen)
        } else {
            SetBackgroundColor(Color::DarkRed)
        },
        Print("  "),
        if success {
            SetForegroundColor(Color::Green)
        } else {
            SetForegroundColor(Color::Red)
        },
        SetAttribute(Attribute::Bold),
        Print(&message),
        Print("  "),
        SetAttribute(Attribute::Reset),
        ResetColor,
        cursor::RestorePosition
    )
    .into_diagnostic()?;

    stdout().flush().into_diagnostic()?;
    Ok(())
}

/// Shows a message when a player attempts to add a new swimmer
///
/// # Arguments
/// * `swimmer` - Reference to the new swimmer (or first swimmer if failed)
/// * `success` - Whether adding the swimmer was successful
/// * `cost` - Optional cost for a new swimmer, used for failure message
///
/// # Returns
/// A Result indicating success or an error
pub fn show_new_swimmer_message(
    swimmer: &Swimmer,
    success: bool,
    cost: Option<usize>,
) -> Result<()> {
    let terminal_dimensions =
        terminal_size().ok_or_else(|| miette::miette!("Failed to get terminal size"))?;
    let (Width(width), Height(height)) = terminal_dimensions;
    let terminal_width: usize = width as usize;

    let message: String = if success {
        format!(
            "✅ New swimmer {} joined with speed {}!",
            swimmer.name,
            swimmer.display_speed()
        )
    } else {
        format!(
            "❌ Not enough lengths! Need {} more for a new swimmer.",
            cost.unwrap_or(0) as i32 - swimmer.lengths
        )
    };

    let padding: usize = center_padding(message.len() + 6, terminal_width);
    let message_row: u16 = height.saturating_sub(3);

    // Create a box for the message
    execute!(
        stdout(),
        cursor::SavePosition,
        cursor::MoveTo(padding as u16, message_row),
        if success {
            SetBackgroundColor(Color::DarkGreen)
        } else {
            SetBackgroundColor(Color::DarkRed)
        },
        Print("  "),
        if success {
            SetForegroundColor(Color::Green)
        } else {
            SetForegroundColor(Color::Red)
        },
        SetAttribute(Attribute::Bold),
        Print(&message),
        Print("  "),
        SetAttribute(Attribute::Reset),
        ResetColor,
        cursor::RestorePosition
    )
    .into_diagnostic()?;

    stdout().flush().into_diagnostic()?;
    Ok(())
}

/// Shows a goodbye message when the player quits the game
///
/// # Returns
/// A Result indicating success or an error
pub fn show_goodbye_message() -> Result<()> {
    let terminal_dimensions =
        terminal_size().ok_or_else(|| miette::miette!("Failed to get terminal size"))?;
    let (Width(width), _) = terminal_dimensions;
    let terminal_width: usize = width as usize;

    execute!(stdout(), Clear(ClearType::All), cursor::MoveTo(0, 0)).into_diagnostic()?;

    // Draw a decorative top border
    let border: String = "═".repeat(terminal_width);
    execute!(
        stdout(),
        cursor::MoveTo(0, 2),
        SetForegroundColor(Color::Cyan),
        Print(&border),
        ResetColor
    )
    .into_diagnostic()?;

    // Goodbye message
    let title: &str = "🏊 Thanks for playing Swimming Idle Game! 🏊";
    let subtitle: &str = "Your swimmers will miss you...";
    let exit_msg: &str = "Press any key to exit...";

    execute!(
        stdout(),
        cursor::MoveTo(center_padding(title.len(), terminal_width) as u16, 4),
        SetForegroundColor(Color::Cyan),
        SetAttribute(Attribute::Bold),
        Print(title),
        SetAttribute(Attribute::Reset),
        ResetColor,
        cursor::MoveTo(center_padding(subtitle.len(), terminal_width) as u16, 6),
        SetForegroundColor(Color::White),
        Print(subtitle),
        ResetColor,
        cursor::MoveTo(center_padding(exit_msg.len(), terminal_width) as u16, 8),
        SetForegroundColor(Color::Grey),
        Print(exit_msg),
        ResetColor
    )
    .into_diagnostic()?;

    // Draw a decorative bottom border
    execute!(
        stdout(),
        cursor::MoveTo(0, 10),
        SetForegroundColor(Color::Cyan),
        Print(&border),
        ResetColor
    )
    .into_diagnostic()?;

    stdout().flush().into_diagnostic()?;
    Ok(())
}
