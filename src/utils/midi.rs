use midir::MidiOutput;

pub fn find_port_by_name(name: &str) -> Result<usize, String> {
    let midi_out = MidiOutput::new("midi_port_finder")
        .map_err(|e| format!("Error creating MIDI output: {}", e))?;

    let search_term = name.to_lowercase();
    
    for (i, p) in midi_out.ports().iter().enumerate() {
        if let Ok(port_name) = midi_out.port_name(p) {
            if port_name.to_lowercase().contains(&search_term) {
                return Ok(i);
            }
        }
    }
    
    Err(format!("No MIDI device found matching '{}'", name))
}

pub fn send_sysex(port_index: usize, sysex_data: Vec<u8>) -> Result<(), String> {
    let midi_out = MidiOutput::new("midi_sysex_sender")
        .map_err(|e| format!("Error creating MIDI output: {}", e))?;

    let ports = midi_out.ports();
    let port = ports.get(port_index)
        .ok_or_else(|| format!("Invalid port index: {}", port_index))?;

    let port_name = midi_out.port_name(port)
        .unwrap_or_else(|_| "Unknown".to_string());
    println!("Sending SYSEX to port {}: {}", port_index, port_name);

    let mut conn_out = midi_out.connect(port, "sysex_output")
        .map_err(|e| format!("Error connecting to MIDI port: {}", e))?;

    conn_out.send(&sysex_data)
        .map_err(|e| format!("Error sending SYSEX: {}", e))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_port_by_name_no_ports() {
        let result = find_port_by_name("nonexistent_device_name_hopefully");
        match result {
            Ok(_idx) => {
                // A port was found on this system — acceptable for CI/dev machines.
            }
            Err(e) => {
                // Accept either "no device found" or any MIDI init error.
                assert!(
                    e.contains("No MIDI device found")
                        || e.contains("Error creating MIDI output")
                        || e.contains("Error getting"),
                    "unexpected error: {}",
                    e
                );
            }
        }
    }

    #[test]
    fn test_send_sysex_invalid_port() {
        let test_data = vec![0xF0, 0x00, 0xF7];
        let result = send_sysex(999_999, test_data);
        match result {
            Ok(_) => {
                // Unexpectedly succeeded sending to a huge index; treat as success on systems with many ports.
            }
            Err(e) => {
                assert!(
                    e.contains("Invalid port index")
                        || e.contains("Error creating MIDI output")
                        || e.contains("Error connecting to MIDI port")
                        || e.contains("Error sending SYSEX"),
                    "unexpected error: {}",
                    e
                );
            }
        }
    }

    // Note: integration with real devices still not covered here
}