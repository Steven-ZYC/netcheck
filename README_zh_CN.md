# NetCheck

[English](README.md) | [繁體中文](README_zh_HK.md)

高性能 Linux 网络测速与设备信息检测 CLI 工具，使用 Rust 编写。

## 功能

- **系统信息** — 主机名、操作系统、CPU、内核、架构
- **网卡信息** — 接口名称、驱动、MAC 地址、链路状态、双工模式
- **网速测试** — HTTP 延迟（含抖动）、多连接并行下载与上传（Cloudflare）
- **DNS 测试** — 原始 UDP 查询 6 个公共 DNS 解析器，按延迟排名
- **路由追踪** — tracepath 彩色输出，无需 root 权限

## Installation Guide

### 方式 1：安装原生软件包

- Debian / Ubuntu：下载最新 Release 中的 `.deb` 文件，然后安装：

```bash
sudo apt install ./netcheck_<version>_amd64.deb
```

- Fedora / RHEL / Rocky / AlmaLinux / openSUSE：下载最新 Release 中的 `.rpm` 文件，然后安装：

```bash
sudo dnf install ./netcheck-<version>-1.x86_64.rpm
```

这类原生软件包会声明 `dns` 和 `trace` 子命令需要的运行时依赖，包管理器可以自动处理。

### 方式 2：直接使用二进制压缩包

下载与你的 CPU 架构匹配的 Release 压缩包：

- `netcheck-v<version>-linux-x86_64.tar.gz`
- `netcheck-v<version>-linux-aarch64.tar.gz`

解压后将二进制放到 `PATH` 中：

```bash
tar -xzf netcheck-v<version>-linux-x86_64.tar.gz
sudo install -m 755 netcheck-v<version>-linux-x86_64/netcheck /usr/local/bin/netcheck
```

如果你是直接下载二进制压缩包，系统依赖需要你自己安装：

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

其中 `netcheck info`、`netcheck net`、`netcheck speed` 只需要程序本身即可运行；`netcheck dns` 需要 `dig`，`netcheck trace` 需要 `tracepath` 或 `traceroute`。

### 方式 3：从源码编译

```bash
cargo build --release
```

编译产物位于 `target/release/netcheck`。

## 使用

```bash
# 完整检测（系统 + 网卡 + 测速）
netcheck

# 单独模块
netcheck info     # 仅系统信息
netcheck net      # 仅网卡信息
netcheck speed    # 仅测速
netcheck dns      # DNS 解析器延迟测试
netcheck trace    # 路由追踪到 Cloudflare
```

## 输出示例

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

## 架构

```
src/
├── main.rs        # CLI 调度
├── cli.rs         # clap 参数解析
├── system.rs      # 系统信息采集
├── network.rs     # 网卡检测
├── speedtest.rs   # 测速（延迟、下载、上传）
├── dns.rs         # DNS 解析器延迟测试
├── trace.rs       # 路由追踪（tracepath/traceroute）
└── utils.rs       # 显示辅助函数
```

### 设计要点

- 下载和上传均使用 **8 个并行连接**，10 秒计时测量，2 秒预热排除 ramp-up
- **HTTP 延迟测试**，含抖动计算，无需依赖 `ping` 命令
- 基于 IP 地理位置**自动选择测速服务器**
- **优雅的错误处理**，服务器不可用时自动切换
- **DNS 延迟测试**，原始 UDP 封包查询 6 个解析器（Cloudflare、Google、Alibaba、OpenDNS、Quad9、系统），自动排名
- **路由追踪**，使用 `tracepath`（无需 root），实时流式输出，彩色显示跳数和延迟

## 依赖

- [tokio](https://crates.io/crates/tokio) — 异步运行时
- [reqwest](https://crates.io/crates/reqwest) — HTTP 客户端（rustls）
- [clap](https://crates.io/crates/clap) — CLI 参数解析
- [indicatif](https://crates.io/crates/indicatif) — 进度条
- [sysinfo](https://crates.io/crates/sysinfo) — 主机名检测

## 许可证

[MIT](LICENSE) - Steven Zhang Yancheng
