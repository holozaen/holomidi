use crate::utils::{hex, midi};

pub fn execute(args: &[String]) -> Result<(), String> {
    if args.len() < 3 {
        return Err("sysex command requires port selection and hex values".to_string());
    }

    let port = match args[0].as_str() {
        "--port" => {
            match args[1].parse() {
                Ok(p) => Ok(p),
                Err(_) => Err("Invalid port number".to_string())
            }
        },
        "--dev" => midi::find_port_by_name(&args[1]),
        _ => Err("Must specify --port or --dev".to_string())
    }?;

    let sysex_data = hex::parse_sysex_message(&args[2..])?;
    midi::send_sysex(port, sysex_data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execute_no_args() {
        let args: Vec<String> = vec![];
        let result = execute(&args);
        assert!(result.is_err());
    }

    #[test]
    fn test_execute_invalid_option() {
        let args: Vec<String> = vec!["--invalid".to_string(), "0".to_string()];
        let result = execute(&args);
        assert!(result.is_err());
    }

    #[test]
    fn test_execute_invalid_port_number() {
        let args = vec![
            "--port".to_string(),
            "not_a_number".to_string(),
            "F0".to_string(),
            "00".to_string(),
            "F7".to_string(),
        ];
        let result = execute(&args);
        assert!(result.is_err());
    }
}