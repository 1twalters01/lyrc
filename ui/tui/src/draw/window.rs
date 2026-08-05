use chrono::Duration;
use lyrc_core::state::AppState;

use ratatui::{
    Frame,
    layout::{Constraint, Layout},
};

use crate::draw::{body::draw_body, footer::draw_footer, header::draw_header};

pub fn draw_window(
    frame: &mut Frame,
    state: &mut AppState,
    position: Option<Duration>,
    active_cues: &[usize],
) {
    let layout = Layout::vertical([
        Constraint::Length(3),
        Constraint::Min(1),
        Constraint::Length(5),
    ])
    .split(frame.area());

    draw_header(frame, layout[0], state, position);

    draw_body(frame, layout[1], state, active_cues);

    draw_footer(frame, layout[2], state, active_cues);
}
