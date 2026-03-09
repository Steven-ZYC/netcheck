mod cli;
mod network;
mod speedtest;
mod system;
mod utils;

use clap::Parser;
use cli::{Cli, Commands};

fn build_client() -> reqwest::Client {
    reqwest::Client::builder()
        .user_agent("netcheck/0.1")
        .build()
        .unwrap_or_else(|e| {
            eprintln!("  \x1b[31mFailed to create HTTP client: {}\x1b[0m", e);
            std::process::exit(1);
        })
}

async fn run_full() {
    utils::print_header();

    let info = system::collect_system_info();
    system::print_system_info(&info);

    let devices = network::get_network_devices();
    network::print_network_devices(&devices);

    let client = build_client();
    speedtest::run_speed_test(&client).await;

    utils::print_footer();
}

async fn run_info() {
    utils::print_header();
    let info = system::collect_system_info();
    system::print_system_info(&info);
}

async fn run_net() {
    utils::print_header();
    let devices = network::get_network_devices();
    network::print_network_devices(&devices);
}

async fn run_speed() {
    utils::print_header();
    let client = build_client();
    speedtest::run_speed_test(&client).await;
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        None => run_full().await,
        Some(Commands::Info) => run_info().await,
        Some(Commands::Net) => run_net().await,
        Some(Commands::Speed) => run_speed().await,
    }
}
