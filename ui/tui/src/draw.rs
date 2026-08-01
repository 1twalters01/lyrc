use chrono::Duration;
use lyrc_core::state::AppState;

use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    widgets::Paragraph,
};

pub fn draw(
    frame: &mut Frame,
    state: &AppState,
    position: Duration,
    current_cue_index: usize,
) {
    let layout = Layout::vertical([
        Constraint::Length(3),
    ])
    .split(frame.area());

    draw_header(
        frame,
        layout[0],
        state,
        position,
    )

    draw_body(
        frame,
        layout[1],
        state,
        current_cue_index,
    )

    draw_footer(
        frame,
        layout[2],
    )
}

fn draw_header(
    frame: &mut Frame,
    area: Rect,
    state: &AppState,
    position: Duration,
) {
    let track = state.track.as_ref();
    let title = track
        .map(|track| track.title.as_str())
        .unwrap_or_else(|| "");

    let artist = track
        .and_then(|track| track.artists.first())
        .cloned()
        .unwrap_or_else(|| String::from(""));

    let duration = track
        .and_then(|track| {
            format!(
                "{}:{:02}",
                track.duration.num_minutes(),
                track.duration.num_seconds() % 60,
            )
            .into()
        })
        .unwrap_or_else(|| String::from(""));

    let position_str = format!(
        "{}:{:02}",
        position.num_minutes(),
        position.num_seconds() % 60,
    );

    let header = format!("{} - {} - {}/{}", title, artist, position_str, duration);

    frame.render_widget(
        Paragraph::new(header),
        area
    );
}

fn draw_body(
    frame: &mut Frame,
    area: Rect,
    state: &AppState,
    current_cue_index: usize,
) {
    let subtitle_document = state.subtitle_document.as_ref();
    Todo!
}

fn draw_footer(
    frame: &mut Frame,
    area: Rect,
) {
    Todo!
}