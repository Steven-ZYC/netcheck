# NetCheck

[English](README.md) | [简体中文](README_zh_CN.md)

高性能 Linux 網絡測速與裝置資訊檢測 CLI 工具，使用 Rust 編寫。

## 功能

- **系統資訊** — 主機名、作業系統、CPU、核心、架構
- **網卡資訊** — 介面名稱、驅動程式、MAC 地址、連結狀態、雙工模式
- **網速測試** — HTTP 延遲（含抖動）、多連接並行下載與上傳（Cloudflare）

## 安裝

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
└── utils.rs       # 顯示輔助函式
```

### 設計要點

- 下載和上傳均使用 **8 個並行連接**，10 秒計時測量，2 秒預熱排除 ramp-up
- **HTTP 延遲測試**，含抖動計算，無需依賴 `ping` 指令
- 基於 IP 地理位置**自動選擇測速伺服器**
- **優雅的錯誤處理**，伺服器不可用時自動切換

## 依賴

- [tokio](https://crates.io/crates/tokio) — 非同步執行環境
- [reqwest](https://crates.io/crates/reqwest) — HTTP 用戶端（rustls）
- [clap](https://crates.io/crates/clap) — CLI 參數解析
- [indicatif](https://crates.io/crates/indicatif) — 進度條
- [sysinfo](https://crates.io/crates/sysinfo) — 主機名檢測

## 授權條款

[MIT](LICENSE) - Steven Zhang Yancheng
