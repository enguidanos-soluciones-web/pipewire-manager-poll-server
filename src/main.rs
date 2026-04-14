use crate::pipewire::PipeWireObject;
use crate::state::SharedMutableState;
use axum::Json;
use axum::extract::Path;
use axum::{Router, extract::State, routing::get};
use serde_json::Value;
use std::process::Stdio;
use std::time::Duration;
use tokio::process::Command;
use tokio::time::interval;
use tracing::instrument;

mod pipewire;
mod state;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    tracing_subscriber::fmt::init();

    let state = SharedMutableState::default();

    spawn_pw_dump_polling(state.clone()).await;

    let app = Router::new().nest(
        "/api",
        Router::new()
            .route("/healthz", get(healthz))
            .route("/pipewire/devices", get(pw_devices))
            .route("/pipewire/devices/{id}", get(pw_devices_by_id))
            .route("/pipewire/devices/{id}/profiles", get(pw_devices_profiles_by_id))
            .route("/pipewire/nodes", get(pw_nodes))
            .with_state(state),
    );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}

#[instrument(level = "info", ret, name = "Healthz")]
async fn healthz() -> &'static str {
    "OK"
}

#[instrument(level = "info", skip(state), ret, name = "PipeWireDevices")]
async fn pw_devices(State(state): State<SharedMutableState>) -> Json<Vec<PipeWireObject>> {
    let pw_dump = state.pw_dump.read().await;
    Json(pw_dump.find_devices())
}

#[instrument(level = "info", skip(state), ret, name = "PipeWireDevicesById")]
async fn pw_devices_by_id(State(state): State<SharedMutableState>, Path(id): Path<u32>) -> Json<Option<PipeWireObject>> {
    let pw_dump = state.pw_dump.read().await;
    Json(pw_dump.find_devices().iter().find(|a| a.id == id).cloned())
}

#[instrument(level = "info", skip(state), ret, name = "PipeWireDevicesProfilesById")]
async fn pw_devices_profiles_by_id(State(state): State<SharedMutableState>, Path(id): Path<u32>) -> Json<Option<PipeWireObject>> {
    let pw_dump = state.pw_dump.read().await;
    Json(pw_dump.find_devices().iter().find(|a| a.id == id).cloned())
}

#[instrument(level = "info", skip(state), ret, name = "PipeWireNodes")]
async fn pw_nodes(State(state): State<SharedMutableState>) -> Json<Vec<PipeWireObject>> {
    let pw_dump = state.pw_dump.read().await;
    Json(pw_dump.find_nodes())
}

async fn spawn_pw_dump_polling(state_writer: SharedMutableState) {
    tokio::spawn(async move {
        let mut timer = interval(Duration::from_secs(2));

        loop {
            timer.tick().await;

            let output = Command::new("pw-dump").stdout(Stdio::piped()).spawn();

            match output {
                Ok(child) => {
                    let result = child.wait_with_output().await;

                    match result {
                        Ok(out) => {
                            if out.status.success() {
                                if let Ok(json) = serde_json::from_slice::<Value>(&out.stdout) {
                                    let mut pw_dump = state_writer.pw_dump.write().await;

                                    if let Some(array) = json.as_array() {
                                        pw_dump.objects = array
                                            .clone()
                                            .into_iter()
                                            .filter_map(|a| serde_json::from_value::<PipeWireObject>(a).ok())
                                            .collect();
                                    }

                                    continue;
                                }
                            }

                            tracing::error!("[pw-dump] Failed with status {}", out.status);
                        }
                        Err(e) => tracing::error!("[pw-dump] {e}"),
                    }
                }
                Err(e) => tracing::error!("[pw-dump] {e}"),
            }
        }
    });
}
