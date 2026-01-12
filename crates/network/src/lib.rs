//! Ada Remote Networking
//!
//! Network layer supporting WebRTC and QUIC protocols for peer-to-peer
//! remote desktop connections with NAT traversal.

use ada_remote_core::{ProtocolMessage, Result, SessionId};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tokio::sync::mpsc;

pub mod signaling;
pub mod webrtc;

/// Connection type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionType {
    /// WebRTC data channel (preferred)
    WebRTC,
    /// QUIC fallback
    QUIC,
}

/// Network configuration
#[derive(Debug, Clone)]
pub struct NetworkConfig {
    /// Signaling server URL
    pub signaling_server: String,
    /// STUN servers for NAT traversal
    pub stun_servers: Vec<String>,
    /// TURN servers for relay
    pub turn_servers: Vec<TurnServer>,
    /// Enable QUIC fallback
    pub enable_quic_fallback: bool,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            signaling_server: "wss://signal.ada-remote.io".to_string(),
            stun_servers: vec![
                "stun:stun.l.google.com:19302".to_string(),
                "stun:stun1.l.google.com:19302".to_string(),
            ],
            turn_servers: vec![],
            enable_quic_fallback: true,
        }
    }
}

/// TURN server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TurnServer {
    pub url: String,
    pub username: String,
    pub credential: String,
}

/// Connection state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionState {
    Disconnected,
    Connecting,
    Connected,
    Failed,
}

/// Network peer representing a remote connection
pub struct NetworkPeer {
    session_id: SessionId,
    connection_type: ConnectionType,
    state: ConnectionState,
    message_tx: mpsc::UnboundedSender<ProtocolMessage>,
    message_rx: mpsc::UnboundedReceiver<ProtocolMessage>,
}

impl NetworkPeer {
    /// Create a new network peer
    pub fn new(session_id: SessionId, connection_type: ConnectionType) -> Self {
        let (message_tx, message_rx) = mpsc::unbounded_channel();

        Self {
            session_id,
            connection_type,
            state: ConnectionState::Disconnected,
            message_tx,
            message_rx,
        }
    }

    /// Get the session ID
    pub fn session_id(&self) -> SessionId {
        self.session_id
    }

    /// Get the connection type
    pub fn connection_type(&self) -> ConnectionType {
        self.connection_type
    }

    /// Get the connection state
    pub fn state(&self) -> ConnectionState {
        self.state
    }

    /// Send a protocol message
    pub fn send(&self, message: ProtocolMessage) -> Result<()> {
        self.message_tx
            .send(message)
            .map_err(|e| ada_remote_core::Error::Network(format!("Failed to send message: {}", e)))
    }

    /// Receive a protocol message
    pub async fn receive(&mut self) -> Option<ProtocolMessage> {
        self.message_rx.recv().await
    }

    /// Connect to a remote peer
    pub async fn connect(&mut self, _config: &NetworkConfig) -> Result<()> {
        self.state = ConnectionState::Connecting;
        tracing::info!("Connecting to peer via {:?}", self.connection_type);

        // TODO: Implement actual connection logic
        // 1. Connect to signaling server
        // 2. Exchange SDP offers/answers for WebRTC
        // 3. Establish ICE candidates
        // 4. Set up data channels

        self.state = ConnectionState::Connected;
        Ok(())
    }

    /// Disconnect from the peer
    pub async fn disconnect(&mut self) -> Result<()> {
        tracing::info!("Disconnecting from peer");
        self.state = ConnectionState::Disconnected;
        Ok(())
    }
}

/// Create a new host peer (waiting for incoming connection)
pub async fn create_host(config: NetworkConfig) -> Result<NetworkPeer> {
    let session_id = SessionId::new();
    tracing::info!("Creating host with session ID: {}", session_id);

    let mut peer = NetworkPeer::new(session_id, ConnectionType::WebRTC);
    peer.state = ConnectionState::Connecting;

    // TODO: Register with signaling server and wait for connection

    Ok(peer)
}

/// Create a client peer (connecting to a host)
pub async fn create_client(session_id: SessionId, config: NetworkConfig) -> Result<NetworkPeer> {
    tracing::info!("Creating client for session ID: {}", session_id);

    let mut peer = NetworkPeer::new(session_id, ConnectionType::WebRTC);
    peer.connect(&config).await?;

    Ok(peer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network_config_default() {
        let config = NetworkConfig::default();
        assert!(!config.stun_servers.is_empty());
        assert!(config.enable_quic_fallback);
    }

    #[test]
    fn test_peer_creation() {
        let session_id = SessionId::new();
        let peer = NetworkPeer::new(session_id, ConnectionType::WebRTC);
        assert_eq!(peer.state(), ConnectionState::Disconnected);
        assert_eq!(peer.connection_type(), ConnectionType::WebRTC);
    }
}
