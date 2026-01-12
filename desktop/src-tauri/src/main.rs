// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use ada_remote_core::{SessionId, SessionConfig, ConnectionMode, VideoQuality};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{info, error};

/// Application state
struct AppState {
    current_session: Option<SessionConfig>,
}

/// Session information for the UI
#[derive(Debug, Clone, Serialize, Deserialize)]
struct SessionInfo {
    session_id: String,
    mode: String,
    status: String,
}

/// Start hosting a remote session
#[tauri::command]
async fn start_host_session(
    password: Option<String>,
    state: tauri::State<'_, Arc<Mutex<AppState>>>,
) -> Result<SessionInfo, String> {
    info!("Starting host session");

    let session_id = SessionId::new();
    let password_hash = if let Some(pwd) = password {
        Some(ada_remote_crypto::hash_password(&pwd).map_err(|e| e.to_string())?)
    } else {
        None
    };

    let config = SessionConfig {
        session_id,
        mode: ConnectionMode::FullControl,
        password_hash,
        clipboard_sync: true,
        quality: VideoQuality::Adaptive,
    };

    let mut app_state = state.lock().await;
    app_state.current_session = Some(config.clone());

    Ok(SessionInfo {
        session_id: format!("{}", session_id),
        mode: "host".to_string(),
        status: "waiting".to_string(),
    })
}

/// Connect to a remote session
#[tauri::command]
async fn connect_to_session(
    session_id: String,
    password: Option<String>,
    state: tauri::State<'_, Arc<Mutex<AppState>>>,
) -> Result<SessionInfo, String> {
    info!("Connecting to session: {}", session_id);

    let session_id = SessionId::from_string(&session_id).map_err(|e| e.to_string())?;

    let config = SessionConfig {
        session_id,
        mode: ConnectionMode::FullControl,
        password_hash: password.map(|pwd| ada_remote_crypto::hash_password(&pwd).ok()).flatten(),
        clipboard_sync: true,
        quality: VideoQuality::Adaptive,
    };

    let mut app_state = state.lock().await;
    app_state.current_session = Some(config.clone());

    // TODO: Establish actual network connection

    Ok(SessionInfo {
        session_id: format!("{}", session_id),
        mode: "client".to_string(),
        status: "connecting".to_string(),
    })
}

/// Disconnect from current session
#[tauri::command]
async fn disconnect_session(
    state: tauri::State<'_, Arc<Mutex<AppState>>>,
) -> Result<(), String> {
    info!("Disconnecting session");

    let mut app_state = state.lock().await;
    app_state.current_session = None;

    Ok(())
}

/// Get current session information
#[tauri::command]
async fn get_session_info(
    state: tauri::State<'_, Arc<Mutex<AppState>>>,
) -> Result<Option<SessionInfo>, String> {
    let app_state = state.lock().await;

    if let Some(config) = &app_state.current_session {
        Ok(Some(SessionInfo {
            session_id: format!("{}", config.session_id),
            mode: format!("{:?}", config.mode),
            status: "connected".to_string(),
        }))
    } else {
        Ok(None)
    }
}

fn main() {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    info!("Ada Remote Desktop starting");

    let app_state = Arc::new(Mutex::new(AppState {
        current_session: None,
    }));

    tauri::Builder::default()
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            start_host_session,
            connect_to_session,
            disconnect_session,
            get_session_info,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
