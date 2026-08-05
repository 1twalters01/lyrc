use lyrc_core::state::AppState;
use ratatui::{Frame, layout::Rect, text::Line, widgets::Paragraph};
use subtitles::subtitles::SubtitleContent;

pub fn draw_body(frame: &mut Frame, area: Rect, state: &AppState, active_cues: &[usize]) {
    let subtitle_document = state.subtitle_document.as_ref();
    let cues = subtitle_document.map(|document| document.cues.clone());
    let active_cues = cues.map(|cues| {
        cues.iter()
            .enumerate()
            .filter(|(idx, _)| active_cues.contains(idx))
            .map(|(_, cue)| cue.clone())
            .collect::<Vec<_>>()
    });
    let active_cues_str: Vec<String> = active_cues
        .map(|cues| {
            cues.iter()
                .map(|cue| {
                    let total_minutes = cue.start.num_minutes();
                    let seconds = cue.start.num_seconds() % 60;
                    let hundredths = (cue.start.num_milliseconds() % 1000) / 10;

                    let timestamp =
                        format!("[{:02}:{:02}.{:02}]", total_minutes, seconds, hundredths,);

                    let content = match &cue.content.clone() {
                        SubtitleContent::Text(content) => content.clone(),
                    };
                    format!("{} {}", timestamp, content)
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();

    let lines: Vec<Line> = active_cues_str.into_iter().map(Line::from).collect();

    frame.render_widget(Paragraph::new(lines), area);
}
