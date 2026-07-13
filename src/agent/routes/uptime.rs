use axum::Json;
use reqwest::StatusCode;
use serde::Serialize;
use std::fs;

#[derive(Serialize)]
pub struct UptimeResponse {
    days: u64,
    hours: u64,
    minutes: u64,
    seconds: u64,
    total_seconds: u64,
}

// ? Linux only atm
pub async fn uptime() -> Result<Json<UptimeResponse>, StatusCode> {
    let uptime =
        fs::read_to_string("/proc/uptime").map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let seconds = uptime
        .split_whitespace()
        .next()
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?
        .parse::<f64>()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)? as u64;

    let days = seconds / 86_400;
    let hours = (seconds % 86_400) / 3_600;
    let minutes = (seconds % 3_600) / 60;
    let secs = seconds % 60;

    Ok(Json(UptimeResponse {
        days,
        hours,
        minutes,
        seconds: secs,
        total_seconds: seconds,
    }))
}
