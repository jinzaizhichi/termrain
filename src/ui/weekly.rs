use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;
use unicode_width::UnicodeWidthStr;

use super::titled_block;
use crate::app::AppState;

/// 文字列を「表示幅 = width」になるよう末尾にスペース追加。
/// 日本語や絵文字（東アジア文字）が混じっても揃う。
fn pad_w(s: &str, width: usize) -> String {
    let w = UnicodeWidthStr::width(s);
    if w >= width {
        s.to_string()
    } else {
        let pad = width - w;
        format!("{}{}", s, " ".repeat(pad))
    }
}

/// 文字列を「表示幅 = width」を超えないように切り詰める。
fn truncate_w(s: &str, width: usize) -> String {
    let mut acc = String::new();
    let mut used = 0usize;
    for c in s.chars() {
        let cw = UnicodeWidthStr::width(c.to_string().as_str());
        if used + cw > width {
            break;
        }
        acc.push(c);
        used += cw;
    }
    acc
}

pub fn draw(f: &mut Frame, area: Rect, state: &AppState) {
    let block = titled_block("週間予報");
    let inner = block.inner(area);
    f.render_widget(block, area);

    let mut lines: Vec<Line> = Vec::new();
    if state.daily.is_empty() {
        lines.push(Line::from(Span::styled(
            "読み込み中…",
            Style::default().fg(Color::Gray),
        )));
        f.render_widget(Paragraph::new(lines), inner);
        return;
    }

    // 各セルの表示幅。日付ラベル "06/09(Tue)" = 10幅 を基準にする。
    // セル内には日付・アイコン・天気・最高最低・降水確率を入れるので少し余裕を持たせる。
    const CELL_W: usize = 12;
    const SEP: &str = "│";

    let join_row = |cells: &[String]| -> String {
        cells
            .iter()
            .map(|s| pad_w(s, CELL_W))
            .collect::<Vec<_>>()
            .join(SEP)
    };

    let mut row_date = Vec::new();
    let mut row_icon = Vec::new();
    let mut row_cond = Vec::new();
    let mut row_temp = Vec::new();
    let mut row_pop = Vec::new();
    for d in state.daily.iter().take(7) {
        row_date.push(format!(" {}", d.date.format("%m/%d(%a)")));
        row_icon.push(format!("  {}", d.icon.symbol()));
        let cond = truncate_w(&d.condition, CELL_W - 1);
        row_cond.push(format!(" {}", cond));
        let hi = d
            .temp_max_c
            .map(|v| format!("{:>3.0}", v))
            .unwrap_or_else(|| "  -".into());
        let lo = d
            .temp_min_c
            .map(|v| format!("{:>3.0}", v))
            .unwrap_or_else(|| "  -".into());
        row_temp.push(format!(" {}/{}", hi, lo));
        let pop = d
            .precipitation_prob_pct
            .map(|p| format!("{:>3.0}%", p))
            .unwrap_or_else(|| "  - ".into());
        row_pop.push(format!(" {}", pop));
    }

    lines.push(Line::from(join_row(&row_date)));
    lines.push(Line::from(Span::styled(
        join_row(&row_icon),
        Style::default().add_modifier(Modifier::BOLD),
    )));
    lines.push(Line::from(join_row(&row_cond)));
    lines.push(Line::from(Span::styled(
        join_row(&row_temp),
        Style::default().fg(Color::Red),
    )));
    lines.push(Line::from(Span::styled(
        join_row(&row_pop),
        Style::default().fg(Color::Blue),
    )));

    f.render_widget(Paragraph::new(lines), inner);
}
