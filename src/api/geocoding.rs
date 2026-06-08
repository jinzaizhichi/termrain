// 都市名 → 緯度経度 (+ 国コード) の解決。
// Open-Meteo Geocoding API を利用（無料・キー不要）。

use anyhow::{Context, Result};
use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct GeoHit {
    pub name: String,
    pub country: String,        // ISO2: "JP", "FR" など
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug, Deserialize)]
struct Resp {
    results: Option<Vec<RespHit>>,
}
#[derive(Debug, Deserialize)]
struct RespHit {
    name: String,
    latitude: f64,
    longitude: f64,
    country_code: Option<String>,
}

pub async fn search(client: &reqwest::Client, query: &str) -> Result<GeoHit> {
    let url = format!(
        "https://geocoding-api.open-meteo.com/v1/search?name={}&count=1&language=ja",
        urlencoding::encode(query)
    );
    let r: Resp = client.get(&url).send().await?.error_for_status()?.json().await?;
    let hit = r
        .results
        .and_then(|mut v| v.pop())
        .context("該当する地点が見つかりません")?;
    Ok(GeoHit {
        name: hit.name,
        country: hit.country_code.unwrap_or_default(),
        latitude: hit.latitude,
        longitude: hit.longitude,
    })
}
