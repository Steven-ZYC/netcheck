use std::path::PathBuf;

#[derive(Debug)]
pub struct NetworkDevice {
    pub name: String,
    pub driver: String,
    pub mac: String,
    pub is_up: bool,
    pub duplex: String,
}

pub fn get_network_devices() -> Vec<NetworkDevice> {
    let mut devices = Vec::new();
    let net_path = PathBuf::from("/sys/class/net");

    if let Ok(entries) = std::fs::read_dir(&net_path) {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            if name == "lo" {
                continue;
            }

            let path = entry.path();

            let mac = std::fs::read_to_string(path.join("address"))
                .map(|s| s.trim().to_string())
                .unwrap_or_else(|_| "N/A".to_string());

            let operstate = std::fs::read_to_string(path.join("operstate"))
                .map(|s| s.trim().to_string())
                .unwrap_or_default();
            let is_up = operstate == "up";

            let driver = get_interface_driver(&name);
            let duplex = get_interface_duplex(&name);

            devices.push(NetworkDevice {
                name,
                driver,
                mac,
                is_up,
                duplex,
            });
        }
    }
    devices
}

fn get_interface_driver(name: &str) -> String {
    let driver_link = format!("/sys/class/net/{}/device/driver", name);
    std::fs::read_link(&driver_link)
        .ok()
        .and_then(|p| p.file_name().map(|n| n.to_string_lossy().to_string()))
        .unwrap_or_else(|| "N/A".to_string())
}

fn get_interface_duplex(name: &str) -> String {
    std::fs::read_to_string(format!("/sys/class/net/{}/duplex", name))
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|_| "N/A".to_string())
}

pub fn print_network_devices(devices: &[NetworkDevice]) {
    crate::utils::print_section("Network Devices");
    for dev in devices {
        let status = if dev.is_up {
            "\x1b[32m●\x1b[0m UP"
        } else {
            "\x1b[31m○\x1b[0m DOWN"
        };
        println!("    {} [{}]", dev.name, status);
        println!("      Driver: {}", dev.driver);
        println!("      MAC: {}", dev.mac);
        println!("      Duplex: {}", dev.duplex);
    }
    println!();
}
