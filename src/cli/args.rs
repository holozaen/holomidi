// This file contains the logic for parsing command-line arguments and validating them.

use std::env;

pub struct CommandLineArgs {
    pub command: String,
    pub options: Vec<String>,
}

impl CommandLineArgs {
    pub fn parse() -> Result<Self, String> {
        let args: Vec<String> = env::args().collect();
        if args.len() < 2 {
            return Err("Not enough arguments".to_string());
        }

        let command = args[1].clone();
        let options = args[2..].to_vec();

        Ok(CommandLineArgs { command, options })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_with_no_args() {
        std::env::set_var("CARGO_BIN_EXE_holomidi", "program");
        let result = CommandLineArgs::parse();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Not enough arguments");
    }

    #[test]
    fn test_parse_with_command() {
        std::env::set_args(vec!["program".into(), "list".into()]);
        let result = CommandLineArgs::parse().unwrap();
        assert_eq!(result.command, "list");
        assert!(result.options.is_empty());
    }

    #[test]
    fn test_parse_with_options() {
        std::env::set_args(vec![
            "program".into(),
            "sysex".into(),
            "--port".into(),
            "1".into(),
        ]);
        let result = CommandLineArgs::parse().unwrap();
        assert_eq!(result.command, "sysex");
        assert_eq!(result.options, vec!["--port", "1"]);
    }
}