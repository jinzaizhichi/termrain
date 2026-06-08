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

    // 縦方向: ヘッダー、上段（左:現在天気 中:レーダー 右:週間予報サイド）、
    //         時間別予報、フッター。下段の週間予報は廃止し、サイドに縦並びで配置することで
    //         レーダー右の余白を埋め、全体のバランスを取る。
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // ヘッダー
            Constraint::Min(18),   // 上段: 現在 + レーダー + 週間サイド
            Constraint::Length(8), // 時間別予報
            Constraint::Length(1), // フッター
        ])
        .split(size);

    draw_header(f, chunks[0], state);

    // 上段の高さからレーダー画像のアスペクト比に応じた幅を計算する。
    // フォント1セルは縦長（横:縦 ≒ 1:2）なので、画像が正方形(aspect=1)なら
    // 見た目正方形にはセル幅 = 高さ × 2 が必要。
    let radar_aspect = if let Some(img) = state
        .radar
        .as_ref()
        .and_then(|r| r.composite_image.as_ref())
    {
        img.width() as f32 / img.height() as f32
    } else {
        1.0
    };
    let radar_h = chunks[1].height;
    let mut radar_w_cells = ((radar_h as f32) * 2.0 * radar_aspect) as u16;

    // 左 (現在天気) と 右 (週間予報サイド) の最低幅を確保する。
    // 週間サイドは「06/09(月)」が入る最小幅として 16 セル欲しい。
    const LEFT_W: u16 = 28;
    const SIDE_MIN_W: u16 = 18;
    let total_w = chunks[1].width;
    let reserved = LEFT_W + SIDE_MIN_W;
    if radar_w_cells + reserved > total_w {
        radar_w_cells = total_w.saturating_sub(reserved);
    }
    let side_w = total_w.saturating_sub(LEFT_W + radar_w_cells);

    let top = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(LEFT_W),         // 現在の天気
            Constraint::Length(radar_w_cells),  // レーダー
            Constraint::Length(side_w),         // 週間予報サイド
        ])
        .split(chunks[1]);
    current::draw(f, top[0], state);
    radar::draw(f, top[1], state);
    weekly::draw(f, top[2], state);

    hourly::draw(f, chunks[2], state);
    draw_footer(f, chunks[3], state);
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
