use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;

use super::theme;
use super::titled_block;
use crate::app::AppState;

pub fn draw(f: &mut Frame, area: Rect, state: &AppState) {
    let block = titled_block("現在の天気");
    let inner = block.inner(area);
    f.render_widget(block, area);

    let mut lines: Vec<Line> = Vec::new();
    if let Some(cw) = &state.current {
        lines.push(Line::from(vec![
            Span::styled(
                format!("  {}  ", cw.icon.symbol()),
                Style::default().add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                cw.condition.chars().take(12).collect::<String>(),
                Style::default()
                    .fg(theme::ACCENT)
                    .add_modifier(Modifier::BOLD),
            ),
        ]));
        lines.push(Line::from(""));
        lines.push(Line::from(vec![
            Span::styled("  気温  ", Style::default().fg(theme::SUBTLE)),
            Span::styled(
                format!("{:>5.1} ℃", cw.temperature_c),
                Style::default()
                    .fg(theme::TEMP)
                    .add_modifier(Modifier::BOLD),
            ),
        ]));
        if let Some(h) = cw.humidity_pct {
            lines.push(Line::from(vec![
                Span::styled("  湿度  ", Style::default().fg(theme::SUBTLE)),
                Span::styled(
                    format!("{:>5.0} %", h),
                    Style::default().fg(theme::RAIN),
                ),
            ]));
        }
        if let Some(w) = cw.wind_speed_ms {
            lines.push(Line::from(vec![
                Span::styled("  風速  ", Style::default().fg(theme::SUBTLE)),
                Span::styled(
                    format!("{:>5.1} m/s", w),
                    Style::default().fg(theme::FG),
                ),
            ]));
        }
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            cw.observed_at.format("  観測 %m/%d %H:%M").to_string(),
            Style::default().fg(theme::SUBTLE),
        )));
    } else {
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            format!("  {} 取得中…", state.spinner()),
            Style::default().fg(theme::SUBTLE),
        )));
    }
    f.render_widget(Paragraph::new(lines), inner);
}
