pub fn print_header() {
    println!("\n  \x1b[36m╔════════════════════════════════════════╗\x1b[0m");
    println!("  \x1b[36m║\x1b[0m        \x1b[1;37mNetCheck - Network Tool\x1b[0m         \x1b[36m║\x1b[0m");
    println!("  \x1b[36m╚════════════════════════════════════════╝\x1b[0m\n");
}

pub fn print_section(title: &str) {
    println!("  \x1b[33m▸ {}\x1b[0m", title);
}

pub fn print_footer() {
    println!("  \x1b[90m────────────────────────────────────\x1b[0m");
    println!("  \x1b[90mDone! Press Enter to exit...\x1b[0m");
    let _ = std::io::stdin().read_line(&mut String::new());
}
