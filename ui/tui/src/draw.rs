use chrono::Duration;
use lyrc_core::state::AppState;

use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    widgets::Paragraph,
};

pub fn draw(frame: &mut Frame, state: &AppState, position: Duration) {
    let layout = Layout::vertical([Constraint::Length(3)]).split(frame.area());

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
    // println!("\nheader: {}", header);

    // frame.render_widget(Clear, frame.area());
    frame.render_widget(Paragraph::new(header), layout[0]);
}
