use lyrc_core::state::AppState;
use ratatui::{layout::Rect, widgets::Paragraph, Frame};

pub fn draw_footer(
    frame: &mut Frame,
    area: Rect,
    state: &mut AppState,
    active_cues: &[usize],
) {
    let selected_line = state.selected_line;
    let automatic_scroll_offset = state.automatic_scroll_offset;

    let text = format!("\nselected line: {:?}\nautomatic scroll: {:?}\nactive cues: {:?}", selected_line, automatic_scroll_offset, active_cues);

    frame.render_widget(
        Paragraph::new(text),
        area,
    );
}
