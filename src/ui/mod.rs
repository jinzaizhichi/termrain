// UI レイアウト: 4 ペイン構成
//   ┌──────────┬─────────────────┐
//   │ 現在天気 │ 雨雲レーダー    │
//   ├──────────┴─────────────────┤
//   │ 時間別予報グラフ           │
//   ├────────────────────────────┤
//   │ 週間予報                   │
//   └────────────────────────────┘

mod current;
mod hourly;
mod radar;
mod weekly;

use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};

use crate::app::AppState;

pub fn draw(f: &mut Frame, state: &mut AppState) {
    let size = f.area();

    // 縦方向: ヘッダー、上段（レーダーがメイン）、中段、下段、フッター
    //
    // レーダーは地図表示なので縦幅を確保しないと意味がない。
    // 端末高さに応じて Percentage で配分し、最低限の行数も保証する。
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),       // ヘッダー
            Constraint::Min(16),         // 上段: 現在天気 + レーダー（大きく）
            Constraint::Length(8),       // 時間別
            Constraint::Length(6),       // 週間
            Constraint::Length(1),       // フッター
        ])
        .split(size);

    draw_header(f, chunks[0], state);

    // 上段を左右に分割。
    // レーダーは正方形の画像（1024x1024）なので、パネルも見た目正方形にする。
    // 端末 1 セルは縦長 (≒ 横10px : 縦20px = 1:2) なので、
    // 見た目正方形にするには「幅セル数 = 高さセル数 × 2」が必要。
    // 画像合成側のアスペクト比に合わせて、パネル幅をここで動的に決める。
    let radar_aspect = if let Some(img) = state
        .radar
        .as_ref()
        .and_then(|r| r.composite_image.as_ref())
    {
        img.width() as f32 / img.height() as f32
    } else {
        1.0 // 画像が無い時のフォールバック
    };
    // 「画像が正方形 (aspect=1)」の場合、見た目正方形にするためにセル幅 = 高さ × 2
    // フォントセルは縦長なので「セル幅 = 高さ × 2 × aspect」
    let radar_h = chunks[1].height;
    let radar_w_cells = ((radar_h as f32) * 2.0 * radar_aspect) as u16;
    // 左ペインの 32 セルを引いた残り幅でクランプ
    let max_radar_w = chunks[1].width.saturating_sub(32);
    let radar_w_cells = radar_w_cells.min(max_radar_w).max(20);

    let top = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(32),             // 現在の天気
            Constraint::Length(radar_w_cells),  // レーダー (画像アスペクト比に合わせる)
            Constraint::Min(0),                  // 余白
        ])
        .split(chunks[1]);
    current::draw(f, top[0], state);
    radar::draw(f, top[1], state);

    hourly::draw(f, chunks[2], state);
    weekly::draw(f, chunks[3], state);

    draw_footer(f, chunks[4], state);
}

// 通常パネル用は &AppState で十分なのでラッパを介す


fn draw_header(f: &mut Frame, area: Rect, state: &AppState) {
    let line = Line::from(vec![
        Span::styled("termrain  ", Style::default().fg(Color::Cyan)),
        Span::raw(format!(
            "📍 {} ({:.3}, {:.3})  ",
            state.config.location.name, state.config.location.latitude, state.config.location.longitude
        )),
        Span::styled(
            format!("[{}]", state.provider_name),
            Style::default().fg(Color::Gray),
        ),
    ]);
    f.render_widget(Paragraph::new(line), area);
}

fn draw_footer(f: &mut Frame, area: Rect, state: &AppState) {
    let play_mark = if state.radar_playing { "▶ " } else { "" };
    let map_label = match state.config.radar.map_style {
        crate::config::MapStyle::GsiStd => "標準",
        crate::config::MapStyle::CartoVoyager => "Carto",
        crate::config::MapStyle::GsiPhoto => "航空",
    };
    let mut spans = vec![Span::styled(
        format!(
            "[q]終了 [r]更新 [+/-]ズーム [hjkl]移動 [, .]時刻 [p]{}再生 [m]地図:{}",
            play_mark, map_label
        ),
        Style::default().fg(Color::Gray),
    )];
    if let Some(err) = &state.last_error {
        spans.push(Span::raw("  "));
        spans.push(Span::styled(
            format!("⚠ {}", err),
            Style::default().fg(Color::Red),
        ));
    }
    f.render_widget(Paragraph::new(Line::from(spans)), area);
}

// 共通ブロック生成（タイトル付きの枠）
//
// 色について:
//   - 枠線は Gray にして、半透明背景の wezterm でも視認できる明るさにする
//     （DarkGray は暗すぎて溶ける）
//   - タイトル文字は Cyan＋太字で目立たせる
fn titled_block(title: &str) -> Block<'_> {
    Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Gray))
        .title(Span::styled(
            title.to_string(),
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(ratatui::style::Modifier::BOLD),
        ))
}
