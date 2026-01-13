# Contributing to Ada Remote

Thank you for your interest in contributing to Ada Remote! We welcome contributions from everyone.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Project Structure](#project-structure)
- [How to Contribute](#how-to-contribute)
- [Coding Guidelines](#coding-guidelines)
- [Testing](#testing)
- [Pull Request Process](#pull-request-process)
- [Areas Needing Help](#areas-needing-help)

## Code of Conduct

This project adheres to a code of conduct that we expect all contributors to follow:

- **Be respectful**: Treat everyone with respect and kindness
- **Be inclusive**: Welcome newcomers and be patient with questions
- **Be collaborative**: Work together and help each other
- **Be professional**: Keep discussions focused and constructive

## Getting Started

1. **Fork the repository** on GitHub
2. **Clone your fork**:
   ```bash
   git clone https://github.com/YOUR_USERNAME/ada-remote.git
   cd ada-remote
   ```
3. **Add upstream remote**:
   ```bash
   git remote add upstream https://github.com/AdaWorldAPI/ada-remote.git
   ```

## Development Setup

### Prerequisites

- **Rust** 1.70 or later ([Install](https://rustup.rs/))
- **Node.js** 18 or later ([Install](https://nodejs.org/))
- **Platform-specific dependencies**:
  - **Linux**: `libx11-dev`, `libxrandr-dev`, `libxtest-dev`, `libwebkit2gtk-4.0-dev`
  - **macOS**: Xcode Command Line Tools
  - **Windows**: Visual Studio Build Tools

### Building the Project

```bash
# Build all workspace crates
cargo build

# Run tests
cargo test

# Check code formatting
cargo fmt --check

# Run linter
cargo clippy
```

### Running the Relay Server

```bash
cd relay-server
cargo run -- --bind 127.0.0.1:8080
```

### Running the Desktop App

```bash
cd desktop
npm install
npm run tauri dev
```

## Project Structure

```
ada-remote/
├── crates/              # Core Rust libraries
│   ├── core/           # Protocol types and shared code
│   ├── capture/        # Screen capture implementations
│   ├── input/          # Input injection
│   ├── codec/          # Video encoding/decoding
│   ├── crypto/         # Encryption and security
│   └── network/        # WebRTC and networking
├── relay-server/       # Signaling/TURN server
├── desktop/            # Tauri desktop application
│   ├── src-tauri/     # Rust backend
│   └── src/           # Web frontend
└── docs/              # Documentation
```

## How to Contribute

### Reporting Bugs

1. **Check existing issues** to avoid duplicates
2. **Use the bug report template** when creating a new issue
3. **Include**:
   - Operating system and version
   - Rust version (`rustc --version`)
   - Steps to reproduce
   - Expected vs actual behavior
   - Error messages or logs

### Suggesting Features

1. **Check existing issues** and discussions
2. **Use the feature request template**
3. **Describe**:
   - The problem you're trying to solve
   - Your proposed solution
   - Alternative approaches considered
   - Any relevant examples or mockups

### Submitting Code

1. **Create a new branch**:
   ```bash
   git checkout -b feature/your-feature-name
   # or
   git checkout -b fix/your-bug-fix
   ```

2. **Make your changes** following the coding guidelines

3. **Write/update tests** for your changes

4. **Ensure all tests pass**:
   ```bash
   cargo test --workspace
   ```

5. **Format your code**:
   ```bash
   cargo fmt --all
   ```

6. **Check for linting issues**:
   ```bash
   cargo clippy --workspace -- -D warnings
   ```

7. **Commit your changes**:
   ```bash
   git commit -m "Brief description of changes"
   ```

   Use clear, descriptive commit messages:
   - `feat: Add screen capture support for Wayland`
   - `fix: Resolve memory leak in video encoder`
   - `docs: Update installation instructions`
   - `refactor: Simplify connection state machine`

8. **Push to your fork**:
   ```bash
   git push origin feature/your-feature-name
   ```

9. **Create a Pull Request** on GitHub

## Coding Guidelines

### Rust Code Style

- Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `cargo fmt` for formatting (enforced by CI)
- Use `cargo clippy` and fix all warnings
- Document public APIs with doc comments (`///`)
- Write idiomatic Rust code

### Error Handling

- Use `Result<T>` for operations that can fail
- Define custom error types using `thiserror`
- Provide meaningful error messages
- Use `?` operator for error propagation

### Security

- Never commit secrets or credentials
- Use the crypto primitives from `ada-remote-crypto`
- Follow secure coding practices
- Report security issues privately

### Performance

- Profile code before optimizing
- Use benchmarks to measure improvements
- Consider memory usage and allocations
- Document any performance-critical code

## Testing

### Unit Tests

```bash
# Run all tests
cargo test --workspace

# Run tests for a specific crate
cargo test -p ada-remote-core

# Run tests with output
cargo test -- --nocapture
```

### Integration Tests

Integration tests are located in the `tests/` directory of each crate.

### Test Coverage

We aim for high test coverage, especially for:
- Core protocol logic
- Cryptographic functions
- Network error handling
- Platform-specific code paths

## Pull Request Process

1. **Ensure CI passes**: All tests, linting, and formatting checks must pass
2. **Update documentation**: Include relevant doc changes
3. **Link related issues**: Reference any related issue numbers
4. **Request review**: Assign reviewers or wait for maintainers
5. **Address feedback**: Respond to review comments promptly
6. **Squash commits**: Keep history clean (maintainers can help with this)

### PR Checklist

- [ ] Code follows project style guidelines
- [ ] All tests pass locally
- [ ] Added tests for new functionality
- [ ] Updated relevant documentation
- [ ] No new compiler warnings
- [ ] Commit messages are clear and descriptive
- [ ] PR description explains what and why

## Areas Needing Help

### High Priority

- **Screen Capture Implementation**: Complete platform-specific implementations
  - Linux: X11 and Wayland/PipeWire support
  - Windows: DXGI Desktop Duplication API
  - macOS: ScreenCaptureKit

- **Video Encoding**: FFmpeg integration with hardware acceleration
  - H.264 encoding/decoding
  - VP9 fallback support
  - Adaptive bitrate

- **WebRTC Connection**: Complete peer connection establishment
  - ICE candidate handling
  - NAT traversal
  - Connection state management

### Medium Priority

- **Input Injection**: Platform-specific keyboard/mouse injection
- **File Transfer**: Chunked file transfer with resume
- **Multi-monitor**: Support for multiple displays
- **Performance Optimization**: Profiling and optimization

### Always Welcome

- **Documentation**: Tutorials, examples, API docs
- **Testing**: More comprehensive test coverage
- **Bug Fixes**: Any bug fixes, large or small
- **UI/UX**: Desktop app interface improvements
- **Accessibility**: Making Ada Remote accessible to everyone

## Questions?

- **GitHub Discussions**: For questions and general discussion
- **GitHub Issues**: For bug reports and feature requests
- **Documentation**: Check the `/docs` folder

---

Thank you for contributing to Ada Remote! 🚀
