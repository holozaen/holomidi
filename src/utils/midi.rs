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
        let result = find_port_by_name("nonexistent");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("No MIDI device found"));
    }

    #[test]
    fn test_send_sysex_invalid_port() {
        let test_data = vec![0xF0, 0x00, 0xF7];
        let result = send_sysex(999, test_data);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid port index"));
    }

    // Note: We can't easily test successful cases without mocking the MIDI devices
    // Consider adding integration tests with actual MIDI devices
    // or implementing a trait for MidiOutput to make it mockable
    
    // Example of how we could test with a trait-based approach:
    /*
    trait MidiDevice {
        fn ports(&self) -> Vec<u8>;
        fn port_name(&self, port: &u8) -> Result<String, ()>;
        fn connect(&self, port: &u8, name: &str) -> Result<(), String>;
    }

    #[test]
    fn test_find_port_by_name_mock() {
        let mock_device = MockMidiDevice::new()
            .with_ports(vec![1, 2])
            .with_port_names(vec!["Test Roland", "Test Yamaha"]);
        
        let result = find_port_by_name_with_device("roland", &mock_device);
        assert_eq!(result.unwrap(), 0);
    }
    */
}