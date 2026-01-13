//! Signaling server protocol
//!
//! WebSocket-based signaling for WebRTC connection establishment.

use ada_remote_core::{Result, SessionId};
use serde::{Deserialize, Serialize};

/// Signaling message types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum SignalingMessage {
    /// Register a new session
    Register {
        session_id: SessionId,
    },
    /// Join an existing session
    Join {
        session_id: SessionId,
    },
    /// WebRTC offer
    Offer {
        session_id: SessionId,
        sdp: String,
    },
    /// WebRTC answer
    Answer {
        session_id: SessionId,
        sdp: String,
    },
    /// ICE candidate
    IceCandidate {
        session_id: SessionId,
        candidate: String,
    },
    /// Error response
    Error {
        message: String,
    },
}

/// Signaling client for WebRTC negotiation
pub struct SignalingClient {
    server_url: String,
}

impl SignalingClient {
    /// Create a new signaling client
    pub fn new(server_url: String) -> Self {
        Self { server_url }
    }

    /// Connect to the signaling server
    pub async fn connect(&mut self) -> Result<()> {
        tracing::info!("Connecting to signaling server: {}", self.server_url);
        // TODO: Implement WebSocket connection
        Ok(())
    }

    /// Send a signaling message
    pub async fn send(&mut self, _message: SignalingMessage) -> Result<()> {
        // TODO: Send message over WebSocket
        Ok(())
    }

    /// Receive a signaling message
    pub async fn receive(&mut self) -> Result<SignalingMessage> {
        // TODO: Receive message from WebSocket
        Err(ada_remote_core::Error::Network(
            "Signaling not implemented".to_string(),
        ))
    }

    /// Disconnect from the signaling server
    pub async fn disconnect(&mut self) -> Result<()> {
        tracing::info!("Disconnecting from signaling server");
        Ok(())
    }
}
