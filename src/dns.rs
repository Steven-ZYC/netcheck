use std::time::{Duration, Instant};
use tokio::net::UdpSocket;

const QUERY_DOMAIN: &str = "cloudflare.com";
const QUERIES_PER_SERVER: usize = 3;

struct DnsServer {
    name: &'static str,
    addr: &'static str,
}

const DNS_SERVERS: &[DnsServer] = &[
    DnsServer {
        name: "System",
        addr: "",
    },
    DnsServer {
        name: "Cloudflare",
        addr: "1.1.1.1",
    },
    DnsServer {
        name: "Google",
        addr: "8.8.8.8",
    },
    DnsServer {
        name: "Alibaba",
        addr: "223.5.5.5",
    },
    DnsServer {
        name: "OpenDNS",
        addr: "208.67.222.222",
    },
    DnsServer {
        name: "Quad9",
        addr: "9.9.9.9",
    },
];

/// Build a minimal DNS A-record query packet
fn build_dns_query(domain: &str) -> Vec<u8> {
    let mut packet = vec![
        0xAB, 0xCD, // ID
        0x01, 0x00, // Flags: standard query, recursion desired
        0x00, 0x01, // QDCOUNT: 1
        0x00, 0x00, // ANCOUNT
        0x00, 0x00, // NSCOUNT
        0x00, 0x00, // ARCOUNT
    ];

    // QNAME: encode domain labels
    for label in domain.split('.') {
        packet.push(label.len() as u8);
        packet.extend_from_slice(label.as_bytes());
    }
    packet.push(0); // root label

    packet.extend_from_slice(&[0x00, 0x01]); // QTYPE: A
    packet.extend_from_slice(&[0x00, 0x01]); // QCLASS: IN

    packet
}

/// Send a raw DNS query over UDP and measure RTT
async fn query_dns(addr: &str, query: &[u8]) -> Result<f64, String> {
    let socket = UdpSocket::bind("0.0.0.0:0")
        .await
        .map_err(|e| format!("Bind failed: {}", e))?;

    let target = format!("{}:53", addr);

    let start = Instant::now();
    socket
        .send_to(query, &target)
        .await
        .map_err(|e| format!("Send failed: {}", e))?;

    let mut buf = [0u8; 512];
    tokio::time::timeout(Duration::from_secs(3), socket.recv_from(&mut buf))
        .await
        .map_err(|_| "Timeout".to_string())?
        .map_err(|e| format!("Recv failed: {}", e))?;

    Ok(start.elapsed().as_secs_f64() * 1000.0)
}

/// Test system resolver using dig/nslookup fallback
async fn query_system_dns() -> Result<f64, String> {
    let start = Instant::now();
    let output = tokio::process::Command::new("dig")
        .args(["+short", "+time=3", "+tries=1", QUERY_DOMAIN])
        .output()
        .await
        .map_err(|_| "dig not found".to_string())?;

    if !output.status.success() {
        return Err("dig failed".to_string());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    if stdout.trim().is_empty() {
        return Err("No result".to_string());
    }

    Ok(start.elapsed().as_secs_f64() * 1000.0)
}

struct DnsResult {
    name: &'static str,
    addr: &'static str,
    avg: f64,
}

pub async fn run_dns_test() {
    crate::utils::print_section("DNS Latency Test");
    println!(
        "    Query: \x1b[90mA record for {}\x1b[0m ({}x per server)",
        QUERY_DOMAIN, QUERIES_PER_SERVER
    );
    println!();

    let query = build_dns_query(QUERY_DOMAIN);
    let mut results: Vec<DnsResult> = Vec::new();

    for server in DNS_SERVERS {
        print!("    Testing \x1b[36m{:<12}\x1b[0m", server.name);
        if !server.addr.is_empty() {
            print!(" ({:<15})", server.addr);
        } else {
            print!(" {:<17}", "(system resolver)");
        }
        print!("  ");

        let mut times = Vec::new();
        let mut last_err = String::new();

        for _ in 0..QUERIES_PER_SERVER {
            let result = if server.addr.is_empty() {
                query_system_dns().await
            } else {
                query_dns(server.addr, &query).await
            };

            match result {
                Ok(ms) => times.push(ms),
                Err(e) => last_err = e,
            }
        }

        if times.is_empty() {
            println!("\x1b[31m{}\x1b[0m", last_err);
        } else {
            let avg = times.iter().sum::<f64>() / times.len() as f64;
            let min = times.iter().cloned().fold(f64::INFINITY, f64::min);
            let max = times.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
            println!(
                "\x1b[32m{:>6.1} ms\x1b[0m  (min: {:.1}, max: {:.1})",
                avg, min, max
            );
            results.push(DnsResult {
                name: server.name,
                addr: server.addr,
                avg,
            });
        }
    }

    // Ranking
    results.sort_by(|a, b| a.avg.partial_cmp(&b.avg).unwrap());
    println!();
    println!("    \x1b[1mRanking:\x1b[0m");
    for (i, r) in results.iter().enumerate() {
        let medal = match i {
            0 => "\x1b[33m1.\x1b[0m",
            1 => "\x1b[37m2.\x1b[0m",
            2 => "\x1b[90m3.\x1b[0m",
            _ => "  ",
        };
        let addr_display = if r.addr.is_empty() {
            "system".to_string()
        } else {
            r.addr.to_string()
        };
        println!(
            "    {} {:<12} ({:<15})  \x1b[32m{:.1} ms\x1b[0m",
            medal, r.name, addr_display, r.avg
        );
    }
    println!();
}
