# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Rust CLI tool for Linux network speed testing and device information detection.

## Build & Run

```bash
cd /home/steven/projects/connection_test/netcheck

# Build
cargo build --release

# Run
./target/release/netcheck

# Development build (faster)
cargo build
./target/debug/netcheck
```

## Architecture

Single binary application (~160 lines) with:
- **Async runtime**: tokio for network operations
- **HTTP client**: reqwest with rustls for TLS
- **Terminal UI**: indicatif for progress bars, ANSI escape codes for colors

### Key Modules

- `get_network_devices()` - Reads `/sys/class/net` for网卡 info (name, MAC, status, speed)
- `get_cpu_model()` - Parses `/proc/cpuinfo` for CPU model
- `test_download_speed()` - Async download from Cloudflare speed test endpoint
- `test_latency()` - Runs `ping` command to 1.1.1.1

## Dependencies

Key crates in `Cargo.toml`:
- `tokio` - Async runtime
- `reqwest` - HTTP client with `rustls-tls` feature
- `futures` - StreamExt for async streaming
- `indicatif` - Progress bars
- `sysinfo` - System information (hostname)

## Testing

This project doesn't have dedicated tests. Run the binary to validate functionality.
