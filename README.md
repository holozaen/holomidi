# Holomidi Project

Holomidi is a Rust application designed to interact with MIDI devices. It provides functionalities to list available MIDI ports and send SYSEX messages to connected devices.

## Features

- **List MIDI Ports**: Easily list all available MIDI input and output ports.
- **Send SYSEX Messages**: Send System Exclusive (SYSEX) messages to MIDI devices using their port number or device name.

## Structure

The project is organized into several modules for better maintainability and extensibility:

- `src/main.rs`: Entry point of the application.
- `src/commands/`: Contains command implementations.
  - `list.rs`: Handles the listing of MIDI ports.
  - `sysex.rs`: Manages sending SYSEX messages.
- `src/utils/`: Contains utility functions for MIDI and hex operations.
  - `midi.rs`: MIDI-related utility functions.
  - `hex.rs`: Functions for parsing hexadecimal strings.
- `src/cli/`: Command-line interface functionalities.
  - `args.rs`: Logic for parsing and validating command-line arguments.

## Usage

To use the application, run it from the command line with the desired command:

```bash
cargo run -- <command> [options]
```

### Commands

- `list`: Lists all available MIDI ports.
- `sysex --port <n> <data>`: Sends a SYSEX message to the specified port number.
- `sysex --dev <name> <data>`: Sends a SYSEX message to the first matching device name.

## Installation

1. Clone the repository:
   ```bash
   git clone <repository-url>
   ```
2. Navigate to the project directory:
   ```bash
   cd holomidi
   ```
3. Build the project:
   ```bash
   cargo build
   ```

## Contributing

Contributions are welcome! Please open an issue or submit a pull request for any improvements or bug fixes.

## License

This project is licensed under the MIT License. See the LICENSE file for details.