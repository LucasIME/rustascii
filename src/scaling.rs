use terminal_size::terminal_size;
use terminal_size::Width;

pub const CHAR_SCALING_FACTOR: usize = 3;

pub fn scale_to_fit_terminal_horizontally(src_width: u32, src_height: u32) -> (u32, u32) {
    let terminal_dimensions = terminal_size().unwrap();
    let Width(terminal_width) = terminal_dimensions.0;
    if src_width < terminal_width as u32 {
        return (src_width, src_height);
    }

    let scaling_factor: f32 =
        terminal_width as f32 / (CHAR_SCALING_FACTOR as f32 * src_width as f32);
    let new_width = (src_width as f32 * scaling_factor) as u32;
    let new_height = (src_height as f32 * scaling_factor) as u32;

    return (new_width, new_height);
}
