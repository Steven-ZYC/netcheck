# NetCheck

[简体中文](README_zh_CN.md) | [繁體中文](README_zh_HK.md)

A high-performance Linux CLI tool for network speed testing and device information detection, written in Rust.

## Features

- **System Info** — Hostname, OS, CPU, Kernel, Architecture
- **Network Devices** — Interface name, driver, MAC address, link state, duplex
- **Speed Test** — Latency (HTTP-based with jitter), multi-connection download & upload via Cloudflare
- **DNS Test** — Raw UDP queries to 6 public resolvers, ranked by latency
- **Route Trace** — Tracepath with colorized output, no root required

## Installation Guide

### Option 1: Install a native package

- Debian / Ubuntu: download the `.deb` asset from the latest release and install it with:

```bash
sudo apt install ./netcheck_<version>_amd64.deb
```

- Fedora / RHEL / Rocky / AlmaLinux / openSUSE: download the `.rpm` asset from the latest release and install it with:

```bash
sudo dnf install ./netcheck-<version>-1.x86_64.rpm
```

These native packages declare the runtime dependencies needed by the `dns` and `trace` commands, so package managers can install them automatically.

### Option 2: Use the portable binary archive

Download the release archive that matches your CPU architecture:

- `netcheck-v<version>-linux-x86_64.tar.gz`
- `netcheck-v<version>-linux-aarch64.tar.gz`

Then extract it and move the binary into your `PATH`:

```bash
tar -xzf netcheck-v<version>-linux-x86_64.tar.gz
sudo install -m 755 netcheck-v<version>-linux-x86_64/netcheck /usr/local/bin/netcheck
```

If you install from the raw binary archive, you must install the runtime dependencies yourself:

- Debian / Ubuntu:

```bash
sudo apt install dnsutils iputils-tracepath traceroute
```

- Fedora / RHEL / Rocky / AlmaLinux:

```bash
sudo dnf install bind-utils iputils traceroute
```

- openSUSE:

```bash
sudo zypper install bind-utils iputils traceroute
```

`netcheck info`, `netcheck net`, and `netcheck speed` work with the bundled binary alone. `netcheck dns` needs `dig`, and `netcheck trace` needs `tracepath` or `traceroute`.

### Option 3: Build from source

```bash
cargo build --release
```

The binary is at `target/release/netcheck`.

## CI/CD

GitHub Actions automates the project lifecycle:

- `CI`: runs `cargo fmt --check`, `cargo clippy -D warnings`, and `cargo test --locked` on pushes to `master` and pull requests
- `Release`: on `v*` tags, builds Linux release archives for `x86_64` and `aarch64`, builds native `.deb` and `.rpm` packages for `x86_64`, creates a GitHub Release, and uploads all artifacts plus SHA-256 checksum files

Create a release with:

```bash
git tag v0.1.0
git push origin v0.1.0
```

NetCheck is currently Linux-focused and depends on Linux system interfaces such as `/proc`, `/sys`, `tracepath`, and `dig`.

## Usage

```bash
# Full check (system + network + speed test)
netcheck

# Individual modules
netcheck info     # System information only
netcheck net      # Network interfaces only
netcheck speed    # Speed test only
netcheck dns      # DNS resolver latency test
netcheck trace    # Route trace to Cloudflare
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
├── dns.rs         # DNS resolver latency testing
├── trace.rs       # Route tracing (tracepath/traceroute)
└── utils.rs       # Display helpers
```

### Key Design

- **8 parallel connections** for download and upload, 10-second timed measurement with 2-second warmup
- **HTTP-based latency** test with jitter calculation (no dependency on `ping`)
- **Automatic server selection** based on IP geolocation
- **Graceful error handling** with server fallback
- **DNS latency test** with raw UDP packets to 6 resolvers (Cloudflare, Google, Alibaba, OpenDNS, Quad9, system), auto-ranked
- **Route trace** via `tracepath` (no root needed), real-time streaming output with colorized hops

## Dependencies

- [tokio](https://crates.io/crates/tokio) — Async runtime
- [reqwest](https://crates.io/crates/reqwest) — HTTP client (rustls)
- [clap](https://crates.io/crates/clap) — CLI argument parsing
- [indicatif](https://crates.io/crates/indicatif) — Progress bars
- [sysinfo](https://crates.io/crates/sysinfo) — Hostname detection

## License

[MIT](LICENSE) - Steven Zhang Yancheng
