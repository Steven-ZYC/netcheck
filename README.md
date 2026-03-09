# NetCheck

[简体中文](README_zh_CN.md) | [繁體中文](README_zh_HK.md)

A high-performance Linux CLI tool for network speed testing and device information detection, written in Rust.

## Features

- **System Info** — Hostname, OS, CPU, Kernel, Architecture
- **Network Devices** — Interface name, driver, MAC address, link state, duplex
- **Speed Test** — Latency (HTTP-based with jitter), multi-connection download & upload via Cloudflare

## Install

```bash
cargo build --release
```

The binary is at `target/release/netcheck`.

## Usage

```bash
# Full check (system + network + speed test)
netcheck

# Individual modules
netcheck info     # System information only
netcheck net      # Network interfaces only
netcheck speed    # Speed test only
```

## Example Output

```
╔════════════════════════════════════════╗
║        NetCheck - Network Tool         ║
╚════════════════════════════════════════╝

▸ System
  Hostname: myserver
  OS: Ubuntu 22.04.5 LTS
  CPU: Intel(R) Core(TM) i5-8365U CPU @ 1.60GHz
  Kernel: 6.8.0-100-generic
  Arch: x86_64

▸ Network Devices
  eth0 [● UP]
    Driver: e1000e
    MAC: 00:1A:2B:3C:4D:5E
    Duplex: full

▸ Speed Test
  Detecting location... Hong Kong (China)
  Testing latency... 43.0 ms (jitter: 1.0 ms)

  Testing download (8 connections, 2+10s)...
  ↓ Download: 29.8 Mbps

  Testing upload (8 connections, 2+10s)...
  ↑ Upload: 64.6 Mbps
```

## Architecture

```
src/
├── main.rs        # CLI dispatch
├── cli.rs         # clap argument parsing
├── system.rs      # System info collection
├── network.rs     # Network interface detection
├── speedtest.rs   # Speed test (latency, download, upload)
└── utils.rs       # Display helpers
```

### Key Design

- **8 parallel connections** for download and upload, 10-second timed measurement with 2-second warmup
- **HTTP-based latency** test with jitter calculation (no dependency on `ping`)
- **Automatic server selection** based on IP geolocation
- **Graceful error handling** with server fallback

## Dependencies

- [tokio](https://crates.io/crates/tokio) — Async runtime
- [reqwest](https://crates.io/crates/reqwest) — HTTP client (rustls)
- [clap](https://crates.io/crates/clap) — CLI argument parsing
- [indicatif](https://crates.io/crates/indicatif) — Progress bars
- [sysinfo](https://crates.io/crates/sysinfo) — Hostname detection

## License

[MIT](LICENSE) - Steven Zhang Yancheng
