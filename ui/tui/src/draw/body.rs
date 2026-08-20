use lyrc_core::{mode::AppMode, state::AppState};
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
    let cues_str: Vec<String> = cues
        .map(|cues| {
            cues.iter()
                .map(|cue| {
                    let start_timestamp = format!(
                        "[{:02}:{:02}.{:03}]",
                        cue.start.num_minutes(),
                        cue.start.num_seconds() % 60,
                        (cue.start.num_milliseconds() % 1000) / 10,
                    );

                    let end_timestamp = format!(
                        "[{:02?}:{:02?}.{:03?}]",
                        cue.end.num_minutes(),
                        cue.end.num_seconds() % 60,
                        (cue.end.num_milliseconds() % 1000) / 10,
                    );

                    let content = match &cue.content.clone() {
                        SubtitleContent::Text(content) => content.clone(),
                        SubtitleContent::Words(words) => todo!(),
                    };

                    format!("{}-{} {}", start_timestamp, end_timestamp, content)
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();

    let visible_height = area.height as usize;
    let middle = visible_height / 2;

    let (selected_cue, selected_cues) = match &state.app_mode {
        AppMode::Normal => (None, Vec::new()),
        AppMode::Select {
            cue_index,
            selected_cues,
        } => (Some(*cue_index), selected_cues.clone()),
        AppMode::Edit {
            cue_index,
            selected_cues,
        } => (
            Some(*cue_index),
            selected_cues.iter().map(|c| c.index).collect(),
        ),
    };

    let selected_line_indices = match selected_cue {
        None => &Vec::new(),
        Some(index) => &Vec::from([index]),
    };

    let mut lines: Vec<Line> = cues_str.into_iter().map(Line::from).collect();
    highlight_lines(
        &mut lines,
        active_cue_indices,
        selected_line_indices,
        &selected_cues,
    );

    let automatic_scroll_offset = active_cue_indices
        .first()
        .unwrap_or(&0usize)
        .saturating_sub(middle);
    state.automatic_scroll_offset = automatic_scroll_offset;

    let scroll_offset = match selected_cue {
        Some(offset) => {
            if offset > middle {
                offset - middle
            } else {
                0
            }
        }
        None => automatic_scroll_offset,
    };

    frame.render_widget(
        Paragraph::new(lines).scroll((scroll_offset as u16, 0)),
        area,
    );
}

fn highlight_lines(
    lines: &mut [Line],
    bold_indices: &[usize],
    reverse_indices: &[usize],
    selected_indices: &[usize],
) {
    for &index in bold_indices {
        if let Some(line) = lines.get_mut(index) {
            *line = line
                .clone()
                .patch_style(Style::default().add_modifier(Modifier::BOLD));
        }
    }

    for &index in reverse_indices {
        if let Some(line) = lines.get_mut(index) {
            *line = line
                .clone()
                .patch_style(Style::default().add_modifier(Modifier::REVERSED));
        }
    }

    for &index in selected_indices {
        if let Some(line) = lines.get_mut(index) {
            *line = line
                .clone()
                .patch_style(Style::default().add_modifier(Modifier::UNDERLINED));
        }
    }
}
