/// Calculates the left padding needed to center text in the terminal
///
/// # Arguments
/// * `text_width` - Width of the text to be centered
/// * `terminal_width` - Total width of the terminal
///
/// # Returns
/// The number of spaces to pad on the left
pub fn center_padding(text_width: usize, terminal_width: usize) -> usize {
    if text_width >= terminal_width {
        0_usize
    } else {
        (terminal_width - text_width) / 2_usize
    }
}
