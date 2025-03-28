mod messages;
mod display;
mod utils;

pub use crate::ui::messages::{
    show_goodbye_message, show_new_swimmer_message, show_upgrade_message,
};

pub use crate::ui::display::display_ui;
