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

pub fn show_welcome_message() -> Result<()> {
    let terminal_dimensions =
        terminal_size().ok_or_else(|| miette::miette!("Failed to get terminal size"))?;
    let (Width(width), _) = terminal_dimensions;
    let terminal_width = width as usize;

    let messages = [
        "Welcome to Swimming Idle Game!",
        "Each swimmer starts from the left and races to the right.",
        "Press arrow keys to select swimmers, Space to upgrade, q to quit.",
    ];

    execute!(stdout(), Clear(ClearType::All), cursor::MoveTo(0, 0)).into_diagnostic()?;

    for (i, msg) in messages.iter().enumerate() {
        let padding = center_padding(msg.len(), terminal_width);
        execute!(
            stdout(),
            cursor::MoveTo(padding as u16, i as u16),
            Print(format!("{}\n", msg))
        )
        .into_diagnostic()?;
    }

    Ok(())
}

pub fn show_upgrade_message(swimmer: &Swimmer, success: bool) -> Result<()> {
    let terminal_dimensions =
        terminal_size().ok_or_else(|| miette::miette!("Failed to get terminal size"))?;
    let (Width(width), Height(height)) = terminal_dimensions;
    let terminal_width = width as usize;

    let message = if success {
        format!("✓ {} upgraded to speed {}!", swimmer.name, swimmer.speed)
    } else {
        format!(
            "✗ Not enough lengths! Need {} more.",
            swimmer.upgrade_cost as i32 - swimmer.lengths
        )
    };

    let padding = center_padding(message.len(), terminal_width);
    let message_row = height.saturating_sub(3);

    execute!(
        stdout(),
        cursor::SavePosition,
        cursor::MoveTo(padding as u16, message_row),
        if success {
            SetForegroundColor(Color::Green)
        } else {
            SetForegroundColor(Color::Red)
        },
        Print(message),
        ResetColor,
        cursor::RestorePosition
    )
    .into_diagnostic()?;

    stdout().flush().into_diagnostic()?;
    Ok(())
}

pub fn show_goodbye_message() -> Result<()> {
    let terminal_dimensions =
        terminal_size().ok_or_else(|| miette::miette!("Failed to get terminal size"))?;
    let (Width(width), _) = terminal_dimensions;
    let terminal_width = width as usize;

    let messages = [
        "Thanks for playing Swimming Idle Game!",
        "Press any key to exit...",
    ];

    execute!(stdout(), Clear(ClearType::All), cursor::MoveTo(0, 0)).into_diagnostic()?;

    execute!(
        stdout(),
        cursor::MoveTo(center_padding(messages[0].len(), terminal_width) as u16, 0),
        SetForegroundColor(Color::Cyan),
        Print(format!("{}\n", messages[0])),
        ResetColor,
        cursor::MoveTo(center_padding(messages[1].len(), terminal_width) as u16, 1),
        Print(format!("{}", messages[1]))
    )
    .into_diagnostic()?;

    Ok(())
}
