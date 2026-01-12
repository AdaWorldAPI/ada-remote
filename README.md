# Ada Remote

[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org)

> **Open-source remote desktop solution** - A modern, fast, and secure alternative to TeamViewer.

Ada Remote is a cross-platform remote desktop application built with Rust, WebRTC, and Tauri. It provides low-latency screen sharing, input control, and file transfer capabilities with end-to-end encryption.

## ✨ Features

### Phase 1 - MVP (In Progress)
- 🖥️ **Remote Desktop Viewing** with adaptive quality
- ⌨️ **Keyboard & Mouse Control** forwarding
- 🔐 **Session-based Authentication** (ID + password)
- 🌐 **NAT Traversal** using STUN/TURN
- 📋 **Clipboard Sync** (text)
- 💻 **Cross-platform**: Windows, macOS, Linux

### Phase 2 - Essential (Planned)
- 📁 File transfer with resume support
- 🖼️ Multi-monitor support
- 🤖 Unattended access (service mode)
- 📖 Address book / saved connections
- 💬 In-session chat

### Phase 3 - Advanced (Future)
- ⚡ Wake-on-LAN
- 🎥 Session recording
- 🔒 Two-factor authentication
- 📱 Mobile clients (iOS/Android)
- 🏠 Self-hosted relay servers

## 🏗️ Architecture

```
ada-remote/
├── crates/
│   ├── core/          # Core types & protocol
│   ├── capture/       # Screen capture (platform-specific)
│   ├── input/         # Input injection
│   ├── codec/         # Video encoding (H.264/VP9)
│   ├── crypto/        # E2E encryption
│   └── network/       # WebRTC & QUIC
├── relay-server/      # Signaling & TURN server
├── desktop/           # Tauri desktop app
│   ├── src-tauri/     # Rust backend
│   └── src/           # Web frontend
└── docs/
    └── PROTOCOL.md    # Protocol specification
```

## 🚀 Quick Start

### Prerequisites

- **Rust** 1.70+ ([Install](https://rustup.rs/))
- **Node.js** 18+ ([Install](https://nodejs.org/))
- **System Dependencies**:
  - **Linux**: `libx11-dev`, `libxrandr-dev`, `libxtest-dev`
  - **macOS**: Xcode Command Line Tools
  - **Windows**: Visual Studio Build Tools

### Build the Desktop App

```bash
# Clone the repository
git clone https://github.com/AdaWorldAPI/ada-remote.git
cd ada-remote

# Build all workspace crates
cargo build --release

# Build the desktop app
cd desktop
npm install
npm run tauri build
```

### Run the Relay Server

```bash
cd relay-server
cargo run --release -- --bind 0.0.0.0:8080
```

### Run the Desktop App (Development)

```bash
cd desktop
npm run tauri dev
```

## 🔧 Technical Stack

| Component | Technology |
|-----------|------------|
| **Core Engine** | Rust |
| **Desktop UI** | Tauri + Vite |
| **Video Codec** | H.264 / VP9 (FFmpeg) |
| **Networking** | WebRTC, QUIC |
| **Encryption** | X25519 + ChaCha20-Poly1305 |
| **Signaling** | WebSocket |

## 🔐 Security

- **End-to-End Encryption**: All sessions encrypted using X25519 key exchange and ChaCha20-Poly1305 AEAD
- **Password Hashing**: Argon2id for session passwords
- **No Telemetry**: Zero data collection, fully privacy-focused
- **Open Source**: Auditable code, no proprietary black boxes

## 📖 Documentation

- [Protocol Specification](docs/PROTOCOL.md) - Technical details of the Ada Remote protocol
- [Building from Source](docs/BUILDING.md) - Detailed build instructions *(coming soon)*
- [Contributing Guide](docs/CONTRIBUTING.md) - How to contribute *(coming soon)*

## 🛠️ Development Status

🚧 **Alpha Stage** - Core functionality is being implemented. Not ready for production use.

### Current Progress

- [x] Project scaffolding
- [x] Core protocol types
- [x] Basic Tauri UI
- [x] Relay server structure
- [ ] Screen capture implementation
- [ ] Video encoding/decoding
- [ ] WebRTC connection establishment
- [ ] Input injection
- [ ] E2E encryption integration

## 🤝 Contributing

Contributions are welcome! Ada Remote is in early development and we'd love your help.

### Areas Needing Help

- Screen capture optimization
- Hardware-accelerated encoding
- Mobile client development
- UI/UX improvements
- Documentation
- Testing on various platforms

Please see our [Contributing Guide](docs/CONTRIBUTING.md) *(coming soon)* for details.

## 📜 License

Ada Remote is dual-licensed under:

- **MIT License** ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
- **Apache License 2.0** ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)

You may choose either license for your use.

## 🙏 Acknowledgments

Inspired by:
- [RustDesk](https://github.com/rustdesk/rustdesk) - Rust-based remote desktop
- [AnyDesk](https://anydesk.com/) - Performance benchmarks
- [Sunshine/Moonlight](https://github.com/LizardByte/Sunshine) - Game streaming approach

## 📬 Contact

- **Issues**: [GitHub Issues](https://github.com/AdaWorldAPI/ada-remote/issues)
- **Discussions**: [GitHub Discussions](https://github.com/AdaWorldAPI/ada-remote/discussions)

---

**Built with ❤️ by the Ada Remote community**
