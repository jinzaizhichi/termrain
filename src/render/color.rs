// 降水強度 (mm/h) → ratatui::Color のマッピング
//
// 気象庁ナウキャストの凡例に近い色階調にしている。
//   0:      透明（描画しない）
//   1未満:  灰
//   1-5:    水色
//   5-10:   青
//   10-20:  黄
//   20-30:  橙
//   30-50:  赤
//   50-80:  紫
//   80+:    濃紫

use ratatui::style::Color;

pub fn precipitation_color(mmh: f64) -> Option<Color> {
    if mmh < 0.1 {
        None
    } else if mmh < 1.0 {
        Some(Color::Rgb(170, 220, 240))
    } else if mmh < 5.0 {
        Some(Color::Rgb(100, 200, 240))
    } else if mmh < 10.0 {
        Some(Color::Rgb(50, 100, 220))
    } else if mmh < 20.0 {
        Some(Color::Rgb(250, 240, 80))
    } else if mmh < 30.0 {
        Some(Color::Rgb(250, 170, 50))
    } else if mmh < 50.0 {
        Some(Color::Rgb(240, 70, 70))
    } else if mmh < 80.0 {
        Some(Color::Rgb(200, 60, 200))
    } else {
        Some(Color::Rgb(120, 30, 130))
    }
}
