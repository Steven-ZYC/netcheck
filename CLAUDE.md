# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Rust CLI tool for Linux network speed testing and device information detection. Modular architecture with clap subcommands.

## Build & Run

```bash
cd /home/steven/projects/connection_test/netcheck

cargo build --release
./target/release/netcheck          # Full check
./target/release/netcheck info     # System info only
./target/release/netcheck net      # Network devices only
./target/release/netcheck speed    # Speed test only
```

## Architecture

```
src/
├── main.rs        # CLI dispatch, build_client()
├── cli.rs         # clap derive: Cli struct, Commands enum (Info, Net, Speed)
├── system.rs      # SystemInfo struct, collect from /proc/cpuinfo, /etc/os-release, uname
├── network.rs     # NetworkDevice struct, reads /sys/class/net (driver, MAC, duplex)
├── speedtest.rs   # Latency (HTTP HEAD), download/upload (8 parallel connections, 10s timed)
└── utils.rs       # print_header(), print_section(), print_footer()
```

### Key Design Decisions

- **No global client timeout**: reqwest Client has no `.timeout()` — Cloudflare streams need unlimited time. Per-request timeouts are set where needed (latency: 5s, location: 10s).
- **User-Agent required**: Cloudflare `__down` endpoint returns 403 without User-Agent header. Set via `client.user_agent("netcheck/0.1")`.
- **Download URL size**: `bytes=25000000` (25MB) per request. Larger values (100MB+) get 403 from Cloudflare. Workers loop continuously so total throughput is unaffected.
- **Warmup mechanism**: Workers count bytes from start; counter is reset to 0 after 2s warmup. Simpler than per-chunk flag checking.
- **Progress refresh**: 200ms tick (5 Hz) for smooth display with minimal CPU overhead.

### Speed Test Flow

1. Detect location via ip-api.com → select nearest server
2. HTTP HEAD latency × 5 (drop first for TLS warmup) → avg + jitter
3. Download: 8 tokio::spawn workers streaming from `__down`, Arc<AtomicU64> byte counter, Arc<AtomicBool> stop signal, 2s warmup + 10s measure
4. Upload: 8 workers POST 4MB chunks to `__up`, same pattern

## Dependencies

- `tokio` — Async runtime (full features)
- `reqwest` — HTTP client (rustls-tls, stream, json; default-features=false)
- `futures` — StreamExt for async byte streaming
- `clap` — CLI argument parsing (derive feature)
- `indicatif` — Progress bars and spinners
- `sysinfo` — Hostname detection
- `serde` — JSON deserialization for IP geolocation

## Testing

No dedicated tests. Validate by running:
```bash
./target/release/netcheck info
./target/release/netcheck net
./target/release/netcheck speed
```
