# Swimming-idle-game-terminal

A terminal-based idle game where you manage swimmers who race across lanes and earn lengths that can be used for upgrades.

## Game features

- Terminal-based lane UI for visualizing gameplay, including swimmer positions, progress, and lane status.
- Visual representation of swimmers racing across the pool from left to right.
- Terminal input for upgrades and in-game interactions, allowing players to enhance swimmer abilities.
- Idle mechanics that let the game progress even when the player is not actively interacting.
- A progression system with lengths as a currency to upgrade swimmers.
- Customizable swimmers with unique stats and swimming styles.
- Real-time feedback and stats to track progress and make informed decisions.

## Project Structure

The project is organized into modules:

- `main.rs` - Entry point of the application
- `swimmer.rs` - Contains the Swimmer struct and implementation
- `ui.rs` - Handles UI rendering and terminal output
- `game.rs` - Manages the game state and main game loop

## Technical Details

This game uses several Rust crates:

- `crossterm` for terminal manipulation, colored output, and input handling
- `miette` for error handling
- `terminal_size` for getting terminal dimensions

## Getting Started

Follow these steps to set up and run the project:

### Prerequisites

- Rust (latest stable version). Install it from [rust-lang.org](https://www.rust-lang.org/).
- A terminal emulator that supports ANSI escape codes and Unicode characters.

### Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/KdntNinja/Swimming-idle-game-terminal.git
   cd Swimming-idle-game-terminal
   ```

2. Build the project:

   ```bash
   cargo build --release
   ```

### Running the Game

Run the game using the following command:

```bash
   cargo run --release
```

### Controls

- **Up/Down Arrow Keys**: Select different swimmers
- **Space**: Upgrade the selected swimmer (costs lengths)
- **q**: Quit the game

### Gameplay

- Swimmers start from the left side of the pool and race to the right
- When they reach the end, they count it as a length and start again from the left
- Use lengths to upgrade swimmers, increasing their speed
- Upgrade costs increase with each purchase
- Faster swimmers complete lengths more quickly
- Each swimmer has their own unique swimming style represented by different characters

### Contributing

Contributions are welcome! Feel free to open issues or submit pull requests to improve the game.
