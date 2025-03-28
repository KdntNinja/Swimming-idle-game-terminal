// Helper function to calculate left padding for centering text
pub fn center_padding(text_width: usize, terminal_width: usize) -> usize {
    if text_width >= terminal_width {
        0
    } else {
        (terminal_width - text_width) / 2
    }
}
