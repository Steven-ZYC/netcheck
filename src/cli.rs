use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "netcheck", about = "Network & system information tool")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Show system information only
    Info,
    /// Show network interface information only
    Net,
    /// Run speed test only
    Speed,
    /// Test DNS resolver latency
    Dns,
    /// Trace route to Cloudflare (1.1.1.1)
    Trace,
}
