use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;

use crate::app::AppState;
use super::titled_block;

pub fn draw(f: &mut Frame, area: Rect, state: &AppState) {
    let block = titled_block("現在の天気");
    let inner = block.inner(area);
    f.render_widget(block, area);

    let mut lines: Vec<Line> = Vec::new();
    if let Some(cw) = &state.current {
        lines.push(Line::from(vec![
            Span::styled(
                format!("{} ", cw.icon.symbol()),
                Style::default().add_modifier(Modifier::BOLD),
            ),
            Span::styled(cw.condition.clone(), Style::default().fg(Color::White)),
        ]));
        lines.push(Line::from(format!("気温  : {:>5.1} ℃", cw.temperature_c)));
        if let Some(h) = cw.humidity_pct {
            lines.push(Line::from(format!("湿度  : {:>5.0} %", h)));
        }
        if let Some(w) = cw.wind_speed_ms {
            lines.push(Line::from(format!("風速  : {:>5.1} m/s", w)));
        }
        lines.push(Line::from(Span::styled(
            cw.observed_at.format("観測: %m/%d %H:%M").to_string(),
            Style::default().fg(Color::DarkGray),
        )));
    } else {
        lines.push(Line::from(Span::styled(
            "読み込み中…",
            Style::default().fg(Color::DarkGray),
        )));
    }
    f.render_widget(Paragraph::new(lines), inner);
}
