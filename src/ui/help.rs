// ヘルプモーダル（`?` キーで表示）
//
// 画面中央に半透明風のオーバーレイで操作一覧と凡例を表示する。
// 現在の画面を Clear で消してから上書きする。

use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{BorderType, Clear, Paragraph};

use super::theme;

pub fn draw(f: &mut Frame, area: Rect) {
    // 中央 60×22 程度のサイズで配置
    let modal = centered_rect(70, 24, area);
    f.render_widget(Clear, modal);

    let block = ratatui::widgets::Block::default()
        .borders(ratatui::widgets::Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(theme::ACCENT))
        .title(Span::styled(
            " ❓ ヘルプ ",
            Style::default()
                .fg(theme::ACCENT_2)
                .add_modifier(Modifier::BOLD),
        ))
        .style(Style::default().bg(theme::BG));
    let inner = block.inner(modal);
    f.render_widget(block, modal);

    let mut lines: Vec<Line> = Vec::new();
    lines.push(section("キー操作"));
    lines.push(kv("q  /  Esc", "終了"));
    lines.push(kv("?", "このヘルプを開く / 閉じる"));
    lines.push(kv("r", "現在時刻に戻して再取得"));
    lines.push(kv("+ / -", "ズーム（市〜街区レベル）"));
    lines.push(kv("h j k l", "地点の移動（≒2km）"));
    lines.push(kv(", / .", "雨雲を時系列で前後にスクラブ"));
    lines.push(kv("p", "雨雲アニメーション再生 toggle"));
    lines.push(kv("m", "地図スタイル切替 (CARTO / 標準 / 航空)"));
    lines.push(Line::from(""));
    lines.push(section("雨雲の色凡例 (mm/h)"));
    lines.push(legend_line());
    lines.push(Line::from(""));
    lines.push(section("データソース"));
    lines.push(kv("雨雲(国内)", "気象庁ナウキャスト"));
    lines.push(kv("雨雲(海外)", "Open-Meteo"));
    lines.push(kv("地図", "CARTO / 国土地理院"));
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "  何かキーを押して閉じる",
        Style::default().fg(theme::SUBTLE),
    )));

    f.render_widget(Paragraph::new(lines), inner);
}

fn section(title: &str) -> Line<'static> {
    Line::from(Span::styled(
        format!(" ▶ {}", title),
        Style::default()
            .fg(theme::ACCENT)
            .add_modifier(Modifier::BOLD),
    ))
}

fn kv(key: &str, desc: &str) -> Line<'static> {
    Line::from(vec![
        Span::raw("    "),
        Span::styled(
            format!("{:<14}", key),
            Style::default()
                .fg(theme::WARN)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(desc.to_string(), Style::default().fg(theme::FG)),
    ])
}

fn legend_line() -> Line<'static> {
    // 雨雲カラーバーをミニチュアで表示
    let stops = [
        ("1", (200u8, 230, 255)),
        ("4", (40, 130, 230)),
        ("8", (50, 180, 80)),
        ("16", (250, 230, 50)),
        ("24", (250, 180, 30)),
        ("40", (240, 70, 50)),
        ("64", (200, 50, 180)),
        ("80+", (120, 30, 130)),
    ];
    let mut spans = vec![Span::raw("    ")];
    for (label, (r, g, b)) in stops.iter() {
        spans.push(Span::styled(
            "  ",
            Style::default().bg(ratatui::style::Color::Rgb(*r, *g, *b)),
        ));
        spans.push(Span::styled(
            format!(" {} ", label),
            Style::default().fg(theme::FG),
        ));
    }
    Line::from(spans)
}

fn centered_rect(width: u16, height: u16, area: Rect) -> Rect {
    let v = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(area.height.saturating_sub(height) / 2),
            Constraint::Length(height.min(area.height)),
            Constraint::Min(0),
        ])
        .split(area);
    let h = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(area.width.saturating_sub(width) / 2),
            Constraint::Length(width.min(area.width)),
            Constraint::Min(0),
        ])
        .split(v[1]);
    h[1]
}
