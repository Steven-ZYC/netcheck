pub struct SystemInfo {
    pub hostname: String,
    pub os: String,
    pub cpu: String,
    pub kernel: String,
    pub arch: String,
}

pub fn collect_system_info() -> SystemInfo {
    SystemInfo {
        hostname: get_hostname(),
        os: get_os_info(),
        cpu: get_cpu_model(),
        kernel: get_kernel(),
        arch: get_architecture(),
    }
}

fn get_hostname() -> String {
    sysinfo::System::host_name().unwrap_or_else(|| "Unknown".to_string())
}

fn get_os_info() -> String {
    std::fs::read_to_string("/etc/os-release")
        .ok()
        .and_then(|content| {
            content
                .lines()
                .find(|l| l.starts_with("PRETTY_NAME="))
                .map(|l| {
                    l.trim_start_matches("PRETTY_NAME=")
                        .trim_matches('"')
                        .to_string()
                })
        })
        .unwrap_or_else(|| "Unknown".to_string())
}

fn get_cpu_model() -> String {
    std::fs::read_to_string("/proc/cpuinfo")
        .ok()
        .and_then(|s| {
            s.lines()
                .find(|l| l.starts_with("model name"))
                .map(|l| {
                    l.split(':')
                        .nth(1)
                        .map(|s| s.trim().to_string())
                        .unwrap_or_else(|| "Unknown".to_string())
                })
        })
        .unwrap_or_else(|| "Unknown".to_string())
}

fn get_kernel() -> String {
    std::process::Command::new("uname")
        .arg("-r")
        .output()
        .ok()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_else(|| "Unknown".to_string())
}

fn get_architecture() -> String {
    std::process::Command::new("uname")
        .arg("-m")
        .output()
        .ok()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_else(|| "Unknown".to_string())
}

pub fn print_system_info(info: &SystemInfo) {
    crate::utils::print_section("System");
    println!("    Hostname: {}", info.hostname);
    println!("    OS: {}", info.os);
    println!("    CPU: {}", info.cpu);
    println!("    Kernel: {}", info.kernel);
    println!("    Arch: {}", info.arch);
    println!();
}
