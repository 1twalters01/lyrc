use std::{fmt::Debug, rc::Rc};

use chrono::Duration;
use lyrc_core::state::AppState;

use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
};
use synchronizer::traits::CueIndexed;

use crate::draw::{body::draw_body, footer::draw_footer, header::draw_header};

pub fn window_layout(frame: &mut Frame) -> Rc<[Rect]> {
    Layout::vertical([
        Constraint::Length(3),
        Constraint::Min(1),
        Constraint::Length(5),
    ])
    .split(frame.area())
}

pub fn draw_window<A: Debug + CueIndexed>(
    frame: &mut Frame,
    layout: Rc<[Rect]>,
    state: &mut AppState,
    position: Option<Duration>,
    active_cues: &[A],
) {
    draw_header(frame, layout[0], state, position);

    draw_body(frame, layout[1], state, active_cues);

    draw_footer(frame, layout[2], state, active_cues);
}
