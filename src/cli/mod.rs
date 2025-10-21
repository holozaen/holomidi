pub fn print_usage(program: &str) {
    println!("Usage: {} <command> [options]", program);
    println!("Commands:");
    println!("  list                     List all available MIDI ports");
    println!("  sysex --port <n>         Send SYSEX message to port number");
    println!("  sysex --dev <name>       Send SYSEX message to first matching device");
    println!("                           Example: sysex --port 0 F0 00 F7");
    println!("                           Example: sysex --dev roland F0 00 F7");
}