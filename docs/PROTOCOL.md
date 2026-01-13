# Ada Remote Protocol Specification

## Overview

Ada Remote uses a hybrid approach for remote desktop connectivity:
- **WebRTC** for peer-to-peer communication (primary)
- **QUIC** as a fallback when WebRTC fails
- **WebSocket** for signaling and session negotiation
- **E2E encryption** using X25519 + ChaCha20-Poly1305

## Architecture

```
┌──────────┐                  ┌──────────────┐                  ┌──────────┐
│  Host    │◄────WebSocket────►│   Signaling  │◄────WebSocket────►│  Client  │
│          │                  │    Server    │                  │          │
└────┬─────┘                  └──────────────┘                  └─────┬────┘
     │                                                                 │
     │                        ┌──────────────┐                        │
     └────────WebRTC──────────►│  STUN/TURN  │◄─────WebRTC───────────┘
                              │    Server    │
                              └──────────────┘
```

## Connection Flow

### 1. Session Initialization (Host)

1. Host generates a **Session ID** (9-digit numeric code)
2. Host optionally sets a **password** (hashed with Argon2)
3. Host connects to **Signaling Server** via WebSocket
4. Host sends `Register` message with Session ID
5. Signaling server confirms registration

**Message:**
```json
{
  "type": "register",
  "session_id": "123456789"
}
```

### 2. Client Connection

1. Client enters Session ID from host
2. Client connects to Signaling Server
3. Client sends `Join` message with Session ID
4. Signaling server verifies session exists

**Message:**
```json
{
  "type": "join",
  "session_id": "123456789"
}
```

### 3. WebRTC Negotiation

#### Step 1: Key Exchange
1. Both peers generate **X25519 key pairs**
2. Public keys exchanged via signaling server
3. Both compute **shared secret** using ECDH
4. Shared secret used to derive encryption key

#### Step 2: SDP Offer/Answer
1. Host creates WebRTC **offer** (SDP)
2. Offer sent to client via signaling server
3. Client creates WebRTC **answer** (SDP)
4. Answer sent back to host

**Offer Message:**
```json
{
  "type": "offer",
  "session_id": "123456789",
  "sdp": "v=0\r\no=- 123456 2 IN IP4 127.0.0.1\r\n..."
}
```

#### Step 3: ICE Candidate Exchange
1. Both peers gather **ICE candidates** (STUN)
2. Candidates exchanged via signaling server
3. Peers attempt direct connection
4. If direct fails, use TURN relay

**ICE Candidate Message:**
```json
{
  "type": "ice_candidate",
  "session_id": "123456789",
  "candidate": "candidate:1 1 UDP 2130706431 192.168.1.100 54321 typ host"
}
```

### 4. Data Channel Establishment

Once WebRTC peer connection established:
1. Create **control channel** for protocol messages
2. Create **video channel** for encoded frames
3. Enable E2E encryption on all channels

## Protocol Messages

All messages sent over control channel are encrypted and serialized as JSON:

### Session Messages

#### `SessionRequest`
```json
{
  "type": "session_request",
  "session_id": "123456789",
  "password": "hashed_password_optional",
  "mode": "full_control"
}
```

#### `SessionResponse`
```json
{
  "type": "session_response",
  "accepted": true,
  "reason": null
}
```

### Video Streaming

#### `VideoFrame`
```json
{
  "type": "video_frame",
  "timestamp": 1234567890,
  "data": [byte_array]
}
```

- **Codec**: H.264 (primary) or VP9 (fallback)
- **Resolution**: Adaptive based on network
- **FPS**: 30-60 depending on quality setting
- **Keyframe interval**: 2 seconds

### Input Events

#### `InputEvent`
```json
{
  "type": "input_event",
  "event_type": "mouse_move",
  "data": [x, y, modifiers]
}
```

**Event Types:**
- `key_press` / `key_release`
- `mouse_move`
- `mouse_button_press` / `mouse_button_release`
- `mouse_scroll`

### File Transfer

#### `FileTransferStart`
```json
{
  "type": "file_transfer_start",
  "file_name": "document.pdf",
  "file_size": 1048576,
  "transfer_id": "uuid-v4"
}
```

#### `FileTransferChunk`
```json
{
  "type": "file_transfer_chunk",
  "transfer_id": "uuid-v4",
  "chunk_index": 0,
  "data": [byte_array]
}
```

**Chunk Size**: 64 KB
**Resume Support**: Yes (via chunk index)

### Clipboard Sync

#### `Clipboard`
```json
{
  "type": "clipboard",
  "content": "copied text content"
}
```

## Encryption

### Session Key Derivation

1. **Key Exchange**: X25519 ECDH
2. **Shared Secret**: 32 bytes
3. **Cipher**: ChaCha20-Poly1305
4. **Nonce**: Random 12 bytes per message
5. **Associated Data**: Session ID for authentication

### Message Encryption

```
plaintext = JSON.stringify(message)
nonce = random(12)
ciphertext = ChaCha20Poly1305.encrypt(
    key: shared_secret,
    nonce: nonce,
    plaintext: plaintext,
    aad: session_id
)

encrypted_message = {
    ciphertext: ciphertext,
    nonce: nonce
}
```

### Password Hashing

Session passwords hashed with **Argon2id**:
```
hash = Argon2id(
    password: user_password,
    salt: random(16),
    iterations: 3,
    memory: 64 MB,
    parallelism: 4
)
```

## Video Encoding

### Encoder Settings

**H.264 Profile**: High
**Preset**: ultrafast (low latency) or fast (better quality)
**Tune**: zerolatency
**Bitrate**: Adaptive (1-10 Mbps)
**GOP Size**: 60 frames (2s at 30fps)

### Adaptive Quality

Network conditions monitored:
- **RTT < 50ms**: High quality (1080p60, 5 Mbps)
- **RTT 50-100ms**: Medium (1080p30, 3 Mbps)
- **RTT > 100ms**: Low (720p30, 1.5 Mbps)
- **Packet loss > 5%**: Reduce bitrate 20%

## NAT Traversal

### STUN Servers (Public)
- `stun:stun.l.google.com:19302`
- `stun:stun1.l.google.com:19302`

### TURN Fallback

If direct connection fails:
1. Use TURN relay server
2. Host connects to TURN and allocates relay
3. Client connects to same relay
4. Traffic flows: Client → TURN → Host

**TURN Protocol**: RFC 5766
**Transport**: UDP (preferred) or TCP

## Security Considerations

1. **E2E Encryption**: All data encrypted between peers
2. **No Plain Credentials**: Passwords always hashed
3. **Session Expiry**: Sessions expire after 24 hours
4. **Rate Limiting**: Prevent brute force on signaling server
5. **No Telemetry**: Zero data collection

## Performance Targets

- **Latency**: < 100ms on local network
- **Latency**: < 200ms over internet
- **Frame Rate**: 30-60 FPS
- **Connection Time**: < 5 seconds
- **Bandwidth**: 1-5 Mbps typical

## Future Enhancements

- **Multi-monitor**: Support multiple displays
- **Wake-on-LAN**: Remote wake up
- **Session Recording**: Record remote sessions
- **Mobile Clients**: iOS/Android support
- **IPv6**: Full IPv6 support
- **UDP Hole Punching**: Improve NAT traversal

## References

- [WebRTC Specification](https://www.w3.org/TR/webrtc/)
- [QUIC Protocol](https://datatracker.ietf.org/doc/html/rfc9000)
- [STUN/TURN](https://datatracker.ietf.org/doc/html/rfc5389)
- [Signal Protocol](https://signal.org/docs/)
- [H.264 Encoding](https://www.itu.int/rec/T-REC-H.264)
