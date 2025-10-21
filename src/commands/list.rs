use midir::{MidiInput, MidiOutput};

pub fn execute() -> Result<(), String> {
    let midi_in = MidiInput::new("midi_ports_list")
        .map_err(|e| format!("Error creating MIDI input: {}", e))?;
    let midi_out = MidiOutput::new("midi_ports_list")
        .map_err(|e| format!("Error creating MIDI output: {}", e))?;

    println!("\nAvailable MIDI input ports:");
    for (i, p) in midi_in.ports().iter().enumerate() {
        let port_name = midi_in.port_name(p).unwrap_or("Unknown".to_string());
        println!("{}: {}", i, port_name);
    }

    println!("\nAvailable MIDI output ports:");
    for (i, p) in midi_out.ports().iter().enumerate() {
        let port_name = midi_out.port_name(p).unwrap_or("Unknown".to_string());
        println!("{}: {}", i, port_name);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // Since this command interacts with actual MIDI devices,
    // we should mock the MidiInput and MidiOutput traits

    #[test]
    fn test_execute_no_ports() {
        // This test will pass on systems with no MIDI devices
        let result = execute();
        assert!(result.is_ok());
    }
}