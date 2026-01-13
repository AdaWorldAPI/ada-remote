//! Ada Remote Core
//!
//! Core types, traits, and protocol definitions for Ada Remote.

use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

/// Session identifier - unique ID for each remote session
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SessionId(Uuid);

impl SessionId {
    /// Generate a new random session ID
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    /// Create from a string representation
    pub fn from_string(s: &str) -> std::result::Result<Self, uuid::Error> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

impl Default for SessionId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for SessionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Display as 9-digit code for easier user entry
        write!(f, "{:09}", (self.0.as_u128() % 1_000_000_000) as u32)
    }
}

/// Connection mode for a remote session
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConnectionMode {
    /// View-only mode - no input control
    ViewOnly,
    /// Full control - keyboard and mouse input enabled
    FullControl,
    /// File transfer only
    FileTransfer,
}

/// Session configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionConfig {
    pub session_id: SessionId,
    pub mode: ConnectionMode,
    pub password_hash: Option<String>,
    pub clipboard_sync: bool,
    pub quality: VideoQuality,
}

/// Video quality settings
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VideoQuality {
    Low,      // 720p, 30fps, high compression
    Medium,   // 1080p, 30fps, medium compression
    High,     // 1080p, 60fps, low compression
    Adaptive, // Adjust based on network conditions
}

impl Default for VideoQuality {
    fn default() -> Self {
        Self::Adaptive
    }
}

/// Message types for the Ada Remote protocol
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProtocolMessage {
    /// Request to establish a new session
    SessionRequest {
        session_id: SessionId,
        password: Option<String>,
        mode: ConnectionMode,
    },
    /// Response to session request
    SessionResponse {
        accepted: bool,
        reason: Option<String>,
    },
    /// Heartbeat to keep connection alive
    Heartbeat,
    /// Video frame data
    VideoFrame {
        timestamp: u64,
        data: Vec<u8>,
    },
    /// Input event (keyboard/mouse)
    InputEvent {
        event_type: InputEventType,
        data: Vec<u8>,
    },
    /// Clipboard data
    Clipboard {
        content: String,
    },
    /// File transfer initiation
    FileTransferStart {
        file_name: String,
        file_size: u64,
        transfer_id: Uuid,
    },
    /// File transfer chunk
    FileTransferChunk {
        transfer_id: Uuid,
        chunk_index: u64,
        data: Vec<u8>,
    },
    /// File transfer complete
    FileTransferComplete {
        transfer_id: Uuid,
    },
    /// Session termination
    Disconnect {
        reason: String,
    },
}

/// Input event types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InputEventType {
    KeyPress,
    KeyRelease,
    MouseMove,
    MouseButtonPress,
    MouseButtonRelease,
    MouseScroll,
}

/// Result type for Ada Remote operations
pub type Result<T> = std::result::Result<T, Error>;

/// Error types for Ada Remote
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Network error: {0}")]
    Network(String),

    #[error("Authentication failed: {0}")]
    Authentication(String),

    #[error("Session error: {0}")]
    Session(String),

    #[error("Encoding error: {0}")]
    Encoding(String),

    #[error("Decoding error: {0}")]
    Decoding(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_id_generation() {
        let id1 = SessionId::new();
        let id2 = SessionId::new();
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_session_id_display() {
        let id = SessionId::new();
        let display = format!("{}", id);
        assert_eq!(display.len(), 9);
    }
}
