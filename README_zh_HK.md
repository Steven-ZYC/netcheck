# NetCheck

[English](README.md) | [简体中文](README_zh_CN.md)

高性能 Linux 網絡測速與裝置資訊檢測 CLI 工具，使用 Rust 編寫。

## 功能

- **系統資訊** — 主機名、作業系統、CPU、核心、架構
- **網卡資訊** — 介面名稱、驅動程式、MAC 地址、連結狀態、雙工模式
- **網速測試** — HTTP 延遲（含抖動）、多連接並行下載與上傳（Cloudflare）
- **DNS 測試** — 原始 UDP 查詢 6 個公共 DNS 解析器，按延遲排名
- **路由追蹤** — tracepath 彩色輸出，無需 root 權限

## Installation Guide

### 方式 1：安裝原生套件

- Debian / Ubuntu：下載最新 Release 中的 `.deb` 檔案後安裝：

```bash
sudo apt install ./netcheck_<version>_amd64.deb
```

- Fedora / RHEL / Rocky / AlmaLinux / openSUSE：下載最新 Release 中的 `.rpm` 檔案後安裝：

```bash
sudo dnf install ./netcheck-<version>-1.x86_64.rpm
```

這類原生套件會宣告 `dns` 和 `trace` 子命令需要的執行時依賴，套件管理器可以自動處理。

### 方式 2：直接使用二進位壓縮包

下載與 CPU 架構相符的 Release 壓縮包：

- `netcheck-v<version>-linux-x86_64.tar.gz`
- `netcheck-v<version>-linux-aarch64.tar.gz`

解壓後把二進位放進 `PATH`：

```bash
tar -xzf netcheck-v<version>-linux-x86_64.tar.gz
sudo install -m 755 netcheck-v<version>-linux-x86_64/netcheck /usr/local/bin/netcheck
```

如果你是直接下載二進位壓縮包，系統依賴需要自行安裝：

- Debian / Ubuntu：

```bash
sudo apt install dnsutils iputils-tracepath traceroute
```

- Fedora / RHEL / Rocky / AlmaLinux：

```bash
sudo dnf install bind-utils iputils traceroute
```

- openSUSE：

```bash
sudo zypper install bind-utils iputils traceroute
```

其中 `netcheck info`、`netcheck net`、`netcheck speed` 只需要程式本身即可執行；`netcheck dns` 需要 `dig`，`netcheck trace` 需要 `tracepath` 或 `traceroute`。

### 方式 3：從原始碼編譯

```bash
cargo build --release
```

編譯產物位於 `target/release/netcheck`。

## 使用

```bash
# 完整檢測（系統 + 網卡 + 測速）
netcheck

# 單獨模組
netcheck info     # 僅系統資訊
netcheck net      # 僅網卡資訊
netcheck speed    # 僅測速
netcheck dns      # DNS 解析器延遲測試
netcheck trace    # 路由追蹤到 Cloudflare
```

## 輸出範例

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

## 架構

```
src/
├── main.rs        # CLI 調度
├── cli.rs         # clap 參數解析
├── system.rs      # 系統資訊採集
├── network.rs     # 網卡檢測
├── speedtest.rs   # 測速（延遲、下載、上傳）
├── dns.rs         # DNS 解析器延遲測試
├── trace.rs       # 路由追蹤（tracepath/traceroute）
└── utils.rs       # 顯示輔助函式
```

### 設計要點

- 下載和上傳均使用 **8 個並行連接**，10 秒計時測量，2 秒預熱排除 ramp-up
- **HTTP 延遲測試**，含抖動計算，無需依賴 `ping` 指令
- 基於 IP 地理位置**自動選擇測速伺服器**
- **優雅的錯誤處理**，伺服器不可用時自動切換
- **DNS 延遲測試**，原始 UDP 封包查詢 6 個解析器（Cloudflare、Google、Alibaba、OpenDNS、Quad9、系統），自動排名
- **路由追蹤**，使用 `tracepath`（無需 root），即時串流輸出，彩色顯示跳數和延遲

## 依賴

- [tokio](https://crates.io/crates/tokio) — 非同步執行環境
- [reqwest](https://crates.io/crates/reqwest) — HTTP 用戶端（rustls）
- [clap](https://crates.io/crates/clap) — CLI 參數解析
- [indicatif](https://crates.io/crates/indicatif) — 進度條
- [sysinfo](https://crates.io/crates/sysinfo) — 主機名檢測

## 授權條款

[MIT](LICENSE) - Steven Zhang Yancheng
