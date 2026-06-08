// カラーテーマ
//
// Catppuccin Mocha インスパイアの統一パレット。
// 各パネルから色を一元参照することで、後で別テーマに切り替えやすい。

use ratatui::style::Color;

/// パネル背景。レーダー画像を除き、半透明背景でも視認できる程度の暗色。
pub const BG: Color = Color::Rgb(30, 30, 46);
/// 通常テキスト
pub const FG: Color = Color::Rgb(205, 214, 244);
/// アクセント（タイトル等）。青系。
pub const ACCENT: Color = Color::Rgb(137, 180, 250);
/// 第二アクセント。紫系。ヘッダーやステータス用。
pub const ACCENT_2: Color = Color::Rgb(203, 166, 247);
/// 成功 / 気温 高め
#[allow(dead_code)]
pub const SUCCESS: Color = Color::Rgb(166, 227, 161);
/// 警告 / 気温
pub const WARN: Color = Color::Rgb(249, 226, 175);
/// エラー / 強い雨
pub const ERROR: Color = Color::Rgb(243, 139, 168);
/// 補助情報 (sub label, hint)
pub const SUBTLE: Color = Color::Rgb(127, 132, 156);
/// パネル枠線
pub const BORDER: Color = Color::Rgb(88, 91, 112);
/// 気温（赤系）
pub const TEMP: Color = Color::Rgb(243, 139, 168);
/// 降水（青系）
pub const RAIN: Color = Color::Rgb(137, 180, 250);
