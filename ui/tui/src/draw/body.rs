use lyrc_core::state::AppState;
use ratatui::{
    Frame,
    layout::Rect,
    style::{Modifier, Style},
    text::Line,
    widgets::Paragraph,
};
use subtitles::subtitles::SubtitleContent;

pub fn draw_body(
    frame: &mut Frame,
    area: Rect,
    state: &mut AppState,
    active_cue_indices: &[usize],
) {
    let subtitle_document = state.subtitle_document.as_ref();
    let cues = subtitle_document.map(|document| document.cues.clone());
    let active_cues = cues.map(|cues| {
        cues.iter()
            .enumerate()
            // .filter(|(idx, _)| active_cue_indices.contains(idx))
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

    let mut lines: Vec<Line> = active_cues_str.into_iter().map(Line::from).collect();
    highlight_line(&mut lines, active_cue_indices);

    let visible_height = area.height as usize;

    let automatic_scroll_offset = active_cue_indices
        .first()
        .unwrap_or(&0usize)
        .saturating_sub(visible_height / 2) as u16;
    state.automatic_scroll_offset = automatic_scroll_offset;

    let scroll_offset = match state.manual_scroll_offset {
        Some(offset) => offset,
        None => automatic_scroll_offset,
    };

    frame.render_widget(Paragraph::new(lines).scroll((scroll_offset, 0)), area);
}

fn highlight_line(lines: &mut Vec<Line>, indices: &[usize]) {
    for &index in indices {
        if let Some(line) = lines.get_mut(index) {
            *line = line
                .clone()
                .style(Style::default().add_modifier(Modifier::BOLD));
        }
    }
}
