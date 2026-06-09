// 起動時 Splash 画面
//
// 大きな ASCII ロゴと、初期データ取得中のスピナーを中央に表示する。
// 2秒経過 or 主要データ取得完了 で AppState.splash_active が false になり、
// 通常画面に切り替わる。

use ratatui::Frame;
use ratatui::layout::{Alignment, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;

use super::theme;
use crate::app::AppState;

const LOGO: &str = r#"
 ████████ ███████ ██████  ███    ███ ██████   █████  ██ ███    ██
    ██    ██      ██   ██ ████  ████ ██   ██ ██   ██ ██ ████   ██
    ██    █████   ██████  ██ ████ ██ ██████  ███████ ██ ██ ██  ██
    ██    ██      ██   ██ ██  ██  ██ ██   ██ ██   ██ ██ ██  ██ ██
    ██    ███████ ██   ██ ██      ██ ██   ██ ██   ██ ██ ██   ████
"#;

pub fn draw(f: &mut Frame, area: Rect, state: &AppState) {
    let mut lines: Vec<Line> = Vec::new();
    // 上の余白
    let logo_lines: Vec<&str> = LOGO.lines().collect();
    let total_lines = logo_lines.len() + 5;
    let pad_top = area.height.saturating_sub(total_lines as u16) / 2;
    for _ in 0..pad_top {
        lines.push(Line::from(""));
    }
    for l in &logo_lines {
        lines.push(Line::from(Span::styled(
            l.to_string(),
            Style::default()
                .fg(theme::ACCENT)
                .add_modifier(Modifier::BOLD),
        )));
    }
    let s = crate::i18n::strings(state.config.ui.language);
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        s.splash_tagline.to_string(),
        Style::default().fg(theme::FG),
    )));
    lines.push(Line::from(Span::styled(
        format!("  v{}", env!("CARGO_PKG_VERSION")),
        Style::default().fg(theme::SUBTLE),
    )));
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        format!("  {} {}", state.spinner(), s.splash_starting),
        Style::default().fg(theme::ACCENT_2),
    )));

    let p = Paragraph::new(lines)
        .alignment(Alignment::Center)
        .style(Style::default().bg(theme::BG));
    f.render_widget(p, area);
}
