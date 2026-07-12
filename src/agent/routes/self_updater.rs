use axum::{Json, http::StatusCode};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct UpdateRequest {
    url: String,
}

#[derive(Serialize)]
pub struct UpdateResponse {
    success: bool,
    message: String,
}

pub async fn self_update(
    Json(req): Json<UpdateRequest>,
) -> Result<Json<UpdateResponse>, (StatusCode, String)> {
    log::info!("Processing update request...");
    log::info!("{}", req.url);

    let bytes = reqwest::get(&req.url)
        .await
        .map_err(internal_error)?
        .bytes()
        .await
        .map_err(internal_error)?;

    let tmp = std::env::temp_dir().join("kizuna.new");

    tokio::fs::write(&tmp, &bytes)
        .await
        .map_err(internal_error)?;

    self_replace::self_replace(&tmp).map_err(internal_error)?;

    let _ = std::fs::remove_file(&tmp);

    // future systemd/sys services handling
    tokio::spawn(async {
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        std::process::exit(0);
    });

    log::info!("Update installed. Restarting...");
    Ok(Json(UpdateResponse {
        success: true,
        message: "Update installed. Restarting...".into(),
    }))
}

fn internal_error<E: std::fmt::Display>(err: E) -> (StatusCode, String) {
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
