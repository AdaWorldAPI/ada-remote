# Building Ada Remote from Source

This guide provides detailed instructions for building Ada Remote on different platforms.

## Table of Contents

- [Prerequisites](#prerequisites)
- [Linux](#linux)
- [macOS](#macos)
- [Windows](#windows)
- [Building the Relay Server](#building-the-relay-server)
- [Building the Desktop App](#building-the-desktop-app)
- [Cross-Compilation](#cross-compilation)
- [Troubleshooting](#troubleshooting)

## Prerequisites

### All Platforms

- **Rust** 1.70 or later ([Install](https://rustup.rs/))
- **Node.js** 18 or later ([Install](https://nodejs.org/))
- **Git** for cloning the repository

### Clone the Repository

```bash
git clone https://github.com/AdaWorldAPI/ada-remote.git
cd ada-remote
```

## Linux

### Ubuntu/Debian

```bash
# Install system dependencies
sudo apt update
sudo apt install -y \
    libx11-dev \
    libxrandr-dev \
    libxtest-dev \
    libwebkit2gtk-4.0-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev \
    pkg-config \
    libssl-dev

# Build workspace
cargo build --release

# Build desktop app
cd desktop
npm install
npm run tauri build
```

The built application will be in `desktop/src-tauri/target/release/bundle/`.

### Fedora/RHEL

```bash
# Install system dependencies
sudo dnf install -y \
    libX11-devel \
    libXrandr-devel \
    libXtst-devel \
    webkit2gtk4.0-devel \
    gtk3-devel \
    libappindicator-gtk3-devel \
    librsvg2-devel \
    openssl-devel

# Build workspace
cargo build --release

# Build desktop app
cd desktop
npm install
npm run tauri build
```

### Arch Linux

```bash
# Install system dependencies
sudo pacman -S \
    libx11 \
    libxrandr \
    libxtst \
    webkit2gtk \
    gtk3 \
    libappindicator-gtk3 \
    librsvg \
    openssl

# Build workspace
cargo build --release

# Build desktop app
cd desktop
npm install
npm run tauri build
```

## macOS

### Install Xcode Command Line Tools

```bash
xcode-select --install
```

### Install Homebrew (if not already installed)

```bash
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
```

### Build

```bash
# No additional system dependencies needed for macOS

# Build workspace
cargo build --release

# Build desktop app
cd desktop
npm install
npm run tauri build
```

The built application will be in `desktop/src-tauri/target/release/bundle/`.

### Creating a DMG

```bash
cd desktop
npm run tauri build -- --bundles dmg
```

## Windows

### Install Visual Studio Build Tools

Download and install [Visual Studio Build Tools](https://visualstudio.microsoft.com/downloads/) with:
- C++ build tools
- Windows 10/11 SDK

### Install WebView2

Download and install [Microsoft Edge WebView2 Runtime](https://developer.microsoft.com/en-us/microsoft-edge/webview2/).

### Build

```powershell
# Build workspace
cargo build --release

# Build desktop app
cd desktop
npm install
npm run tauri build
```

The built application will be in `desktop\src-tauri\target\release\bundle\`.

### Creating an Installer

```powershell
cd desktop
npm run tauri build -- --bundles msi
```

## Building the Relay Server

The relay server is a standalone binary with minimal dependencies.

### Linux/macOS

```bash
cargo build --release -p ada-remote-relay-server

# Binary will be at:
# target/release/relay-server
```

### Windows

```powershell
cargo build --release -p ada-remote-relay-server

# Binary will be at:
# target\release\relay-server.exe
```

### Docker

```bash
# Build Docker image
docker build -t ada-remote-relay .

# Run
docker run -p 8080:8080 ada-remote-relay
```

### Cross-compile for Linux on macOS/Windows

```bash
# Install cross-compilation target
rustup target add x86_64-unknown-linux-gnu

# Install cross
cargo install cross

# Build for Linux
cross build --release --target x86_64-unknown-linux-gnu -p ada-remote-relay-server
```

## Building the Desktop App

### Development Build

```bash
cd desktop
npm install
npm run tauri dev
```

This starts a development server with hot-reload.

### Production Build

```bash
cd desktop
npm install
npm run tauri build
```

### Platform-Specific Bundles

```bash
# Linux: AppImage, DEB, RPM
npm run tauri build -- --bundles appimage,deb

# macOS: DMG, APP
npm run tauri build -- --bundles dmg,app

# Windows: MSI, NSIS
npm run tauri build -- --bundles msi,nsis
```

## Cross-Compilation

### Linux to Windows

```bash
# Install mingw-w64
sudo apt install -y mingw-w64

# Add target
rustup target add x86_64-pc-windows-gnu

# Build
cargo build --release --target x86_64-pc-windows-gnu
```

### macOS Universal Binary (Apple Silicon + Intel)

```bash
# Add targets
rustup target add aarch64-apple-darwin
rustup target add x86_64-apple-darwin

# Build desktop app
cd desktop
npm run tauri build -- --target universal-apple-darwin
```

## Optimizing Build Size

### Enable LTO and Strip Symbols

Already configured in `Cargo.toml`:

```toml
[profile.release]
lto = true
codegen-units = 1
opt-level = 3
strip = true
```

### Additional Size Reduction

```bash
# Install upx (binary compressor)
# Linux/macOS
sudo apt install upx  # or brew install upx

# Compress binary
upx --best --lzma target/release/relay-server
```

## Build Caching

### Using sccache

```bash
# Install sccache
cargo install sccache

# Configure Cargo to use it
export RUSTC_WRAPPER=sccache

# Build as normal
cargo build --release
```

### GitHub Actions Cache

The CI configuration already includes caching for:
- Cargo registry
- Cargo index
- Build artifacts

## Troubleshooting

### "linker 'cc' not found"

**Linux**: Install build-essential:
```bash
sudo apt install build-essential
```

**macOS**: Install Xcode Command Line Tools:
```bash
xcode-select --install
```

### "could not find system library 'X11'"

Install the missing system dependency. See platform-specific sections above.

### "error: toolchain 'stable-x86_64-unknown-linux-gnu' does not support components"

Update Rust:
```bash
rustup update stable
```

### "npm ERR! code ELIFECYCLE"

Clean and rebuild:
```bash
cd desktop
rm -rf node_modules package-lock.json
npm install
```

### WebView2 Errors (Windows)

Install [Microsoft Edge WebView2 Runtime](https://developer.microsoft.com/en-us/microsoft-edge/webview2/).

### Out of Memory During Build

Reduce parallel jobs:
```bash
cargo build --release -j 2
```

### "Package 'webkit2gtk-4.0' not found" (Linux)

Install webkit2gtk:
```bash
# Ubuntu/Debian
sudo apt install libwebkit2gtk-4.0-dev

# Fedora
sudo dnf install webkit2gtk4.0-devel

# Arch
sudo pacman -S webkit2gtk
```

## Build Times

Typical build times on modern hardware:

- **Relay Server**: 2-5 minutes (clean build)
- **Desktop App**: 5-10 minutes (clean build)
- **Full Workspace**: 10-15 minutes (clean build)

Incremental builds are much faster (10-30 seconds).

## Verifying the Build

### Run Tests

```bash
cargo test --workspace --all-features
```

### Check Binary Size

```bash
ls -lh target/release/relay-server
ls -lh desktop/src-tauri/target/release/bundle/
```

### Test the Relay Server

```bash
./target/release/relay-server --bind 127.0.0.1:8080
```

### Test the Desktop App

```bash
cd desktop/src-tauri/target/release/
./ada-remote  # or ada-remote.exe on Windows
```

## Getting Help

- **GitHub Discussions**: Ask questions
- **GitHub Issues**: Report build problems
- **Documentation**: Check `/docs` folder

---

Happy building! 🔨
