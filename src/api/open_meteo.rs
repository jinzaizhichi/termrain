// Open-Meteo プロバイダー実装。
// API キー不要、世界対応、JSON で返ってくるシンプルな仕様。
// https://open-meteo.com/en/docs

use anyhow::{Context, Result};
use async_trait::async_trait;
use chrono::{DateTime, Local, NaiveDate, NaiveDateTime, TimeZone};
use serde::Deserialize;

use super::{
    CurrentWeather, DailyPoint, HourlyPoint, RadarGrid, WeatherIcon, WeatherProvider,
};

pub struct OpenMeteo {
    client: reqwest::Client,
}

impl OpenMeteo {
    pub fn new() -> Self {
        // User-Agent を付けるのが行儀。何かあった時に運営側がブロックしやすくなる
        // ぶん、こちらのアプリも特定しやすくなる（双方にメリット）。
        let client = reqwest::Client::builder()
            .user_agent("termrain/0.1 (+https://github.com/iorinu/termrain)")
            .timeout(std::time::Duration::from_secs(15))
            .build()
            .expect("reqwest クライアントの構築に失敗");
        Self { client }
    }
}

impl Default for OpenMeteo {
    fn default() -> Self {
        Self::new()
    }
}

// ===== レスポンスの型 =====
//
// JSON のフィールドをそのまま映す構造体。
// serde の rename_all を使わず、API 側のキーがスネークケースなのでそのままで OK。

#[derive(Debug, Deserialize)]
struct ForecastResponse {
    current: Option<CurrentBlock>,
    hourly: Option<HourlyBlock>,
    daily: Option<DailyBlock>,
}

#[derive(Debug, Deserialize)]
struct CurrentBlock {
    time: String,
    temperature_2m: f64,
    relative_humidity_2m: Option<f64>,
    weather_code: u32,
    wind_speed_10m: Option<f64>,
    wind_direction_10m: Option<f64>,
}

#[derive(Debug, Deserialize)]
struct HourlyBlock {
    time: Vec<String>,
    temperature_2m: Vec<f64>,
    precipitation: Vec<f64>,
    precipitation_probability: Option<Vec<Option<f64>>>,
}

#[derive(Debug, Deserialize)]
struct DailyBlock {
    time: Vec<String>,
    weather_code: Vec<u32>,
    temperature_2m_max: Vec<Option<f64>>,
    temperature_2m_min: Vec<Option<f64>>,
    precipitation_probability_max: Option<Vec<Option<f64>>>,
}

// ===== WMO 天気コード → アイコン / 文字列 =====
// https://open-meteo.com/en/docs (Weather variable documentation)
fn wmo_to_icon(code: u32) -> WeatherIcon {
    match code {
        0 => WeatherIcon::Sunny,
        1..=2 => WeatherIcon::PartlyCloudy,
        3 => WeatherIcon::Cloudy,
        45 | 48 => WeatherIcon::Cloudy, // 霧
        51..=67 | 80..=82 => WeatherIcon::Rain,
        71..=77 | 85 | 86 => WeatherIcon::Snow,
        95..=99 => WeatherIcon::Thunder,
        _ => WeatherIcon::Unknown,
    }
}

fn wmo_to_text(code: u32) -> &'static str {
    match code {
        0 => "快晴",
        1 => "晴れ",
        2 => "晴れ時々曇り",
        3 => "曇り",
        45 | 48 => "霧",
        51 | 53 | 55 => "霧雨",
        61 | 63 | 65 => "雨",
        66 | 67 => "凍雨",
        71 | 73 | 75 => "雪",
        77 => "霧雪",
        80 | 81 | 82 => "にわか雨",
        85 | 86 => "にわか雪",
        95 => "雷雨",
        96 | 99 => "雷雨（雹あり）",
        _ => "不明",
    }
}

/// ISO8601 (例: "2026-06-08T12:00") を Local 時刻として解釈する。
/// Open-Meteo のデフォルトはタイムゾーンが無い「UTC ベース」だが、
/// このアプリでは timezone=auto を付けてリクエストするので、
/// 返ってくる値はその地点のローカル時刻と解釈してよい。
fn parse_local(s: &str) -> Result<DateTime<Local>> {
    let ndt = NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M")
        .with_context(|| format!("時刻パース失敗: {s}"))?;
    Local
        .from_local_datetime(&ndt)
        .single()
        .context("ローカル時刻の単一解決に失敗")
}

#[async_trait]
impl WeatherProvider for OpenMeteo {
    fn name(&self) -> &'static str {
        "Open-Meteo"
    }

    async fn current(&self, lat: f64, lon: f64) -> Result<CurrentWeather> {
        let url = format!(
            "https://api.open-meteo.com/v1/forecast?latitude={lat}&longitude={lon}\
             &current=temperature_2m,relative_humidity_2m,weather_code,wind_speed_10m,wind_direction_10m\
             &timezone=auto"
        );
        let resp: ForecastResponse = self
            .client
            .get(&url)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;
        let cur = resp.current.context("Open-Meteo: current が無い")?;
        Ok(CurrentWeather {
            observed_at: parse_local(&cur.time)?,
            condition: wmo_to_text(cur.weather_code).to_string(),
            icon: wmo_to_icon(cur.weather_code),
            temperature_c: cur.temperature_2m,
            humidity_pct: cur.relative_humidity_2m,
            wind_speed_ms: cur.wind_speed_10m,
            wind_direction_deg: cur.wind_direction_10m,
        })
    }

    async fn hourly(&self, lat: f64, lon: f64) -> Result<Vec<HourlyPoint>> {
        let url = format!(
            "https://api.open-meteo.com/v1/forecast?latitude={lat}&longitude={lon}\
             &hourly=temperature_2m,precipitation,precipitation_probability\
             &forecast_days=2&timezone=auto"
        );
        let resp: ForecastResponse = self
            .client
            .get(&url)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;
        let h = resp.hourly.context("Open-Meteo: hourly が無い")?;
        let mut out = Vec::with_capacity(h.time.len());
        for i in 0..h.time.len() {
            out.push(HourlyPoint {
                time: parse_local(&h.time[i])?,
                temperature_c: h.temperature_2m[i],
                precipitation_mm: h.precipitation[i],
                precipitation_prob_pct: h
                    .precipitation_probability
                    .as_ref()
                    .and_then(|v| v.get(i).copied().flatten()),
            });
        }
        Ok(out)
    }

    async fn daily(&self, lat: f64, lon: f64) -> Result<Vec<DailyPoint>> {
        let url = format!(
            "https://api.open-meteo.com/v1/forecast?latitude={lat}&longitude={lon}\
             &daily=weather_code,temperature_2m_max,temperature_2m_min,precipitation_probability_max\
             &forecast_days=7&timezone=auto"
        );
        let resp: ForecastResponse = self
            .client
            .get(&url)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;
        let d = resp.daily.context("Open-Meteo: daily が無い")?;
        let mut out = Vec::with_capacity(d.time.len());
        for i in 0..d.time.len() {
            let date = NaiveDate::parse_from_str(&d.time[i], "%Y-%m-%d")?;
            let code = d.weather_code[i];
            out.push(DailyPoint {
                date,
                condition: wmo_to_text(code).into(),
                icon: wmo_to_icon(code),
                temp_max_c: d.temperature_2m_max[i],
                temp_min_c: d.temperature_2m_min[i],
                precipitation_prob_pct: d
                    .precipitation_probability_max
                    .as_ref()
                    .and_then(|v| v.get(i).copied().flatten()),
            });
        }
        Ok(out)
    }

    async fn radar(&self, lat: f64, lon: f64, zoom: u8, _time_offset: i32) -> Result<RadarGrid> {
        // Open-Meteo はピクセルレーダーは無いので、緯度経度グリッドで
        // 「次の 1 時間の降水量」を取って近似する。
        // zoom は「表示範囲の半径（度）」に変換: zoom=8 → ±0.5度, 大きいほど狭い
        let half_deg = match zoom {
            ..=5 => 2.0,
            6 => 1.5,
            7 => 1.0,
            8 => 0.5,
            9 => 0.3,
            _ => 0.15,
        };
        let width: usize = 32;
        let height: usize = 16;

        // 1 リクエストで複数地点を取れるので、緯度経度を全部並べる
        let mut lats = Vec::with_capacity(width * height);
        let mut lons = Vec::with_capacity(width * height);
        for j in 0..height {
            // y は北が上なので上から南へ
            let lat_j = lat + half_deg - (2.0 * half_deg) * (j as f64) / (height as f64 - 1.0);
            for i in 0..width {
                let lon_i = lon - half_deg + (2.0 * half_deg) * (i as f64) / (width as f64 - 1.0);
                lats.push(format!("{lat_j:.4}"));
                lons.push(format!("{lon_i:.4}"));
            }
        }

        let url = format!(
            "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}\
             &current=precipitation&timezone=auto",
            lats.join(","),
            lons.join(",")
        );

        // 複数地点モードでは JSON は配列で返るので、Vec<...> として受け取る
        #[derive(Deserialize)]
        struct MultiCurrent {
            current: Option<MultiCurrentInner>,
        }
        #[derive(Deserialize)]
        struct MultiCurrentInner {
            time: String,
            precipitation: f64,
        }

        let arr: Vec<MultiCurrent> = self
            .client
            .get(&url)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        let mut data = vec![vec![0.0f64; width]; height];
        let mut observed = None;
        for (idx, item) in arr.iter().enumerate() {
            if let Some(c) = &item.current {
                let y = idx / width;
                let x = idx % width;
                if y < height {
                    data[y][x] = c.precipitation.max(0.0);
                }
                if observed.is_none() {
                    observed = Some(parse_local(&c.time)?);
                }
            }
        }

        Ok(RadarGrid {
            width,
            height,
            data,
            map_dots: Vec::new(), // Open-Meteo は背景地図なし
            composite_image: None,
            bounds: ((lat - half_deg, lon - half_deg), (lat + half_deg, lon + half_deg)),
            observed_at: observed.unwrap_or_else(Local::now),
        })
    }
}
