use std::time::Duration;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;

const TRACE_TARGET: &str = "1.1.1.1";
const MAX_HOPS: &str = "30";

pub async fn run_trace() {
    crate::utils::print_section("Route Trace");
    println!("    Target: \x1b[36m{}\x1b[0m (Cloudflare)", TRACE_TARGET);
    println!();

    // Try tracepath first (no root needed), then traceroute
    if !try_tracepath().await && !try_traceroute().await {
        println!("    \x1b[31mError: neither tracepath nor traceroute found\x1b[0m");
        println!("    Install with: \x1b[90msudo apt install iputils-tracepath\x1b[0m");
    }
    println!();
}

async fn try_tracepath() -> bool {
    let mut child = match Command::new("tracepath")
        .args(["-m", MAX_HOPS, TRACE_TARGET])
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
    {
        Ok(c) => c,
        Err(_) => return false,
    };

    let stdout = child.stdout.take().unwrap();
    let mut reader = BufReader::new(stdout).lines();

    while let Ok(Ok(Some(line))) =
        tokio::time::timeout(Duration::from_secs(5), reader.next_line()).await
    {
        print_trace_line(&line);
    }

    let _ = child.kill().await;
    true
}

async fn try_traceroute() -> bool {
    let mut child = match Command::new("traceroute")
        .args(["-m", MAX_HOPS, "-w", "3", TRACE_TARGET])
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
    {
        Ok(c) => c,
        Err(_) => return false,
    };

    let stdout = child.stdout.take().unwrap();
    let mut reader = BufReader::new(stdout).lines();

    while let Ok(Ok(Some(line))) =
        tokio::time::timeout(Duration::from_secs(5), reader.next_line()).await
    {
        print_trace_line(&line);
    }

    let _ = child.kill().await;
    true
}

fn print_trace_line(line: &str) {
    let trimmed = line.trim();
    if trimmed.is_empty() {
        return;
    }

    // Colorize: hop number green, "no reply" yellow, ms values cyan
    if trimmed.contains("no reply") {
        println!("    \x1b[33m{}\x1b[0m", trimmed);
    } else if trimmed.contains("reached") || trimmed.contains("Resume") {
        println!("    \x1b[32m{}\x1b[0m", trimmed);
    } else {
        // Color the latency values
        let colored = colorize_ms(trimmed);
        println!("    {}", colored);
    }
}

fn colorize_ms(line: &str) -> String {
    let mut result = String::new();
    let mut chars = line.chars().peekable();

    while let Some(c) = chars.next() {
        // Look for patterns like "1.234ms" or "1.234 ms"
        if c.is_ascii_digit() || c == '.' {
            let mut num = String::new();
            num.push(c);
            while let Some(&next) = chars.peek() {
                if next.is_ascii_digit() || next == '.' {
                    num.push(chars.next().unwrap());
                } else {
                    break;
                }
            }
            // Check if followed by "ms"
            let rest: String = chars.clone().take(3).collect();
            if rest.starts_with("ms") {
                chars.next(); // m
                chars.next(); // s
                result.push_str(&format!("\x1b[36m{}ms\x1b[0m", num));
            } else if rest.starts_with(" ms") {
                chars.next(); // space
                chars.next(); // m
                chars.next(); // s
                result.push_str(&format!("\x1b[36m{} ms\x1b[0m", num));
            } else {
                result.push_str(&num);
            }
        } else {
            result.push(c);
        }
    }
    result
}
