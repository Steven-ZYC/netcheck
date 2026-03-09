---

# NetCheck — 高性能 Linux 网络与设备信息检测工具

## 1. 项目简介

**NetCheck** 是一个使用 **Rust** 编写的高性能 CLI 工具，用于快速检测 **Linux 系统的网络状态、设备信息和实际带宽性能**。

目标是提供一个类似：

```
neofetch + speedtest
```

的轻量级工具，帮助用户快速了解：

* 当前网络接口状态
* 实际网络带宽
* 网络延迟
* 硬件信息

典型应用场景：

* 服务器部署后快速网络检测
* Raspberry Pi / NAS 网络调试
* DevOps / 系统管理员日常检查
* Homelab 环境监控

---

# 2. 设计目标

### 主要目标

* **高性能**：Rust + async 并行测速
* **快速启动**：CLI 启动 < 100ms
* **单二进制部署**
* **无 root 权限即可运行**
* **简洁输出**

---

### 非目标 (Non-goals)

当前版本不实现：

* GUI
* 网络长期监控
* 云端数据上传
* ISP 质量排名

这些可以作为未来扩展。

---

# 3. 功能模块

项目分为三个核心模块：

```
NetCheck
 ├── System Info
 ├── Network Interface
 └── Speed Test
```

---

# 4. 功能规格

## 4.1 系统与设备信息

检测当前机器基本硬件信息。

信息来源：

```
/proc/cpuinfo
/sys/class/net/
/etc/os-release
```

输出示例：

```
System
------
OS: Ubuntu 22.04
CPU: Intel i7-1185G7
Kernel: 6.5.0
Architecture: x86_64
```

---

## 4.2 网络接口检测

检测当前系统的网络接口信息。

数据来源：

```
/sys/class/net/<interface>/
```

输出内容：

| 信息          | 示例                |
| ----------- | ----------------- |
| Interface   | eth0              |
| Driver      | e1000             |
| MAC Address | 00:1A:2B:3C:4D:5E |
| Link State  | UP                |
| Speed       | 1000 Mbps         |
| Duplex      | full              |

示例输出：

```
Network Interface
-----------------
Interface: eth0
Driver: e1000
MAC: 00:1A:2B:3C:4D:5E
Status: Connected
Speed: 1000 Mbps
```

---

## 4.3 网络测速

实现轻量级 **带宽测试系统**。

支持：

* 下载测速
* 上传测速
* Ping 延迟

---

### 下载测速

实现方法：

```
并行 HTTP 下载多个 chunk
统计总吞吐量
```

流程：

```
1. 获取测速服务器
2. 创建 N 个并行下载任务
3. 持续下载固定时间
4. 统计平均带宽
```

默认参数：

```
connections = 8
duration = 10 seconds
```

---

### 上传测速

实现：

```
向测速服务器上传随机数据
统计吞吐量
```

---

### 延迟测试

实现：

```
HTTP ping
或
ICMP ping
```

输出：

```
Latency: 18 ms
```

---

# 5. CLI 设计

命令结构：

```
netcheck [command]
```

子命令：

| 命令             | 功能   |
| -------------- | ---- |
| netcheck       | 完整检测 |
| netcheck info  | 系统信息 |
| netcheck net   | 网卡信息 |
| netcheck speed | 网速测试 |

示例：

```
netcheck
```

输出：

```
NetCheck v0.1

System
------
OS: Ubuntu 22.04
CPU: Intel i7

Network
-------
Interface: eth0
Speed: 1 Gbps

Speed Test
----------
Ping: 18 ms
Download: 512 Mbps
Upload: 48 Mbps
```

---

# 6. 终端 UI 设计

使用简单 CLI UI：

建议库：

* indicatif (progress bar)
* console

测速进度：

```
Download Test
[███████████████░░░░░░] 6.2s
Speed: 480 Mbps
```

最终结果：

```
Result
------
Ping: 18 ms
Download: 512 Mbps
Upload: 48 Mbps
```

---

# 7. 技术架构

## Runtime

```
tokio
```

---

## HTTP

```
reqwest (rustls)
```

---

## CLI

建议使用：

```
clap
```

---

## UI

建议使用：

```
indicatif
```

---

## 模块结构

推荐项目结构：

```
netcheck
 ├─ src
 │  ├─ main.rs
 │  ├─ cli.rs
 │  ├─ system.rs
 │  ├─ network.rs
 │  ├─ speedtest.rs
 │  └─ utils.rs
 │
 ├─ Cargo.toml
 └─ README.md
```

---

# 8. 性能要求

目标性能：

| 指标       | 目标      |
| -------- | ------- |
| CLI 启动时间 | < 100ms |
| 测速精度     | ±5%     |
| CPU 占用   | < 50%   |
| 内存使用     | < 50MB  |

---

# 9. 错误处理

必须处理：

| 场景       | 行为       |
| -------- | -------- |
| 无网络      | 显示错误     |
| 无权限      | fallback |
| 测速服务器不可用 | 自动切换     |

示例：

```
Error: Network unreachable
```

---

# 10. 验收标准

必须满足：

* [x] 正确检测网卡信息
* [x] 正确显示 MAC 地址
* [x] 下载测速正常
* [x] 上传测速正常
* [x] 延迟测试正常
* [x] CLI 输出整洁
* [x] 编译无 warning
* [x] 运行无 panic

---

# 11. 未来扩展

潜在扩展：

### JSON 输出

```
netcheck --json
```

用于：

* 自动化脚本
* DevOps

---

### Web UI

未来可以增加：

```
netcheck serve
```

提供：

```
localhost dashboard
```

---

### 长期监控

```
netcheck monitor
```

输出：

```
bandwidth graph
```

---

# 12. License

```
MIT License
```

---

