use lyrc_core::state::AppState;
use ratatui::{Frame, layout::Rect, widgets::Paragraph};

pub fn draw_footer(frame: &mut Frame, area: Rect, state: &mut AppState, active_cues: &[usize]) {
    let selected_cue = state.selected_cue;
    let automatic_scroll_offset = state.automatic_scroll_offset;
    let mode = state.app_mode.to_string();

    let text = format!(
        "\nselected line: {:?}\nautomatic scroll: {:?}\nactive cues: {:?}\nmode: {:?}",
        selected_cue, automatic_scroll_offset, active_cues, mode,
    );

    frame.render_widget(Paragraph::new(text), area);
}
