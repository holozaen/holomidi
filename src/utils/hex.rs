pub fn parse_hex(hex_str: &str) -> Result<u8, String> {
    u8::from_str_radix(hex_str, 16)
        .map_err(|e| format!("Invalid hex value '{}': {}", hex_str, e))
}

pub fn parse_sysex_message(args: &[String]) -> Result<Vec<u8>, String> {
    let mut message = Vec::new();
    for hex in args {
        message.push(parse_hex(hex)?);
    }
    
    if message.first() != Some(&0xF0) || message.last() != Some(&0xF7) {
        return Err("SYSEX message must start with F0 and end with F7".to_string());
    }
    
    Ok(message)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_hex_valid() {
        assert_eq!(parse_hex("F0").unwrap(), 0xF0);
        assert_eq!(parse_hex("00").unwrap(), 0x00);
        assert_eq!(parse_hex("FF").unwrap(), 0xFF);
    }

    #[test]
    fn test_parse_hex_invalid() {
        assert!(parse_hex("GG").is_err());
        assert!(parse_hex("").is_err());
        assert!(parse_hex("FFF").is_err());
    }

    #[test]
    fn test_parse_sysex_message_valid() {
        let input = vec!["F0".to_string(), "00".to_string(), "F7".to_string()];
        let result = parse_sysex_message(&input).unwrap();
        assert_eq!(result, vec![0xF0, 0x00, 0xF7]);
    }

    #[test]
    fn test_parse_sysex_message_invalid_start() {
        let input = vec!["00".to_string(), "00".to_string(), "F7".to_string()];
        assert!(parse_sysex_message(&input).is_err());
    }

    #[test]
    fn test_parse_sysex_message_invalid_end() {
        let input = vec!["F0".to_string(), "00".to_string(), "00".to_string()];
        assert!(parse_sysex_message(&input).is_err());
    }
}