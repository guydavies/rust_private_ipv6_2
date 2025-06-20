use rand::Rng;

/// Generate a random hexadecimal string with a value up to the maximum provided
fn generate_hex(max_hex_value_as_int: u16) -> String {
    let random_value = rand::rng().random_range(0..=max_hex_value_as_int);
    format!("{:x}", random_value)
}

/// Generates a random private IPv6 address with specific values for testing
#[cfg(test)]
fn generate_private_ipv6_with_values(first_byte: u8, group2: u16, group3: u16, group4: u16) -> String {
    let hex_first_byte = "fd";
    let hex_byte = format!("{:02x}", first_byte);
    let hex_two_byte_str = format!("{}{}", hex_first_byte, hex_byte);

    let mut addr_all_groups = Vec::new();
    addr_all_groups.push(hex_two_byte_str);
    addr_all_groups.push(format!("{:x}", group2));
    addr_all_groups.push(format!("{:x}", group3));
    addr_all_groups.push(format!("{:x}", group4));

    format!("{}:/64", addr_all_groups.join(":"))
}

/// Generate a random private IPv6 address with CIDR notation
pub fn generate_private_ipv6() -> String {
    // Construct random private IPv6 CIDR block
    //
    // First construct the first group which must begin with 'fd' and append a
    // random byte
    //
    // Then iterate three times to construct three more random hex blocks of 4
    // hex digits (two bytes) each
    let max_hex_value_as_int: u16 = 65535;
    let hex_first_byte = "fd";
    let hex_byte: String = generate_hex(255);
    let hex_two_byte_str = format!("{}{}", hex_first_byte, hex_byte);

    let mut addr_all_groups = Vec::new();
    addr_all_groups.push(hex_two_byte_str);

    for _ in 0..3 {
        addr_all_groups.push(generate_hex(max_hex_value_as_int));
    }

    // Join the address groups with colons and append the CIDR notation
    format!("{}:/64", addr_all_groups.join(":"))
}

/// Validate if a string is a valid private IPv6 address with CIDR notation
pub fn is_valid_private_ipv6(ipv6_str: &str) -> bool {
    // Check if it ends with :/64
    if !ipv6_str.ends_with(":/64") {
        return false;
    }

    // Remove the CIDR part
    let address_part = &ipv6_str[0..ipv6_str.len() - 4];

    // Split by colon
    let groups: Vec<&str> = address_part.split(':').collect();

    // Should have 4 groups
    if groups.len() != 4 {
        return false;
    }

    // First group must start with fd
    if !groups[0].starts_with("fd") {
        return false;
    }

    // All groups should be valid hexadecimal
    for group in groups {
        if group.is_empty() {
            return false;
        }

        for c in group.chars() {
            if !c.is_ascii_hexdigit() {
                return false;
            }
        }
    }

    true
}

fn main() {
    let ipv6_address = generate_private_ipv6();
    println!("{}", ipv6_address);
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn test_generate_hex() {
        // Test with max value 255
        let hex = generate_hex(255);
        let value = u16::from_str_radix(&hex, 16).unwrap();
        assert!(value <= 255);

        // Test with max value 65535
        let hex = generate_hex(65535);
        let value = u32::from_str_radix(&hex, 16).unwrap();
        assert!(value <= 65535);

        // Test with max value 0 (edge case)
        let hex = generate_hex(0);
        assert_eq!(hex, "0");

        // Test with max value 1
        let hex = generate_hex(1);
        let value = u16::from_str_radix(&hex, 16).unwrap();
        assert!(value <= 1);
    }

    #[test]
    fn test_generate_private_ipv6() {
        let ipv6 = generate_private_ipv6();

        // Check format
        assert!(is_valid_private_ipv6(&ipv6));

        // Check if starts with fd
        assert!(ipv6.starts_with("fd"));

        // Check if ends with :/64
        assert!(ipv6.ends_with(":/64"));

        // Check if it has 4 groups
        let groups: Vec<&str> = ipv6[0..ipv6.len() - 4].split(':').collect();
        assert_eq!(groups.len(), 4);

        // Generate multiple addresses and ensure they're different (probabilistic test)
        let ipv6_2 = generate_private_ipv6();
        let ipv6_3 = generate_private_ipv6();

        // It's extremely unlikely that 3 random IPv6 addresses would be identical
        assert!(ipv6 != ipv6_2 || ipv6 != ipv6_3 || ipv6_2 != ipv6_3);
    }

    #[test]
    fn test_is_valid_private_ipv6() {
        // Valid addresses
        assert!(is_valid_private_ipv6("fd01:2345:6789:abcd:/64"));
        assert!(is_valid_private_ipv6("fdff:1:2:3:/64"));
        assert!(is_valid_private_ipv6("fdaa:0:0:0:/64"));
        assert!(is_valid_private_ipv6("fdff:ffff:ffff:ffff:/64"));

        // Invalid addresses
        assert!(!is_valid_private_ipv6("fc01:2345:6789:abcd:/64")); // Not starting with fd
        assert!(!is_valid_private_ipv6("fd01:2345:6789:abcd:/48")); // Wrong CIDR
        assert!(!is_valid_private_ipv6("fd01:2345:6789:/64"));      // Not enough groups
        assert!(!is_valid_private_ipv6("fd01:2345:6789:abcd:ef01:/64")); // Too many groups
        assert!(!is_valid_private_ipv6("fd01:2345:6789:abcd")); // No CIDR notation
        assert!(!is_valid_private_ipv6("fd01:2345:6789:xyz:/64")); // Invalid hex
        assert!(!is_valid_private_ipv6("gd01:2345:6789:abcd:/64")); // Invalid first character
        assert!(!is_valid_private_ipv6("")); // Empty string
        assert!(!is_valid_private_ipv6("fd01::6789:abcd:/64")); // Contains empty group
        assert!(!is_valid_private_ipv6("fd01:2345:6789:abcd:/64extra")); // Extra text after CIDR
    }

    #[test]
    fn test_generate_private_ipv6_with_values() {
        // Test with minimum values
        let ipv6 = generate_private_ipv6_with_values(0, 0, 0, 0);
        assert_eq!(ipv6, "fd00:0:0:0:/64");
        assert!(is_valid_private_ipv6(&ipv6));

        // Test with maximum values
        let ipv6 = generate_private_ipv6_with_values(255, 65535, 65535, 65535);
        assert_eq!(ipv6, "fdff:ffff:ffff:ffff:/64");
        assert!(is_valid_private_ipv6(&ipv6));

        // Test with mixed values
        let ipv6 = generate_private_ipv6_with_values(10, 256, 4096, 1);
        assert_eq!(ipv6, "fd0a:100:1000:1:/64");
        assert!(is_valid_private_ipv6(&ipv6));
    }

    #[test]
    fn test_output_format_consistency() {
        // Generate multiple IPv6 addresses and check that they all follow the correct format
        for _ in 0..10 {
            let ipv6 = generate_private_ipv6();

            // Validate format
            assert!(is_valid_private_ipv6(&ipv6));

            // Extract and validate parts
            let without_cidr = &ipv6[0..ipv6.len() - 4];
            let groups: Vec<&str> = without_cidr.split(':').collect();

            // First group should start with "fd"
            assert!(groups[0].starts_with("fd"));

            // All groups should be valid hexadecimal
            for group in groups {
                assert!(!group.is_empty());
                for c in group.chars() {
                    assert!(c.is_ascii_hexdigit());
                }
            }
        }
    }

    proptest! {
        // Test that generate_hex always produces valid hex values within range
        #[test]
        fn prop_generate_hex_within_range(max_value in 0u16..=65535) {
            let hex = generate_hex(max_value);
            let parsed_value = u32::from_str_radix(&hex, 16).unwrap();
            prop_assert!(parsed_value <= max_value as u32);
        }

        // Test that generate_hex always produces valid hex strings
        #[test]
        fn prop_generate_hex_valid_hex(max_value in 0u16..=65535) {
            let hex = generate_hex(max_value);
            for c in hex.chars() {
                prop_assert!(c.is_ascii_hexdigit());
            }
        }

        // Test that is_valid_private_ipv6 correctly validates addresses
        #[test]
        fn prop_validate_generated_ipv6(
            byte in 0u8..=255,
            group2 in 0u16..=65535,
            group3 in 0u16..=65535,
            group4 in 0u16..=65535,
        ) {
            let ipv6 = generate_private_ipv6_with_values(byte, group2, group3, group4);
            prop_assert!(is_valid_private_ipv6(&ipv6));

            // Verify structure
            prop_assert!(ipv6.starts_with("fd"));
            prop_assert!(ipv6.ends_with(":/64"));

            let parts: Vec<&str> = ipv6[0..ipv6.len()-4].split(':').collect();
            prop_assert_eq!(parts.len(), 4);

            // Check specific parts match the input values
            let first_byte_hex = format!("{:02x}", byte);
            prop_assert_eq!(parts[0], format!("fd{}", first_byte_hex));

            // Parse the generated groups and compare with inputs
            let group2_parsed = u32::from_str_radix(parts[1], 16).unwrap();
            let group3_parsed = u32::from_str_radix(parts[2], 16).unwrap();
            let group4_parsed = u32::from_str_radix(parts[3], 16).unwrap();

            prop_assert_eq!(group2_parsed, group2 as u32);
            prop_assert_eq!(group3_parsed, group3 as u32);
            prop_assert_eq!(group4_parsed, group4 as u32);
        }

        // Test the validator with manually constructed addresses
        #[test]
        fn prop_validator_correctness(
            byte in 0u8..=255,
            group2 in 0u16..=65535,
            group3 in 0u16..=65535,
            group4 in 0u16..=65535,
        ) {
            // Valid address
            let valid = format!("fd{:02x}:{:x}:{:x}:{:x}:/64", byte, group2, group3, group4);
            prop_assert!(is_valid_private_ipv6(&valid));

            // Invalid prefix (not fd)
            let invalid_prefix = format!("fc{:02x}:{:x}:{:x}:{:x}:/64", byte, group2, group3, group4);
            prop_assert!(!is_valid_private_ipv6(&invalid_prefix));

            // Invalid CIDR
            let invalid_cidr = format!("fd{:02x}:{:x}:{:x}:{:x}:/48", byte, group2, group3, group4);
            prop_assert!(!is_valid_private_ipv6(&invalid_cidr));

            // Too few groups
            let too_few = format!("fd{:02x}:{:x}:{:x}:/64", byte, group2, group3);
            prop_assert!(!is_valid_private_ipv6(&too_few));

            // Too many groups
            let too_many = format!("fd{:02x}:{:x}:{:x}:{:x}:1:/64", byte, group2, group3, group4);
            prop_assert!(!is_valid_private_ipv6(&too_many));

            // No CIDR
            let no_cidr = format!("fd{:02x}:{:x}:{:x}:{:x}", byte, group2, group3, group4);
            prop_assert!(!is_valid_private_ipv6(&no_cidr));
        }
    }
}
