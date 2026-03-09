use std::io::Write;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use futures::stream::StreamExt;
use serde::Deserialize;

const CONNECTIONS: usize = 8;
const WARMUP_SECS: u64 = 2;
const MEASURE_SECS: u64 = 10;
const TICK_MS: u64 = 200;
const UPLOAD_CHUNK: usize = 4_000_000; // 4MB per POST

// Speed test servers by region
#[derive(Debug, Clone)]
pub struct SpeedServer {
    pub name: &'static str,
    download_url: &'static str,
    country: &'static str,
    pub city: &'static str,
}

pub const SPEED_SERVERS: &[SpeedServer] = &[
    SpeedServer {
        name: "Cloudflare Global",
        download_url: "https://speed.cloudflare.com/__down?bytes=25000000",
        country: "Any",
        city: "Global",
    },
    SpeedServer {
        name: "Cloudflare Hong Kong",
        download_url: "https://speed.cloudflare.com/__down?bytes=25000000",
        country: "HK",
        city: "Hong Kong",
    },
    SpeedServer {
        name: "China Telecom Shanghai",
        download_url: "https://speedtest.shanghai.ctyun.cn/10MB.dat",
        country: "CN",
        city: "Shanghai",
    },
    SpeedServer {
        name: "China Telecom Guangzhou",
        download_url: "https://speedtest.gd.ctyun.cn/10MB.dat",
        country: "CN",
        city: "Guangzhou",
    },
    SpeedServer {
        name: "China Unicom Beijing",
        download_url: "https://speedtest.bjunicom.com/10MB.bin",
        country: "CN",
        city: "Beijing",
    },
    SpeedServer {
        name: "HiNet Taiwan",
        download_url: "https://speedtest.hinet.net/10MB",
        country: "TW",
        city: "Taipei",
    },
    SpeedServer {
        name: "NTT Japan",
        download_url: "https://speedtest.ntt.com/10MB",
        country: "JP",
        city: "Tokyo",
    },
    SpeedServer {
        name: "M1 Singapore",
        download_url: "https://speedtest.m1.com.sg/10MB.bin",
        country: "SG",
        city: "Singapore",
    },
    SpeedServer {
        name: "Fast.com US",
        download_url: "https://speedtest.t-online.de/10MB.bin",
        country: "US",
        city: "Los Angeles",
    },
];

#[derive(Debug, Deserialize)]
pub struct IpInfo {
    #[allow(dead_code)]
    pub status: String,
    #[serde(rename = "countryCode", default)]
    pub country_code: String,
    #[serde(default)]
    pub country: String,
    #[serde(default)]
    pub city: String,
    #[serde(rename = "regionName", default)]
    pub region: String,
    #[serde(default)]
    pub isp: String,
}

pub async fn detect_location(client: &reqwest::Client) -> Option<IpInfo> {
    let url = "http://ip-api.com/json/?fields=status,country,countryCode,regionName,city,isp";

    let result = client
        .get(url)
        .timeout(Duration::from_secs(10))
        .send()
        .await;

    match result {
        Ok(resp) => match resp.json::<IpInfo>().await {
            Ok(info) => {
                if info.country.is_empty() {
                    None
                } else {
                    Some(info)
                }
            }
            Err(e) => {
                eprintln!("    Parse error: {}", e);
                None
            }
        },
        Err(e) => {
            eprintln!("    Request error: {}", e);
            None
        }
    }
}

pub fn select_server(ip_info: &IpInfo) -> &'static SpeedServer {
    for server in SPEED_SERVERS {
        if server.country == ip_info.country_code {
            return server;
        }
    }
    &SPEED_SERVERS[0]
}

/// HTTP-based latency test: measure round-trip time of HTTP HEAD requests
pub async fn test_latency(client: &reqwest::Client) -> Result<(f64, f64), String> {
    let url = "https://speed.cloudflare.com/__down?bytes=0";
    let mut times = Vec::new();

    for _ in 0..5 {
        let start = Instant::now();
        let result = client
            .head(url)
            .timeout(Duration::from_secs(5))
            .send()
            .await;
        let rtt = start.elapsed().as_secs_f64() * 1000.0;

        match result {
            Ok(_) => times.push(rtt),
            Err(e) => {
                if times.is_empty() {
                    return Err(format!("Network unreachable: {}", e));
                }
                // Partial success — use what we have
                break;
            }
        }
    }

    if times.is_empty() {
        return Err("Network unreachable".to_string());
    }

    // Drop the first measurement (TCP+TLS handshake overhead)
    if times.len() > 1 {
        times.remove(0);
    }

    let avg = times.iter().sum::<f64>() / times.len() as f64;

    // Jitter: average deviation from mean
    let jitter = if times.len() > 1 {
        times.iter().map(|t| (t - avg).abs()).sum::<f64>() / times.len() as f64
    } else {
        0.0
    };

    Ok((avg, jitter))
}

/// Parallel timed download with warmup phase
pub async fn test_download_speed(
    client: &reqwest::Client,
    server: &SpeedServer,
) -> Result<f64, String> {
    let url = server.download_url.to_string();
    let total_bytes = Arc::new(AtomicU64::new(0));
    let stop = Arc::new(AtomicBool::new(false));

    // Spawn parallel download workers — always count bytes
    let mut handles = Vec::new();
    for _ in 0..CONNECTIONS {
        let client = client.clone();
        let url = url.clone();
        let total_bytes = Arc::clone(&total_bytes);
        let stop = Arc::clone(&stop);

        handles.push(tokio::spawn(async move {
            while !stop.load(Ordering::Relaxed) {
                let response = match client.get(&url).send().await {
                    Ok(r) => r,
                    Err(_) => break,
                };
                let mut stream = response.bytes_stream();
                while let Some(chunk_result) = stream.next().await {
                    if stop.load(Ordering::Relaxed) {
                        break;
                    }
                    match chunk_result {
                        Ok(data) => {
                            total_bytes.fetch_add(data.len() as u64, Ordering::Relaxed);
                        }
                        Err(_) => break,
                    }
                }
            }
        }));
    }

    // Warmup with spinner
    let spinner = indicatif::ProgressBar::new_spinner();
    spinner.set_style(
        indicatif::ProgressStyle::default_spinner()
            .template("    {spinner:.yellow} Warming up...")
            .unwrap(),
    );
    spinner.enable_steady_tick(Duration::from_millis(80));
    tokio::time::sleep(Duration::from_secs(WARMUP_SECS)).await;
    total_bytes.store(0, Ordering::Relaxed);
    spinner.finish_and_clear();

    // Measure with fast refresh
    println!("    Measuring download...");
    let pb = indicatif::ProgressBar::new(MEASURE_SECS * 1000 / TICK_MS);
    pb.set_style(
        indicatif::ProgressStyle::default_bar()
            .template("    {bar:40.green} {elapsed_precise} {msg}")
            .unwrap()
            .progress_chars("━━╺"),
    );

    let measure_start = Instant::now();
    let total_ticks = MEASURE_SECS * 1000 / TICK_MS;
    for tick in 1..=total_ticks {
        tokio::time::sleep(Duration::from_millis(TICK_MS)).await;
        let elapsed = measure_start.elapsed().as_secs_f64();
        let bytes = total_bytes.load(Ordering::Relaxed);
        let speed_mbps = (bytes as f64 * 8.0) / elapsed / 1_000_000.0;
        pb.set_position(tick);
        pb.set_message(format!("\x1b[1;33m{:.1} Mbps\x1b[0m", speed_mbps));
    }

    stop.store(true, Ordering::Relaxed);
    for h in handles {
        let _ = h.await;
    }
    pb.finish_and_clear();

    let elapsed = measure_start.elapsed().as_secs_f64();
    let bytes = total_bytes.load(Ordering::Relaxed);

    if elapsed > 0.0 && bytes > 0 {
        Ok((bytes as f64 * 8.0) / elapsed / 1_000_000.0)
    } else {
        Err("Download failed: no data received".to_string())
    }
}

/// Parallel timed upload with warmup phase
pub async fn test_upload_speed(client: &reqwest::Client) -> Result<f64, String> {
    let total_bytes = Arc::new(AtomicU64::new(0));
    let stop = Arc::new(AtomicBool::new(false));

    let upload_data: Arc<Vec<u8>> = Arc::new(vec![0u8; UPLOAD_CHUNK]);

    // Spawn parallel upload workers — always count bytes
    let mut handles = Vec::new();
    for _ in 0..CONNECTIONS {
        let client = client.clone();
        let total_bytes = Arc::clone(&total_bytes);
        let stop = Arc::clone(&stop);
        let data = Arc::clone(&upload_data);

        handles.push(tokio::spawn(async move {
            while !stop.load(Ordering::Relaxed) {
                let chunk = data.as_ref().clone();
                let chunk_len = chunk.len() as u64;
                let result = client
                    .post("https://speed.cloudflare.com/__up")
                    .body(chunk)
                    .send()
                    .await;
                match result {
                    Ok(_) => {
                        total_bytes.fetch_add(chunk_len, Ordering::Relaxed);
                    }
                    Err(_) => break,
                }
            }
        }));
    }

    // Warmup with spinner
    let spinner = indicatif::ProgressBar::new_spinner();
    spinner.set_style(
        indicatif::ProgressStyle::default_spinner()
            .template("    {spinner:.yellow} Warming up...")
            .unwrap(),
    );
    spinner.enable_steady_tick(Duration::from_millis(80));
    tokio::time::sleep(Duration::from_secs(WARMUP_SECS)).await;
    total_bytes.store(0, Ordering::Relaxed);
    spinner.finish_and_clear();

    // Measure with fast refresh
    println!("    Measuring upload...");
    let pb = indicatif::ProgressBar::new(MEASURE_SECS * 1000 / TICK_MS);
    pb.set_style(
        indicatif::ProgressStyle::default_bar()
            .template("    {bar:40.cyan} {elapsed_precise} {msg}")
            .unwrap()
            .progress_chars("━━╺"),
    );

    let measure_start = Instant::now();
    let total_ticks = MEASURE_SECS * 1000 / TICK_MS;
    for tick in 1..=total_ticks {
        tokio::time::sleep(Duration::from_millis(TICK_MS)).await;
        let elapsed = measure_start.elapsed().as_secs_f64();
        let bytes = total_bytes.load(Ordering::Relaxed);
        let speed_mbps = (bytes as f64 * 8.0) / elapsed / 1_000_000.0;
        pb.set_position(tick);
        pb.set_message(format!("\x1b[1;33m{:.1} Mbps\x1b[0m", speed_mbps));
    }

    stop.store(true, Ordering::Relaxed);
    for h in handles {
        let _ = h.await;
    }
    pb.finish_and_clear();

    let elapsed = measure_start.elapsed().as_secs_f64();
    let bytes = total_bytes.load(Ordering::Relaxed);

    if elapsed > 0.0 && bytes > 0 {
        Ok((bytes as f64 * 8.0) / elapsed / 1_000_000.0)
    } else {
        Err("Upload failed: no data sent".to_string())
    }
}

pub async fn run_speed_test(client: &reqwest::Client) {
    crate::utils::print_section("Speed Test");

    // Detect location
    print!("    Detecting location... ");
    std::io::stdout().flush().ok();

    let location = detect_location(client).await;
    let server = match &location {
        Some(info) => {
            let srv = select_server(info);
            println!(
                "\x1b[32m{}, {} ({})\x1b[0m",
                info.city, info.region, info.country
            );
            println!("    ISP: \x1b[90m{}\x1b[0m", info.isp);
            println!(
                "    Server: \x1b[36m{}\x1b[0m ({}, {})",
                srv.name, srv.city, srv.country
            );
            srv
        }
        None => {
            println!("\x1b[33mUnknown\x1b[0m (using default)");
            println!("    Server: \x1b[36m{}\x1b[0m", SPEED_SERVERS[0].name);
            &SPEED_SERVERS[0]
        }
    };
    println!();

    // Latency (HTTP-based)
    print!("    Testing latency... ");
    std::io::stdout().flush().ok();
    match test_latency(client).await {
        Ok((avg, jitter)) => {
            println!(
                "\x1b[32m{:.1} ms\x1b[0m (jitter: {:.1} ms)",
                avg, jitter
            );
        }
        Err(e) => println!("\x1b[31mError: {}\x1b[0m", e),
    }
    println!();

    // Download
    println!("    \x1b[1m↓ Download\x1b[0m ({} connections)", CONNECTIONS);
    match test_download_speed(client, server).await {
        Ok(speed) => println!("    \x1b[32m↓ Download: {:.1} Mbps\x1b[0m", speed),
        Err(e) => {
            if !server.name.contains("Cloudflare") {
                println!(
                    "    \x1b[33mServer failed ({}), trying Cloudflare...\x1b[0m",
                    e
                );
                match test_download_speed(client, &SPEED_SERVERS[0]).await {
                    Ok(speed) => println!("    \x1b[32m↓ Download: {:.1} Mbps\x1b[0m", speed),
                    Err(e2) => println!("    \x1b[31m↓ Download: Error: {}\x1b[0m", e2),
                }
            } else {
                println!("    \x1b[31m↓ Download: Error: {}\x1b[0m", e);
            }
        }
    }
    println!();

    // Upload
    println!("    \x1b[1m↑ Upload\x1b[0m ({} connections)", CONNECTIONS);
    match test_upload_speed(client).await {
        Ok(speed) => println!("    \x1b[32m↑ Upload: {:.1} Mbps\x1b[0m", speed),
        Err(e) => println!("    \x1b[31m↑ Upload: Error: {}\x1b[0m", e),
    }
    println!();
}
