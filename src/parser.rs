use std::collections::HashMap;

pub fn parse_key_value(input: &str, separator: char, trim_quotes: bool) -> HashMap<String, String> {
    let mut result = HashMap::new();

    for line in input.lines() {
        if let Some((key, value)) = line.split_once(separator) {
            let key = key.trim().to_string();
            let value = if trim_quotes {
                value.trim().trim_matches('"').to_string()
            } else {
                value.trim().to_string()
            };
            result.insert(key, value);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_key_value_with_colon() {
        let input = "Arch: x86_64\nModel: Test CPU";
        let result = parse_key_value(input, ':', false);

        assert_eq!(result.get("Arch"), Some(&"x86_64".to_string()));
        assert_eq!(result.get("Model"), Some(&"Test CPU".to_string()));
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_parse_key_value_with_equals() {
        let input = "NAME=\"Ubuntu\"\nVERSION=\"22.04\"";
        let result = parse_key_value(input, '=', true);

        assert_eq!(result.get("NAME"), Some(&"Ubuntu".to_string()));
        assert_eq!(result.get("VERSION"), Some(&"22.04".to_string()));
    }

    #[test]
    fn test_parse_key_value_without_trim_quotes() {
        let input = "NAME=\"Ubuntu\"";
        let result = parse_key_value(input, '=', false);

        assert_eq!(result.get("NAME"), Some(&"\"Ubuntu\"".to_string()));
    }

    #[test]
    fn test_parse_key_value_empty_lines() {
        let input = "Key1: Value1\n\nKey2: Value2";
        let result = parse_key_value(input, ':', false);

        assert_eq!(result.len(), 2);
        assert_eq!(result.get("Key1"), Some(&"Value1".to_string()));
        assert_eq!(result.get("Key2"), Some(&"Value2".to_string()));
    }

    #[test]
    fn test_parse_key_value_no_separator() {
        let input = "This is just text\nKey: Value";
        let result = parse_key_value(input, ':', false);

        assert_eq!(result.len(), 1);
        assert_eq!(result.get("Key"), Some(&"Value".to_string()));
    }

    #[test]
    fn test_parse_key_value_whitespace() {
        let input = "  Key  :  Value  ";
        let result = parse_key_value(input, ':', false);

        assert_eq!(result.get("Key"), Some(&"Value".to_string()));
    }
}
