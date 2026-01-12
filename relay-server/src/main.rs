//! Ada Remote Relay Server
//!
//! WebSocket-based signaling server for WebRTC connection establishment.
//! Also provides TURN relay functionality for NAT traversal.

use ada_remote_core::SessionId;
use anyhow::Result;
use clap::Parser;
use futures::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::RwLock;
use tokio_tungstenite::{accept_async, tungstenite::Message};
use tracing::{error, info, warn};

/// Command-line arguments
#[derive(Parser, Debug)]
#[command(name = "Ada Remote Relay Server")]
#[command(about = "Signaling and relay server for Ada Remote", long_about = None)]
struct Args {
    /// Address to bind to
    #[arg(short, long, default_value = "0.0.0.0:8080")]
    bind: SocketAddr,

    /// Enable verbose logging
    #[arg(short, long)]
    verbose: bool,
}

/// Signaling message types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum SignalingMessage {
    Register {
        session_id: String,
    },
    Join {
        session_id: String,
    },
    Offer {
        session_id: String,
        sdp: String,
    },
    Answer {
        session_id: String,
        sdp: String,
    },
    IceCandidate {
        session_id: String,
        candidate: String,
    },
    Success {
        message: String,
    },
    Error {
        message: String,
    },
}

/// Active session
struct Session {
    session_id: SessionId,
    host_addr: Option<SocketAddr>,
    client_addr: Option<SocketAddr>,
}

/// Server state
struct ServerState {
    sessions: HashMap<String, Session>,
}

impl ServerState {
    fn new() -> Self {
        Self {
            sessions: HashMap::new(),
        }
    }
}

type SharedState = Arc<RwLock<ServerState>>;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(if args.verbose {
            tracing::Level::DEBUG
        } else {
            tracing::Level::INFO
        })
        .init();

    info!("Ada Remote Relay Server starting on {}", args.bind);

    let state = Arc::new(RwLock::new(ServerState::new()));
    let listener = TcpListener::bind(args.bind).await?;

    info!("Relay server listening on {}", args.bind);

    while let Ok((stream, addr)) = listener.accept().await {
        info!("New connection from {}", addr);
        let state = Arc::clone(&state);
        tokio::spawn(async move {
            if let Err(e) = handle_connection(stream, addr, state).await {
                error!("Error handling connection from {}: {}", addr, e);
            }
        });
    }

    Ok(())
}

async fn handle_connection(
    stream: TcpStream,
    addr: SocketAddr,
    state: SharedState,
) -> Result<()> {
    let ws_stream = accept_async(stream).await?;
    info!("WebSocket connection established with {}", addr);

    let (mut ws_sender, mut ws_receiver) = ws_stream.split();

    while let Some(msg) = ws_receiver.next().await {
        let msg = match msg {
            Ok(msg) => msg,
            Err(e) => {
                error!("Error receiving message: {}", e);
                break;
            }
        };

        if !msg.is_text() {
            continue;
        }

        let text = msg.to_text()?;
        let signaling_msg: SignalingMessage = match serde_json::from_str(text) {
            Ok(msg) => msg,
            Err(e) => {
                warn!("Invalid message format: {}", e);
                let error_msg = SignalingMessage::Error {
                    message: "Invalid message format".to_string(),
                };
                let response = serde_json::to_string(&error_msg)?;
                ws_sender.send(Message::Text(response)).await?;
                continue;
            }
        };

        let response = handle_signaling_message(signaling_msg, addr, &state).await?;
        let response_text = serde_json::to_string(&response)?;
        ws_sender.send(Message::Text(response_text)).await?;
    }

    info!("Connection closed for {}", addr);
    Ok(())
}

async fn handle_signaling_message(
    msg: SignalingMessage,
    addr: SocketAddr,
    state: &SharedState,
) -> Result<SignalingMessage> {
    match msg {
        SignalingMessage::Register { session_id } => {
            info!("Registering new session: {} from {}", session_id, addr);
            let mut state = state.write().await;

            let parsed_session_id = SessionId::from_string(&session_id)
                .map_err(|_| anyhow::anyhow!("Invalid session ID"))?;

            state.sessions.insert(
                session_id.clone(),
                Session {
                    session_id: parsed_session_id,
                    host_addr: Some(addr),
                    client_addr: None,
                },
            );

            Ok(SignalingMessage::Success {
                message: format!("Session {} registered", session_id),
            })
        }
        SignalingMessage::Join { session_id } => {
            info!("Client joining session: {} from {}", session_id, addr);
            let mut state = state.write().await;

            if let Some(session) = state.sessions.get_mut(&session_id) {
                session.client_addr = Some(addr);
                Ok(SignalingMessage::Success {
                    message: format!("Joined session {}", session_id),
                })
            } else {
                Ok(SignalingMessage::Error {
                    message: "Session not found".to_string(),
                })
            }
        }
        SignalingMessage::Offer { session_id, sdp } => {
            info!("Received offer for session: {}", session_id);
            // TODO: Forward offer to the other peer
            Ok(SignalingMessage::Success {
                message: "Offer received".to_string(),
            })
        }
        SignalingMessage::Answer { session_id, sdp } => {
            info!("Received answer for session: {}", session_id);
            // TODO: Forward answer to the other peer
            Ok(SignalingMessage::Success {
                message: "Answer received".to_string(),
            })
        }
        SignalingMessage::IceCandidate {
            session_id,
            candidate,
        } => {
            info!("Received ICE candidate for session: {}", session_id);
            // TODO: Forward ICE candidate to the other peer
            Ok(SignalingMessage::Success {
                message: "ICE candidate received".to_string(),
            })
        }
        _ => Ok(SignalingMessage::Error {
            message: "Invalid message type".to_string(),
        }),
    }
}
