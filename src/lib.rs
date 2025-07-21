use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
pub enum IpVersion {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

impl std::fmt::Display for IpVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IpVersion::V4(addr) => write!(f, "{addr}"),
            IpVersion::V6(addr) => write!(f, "{addr}"),
        }
    }
}

impl From<IpAddr> for IpVersion {
    fn from(addr: IpAddr) -> Self {
        match addr {
            IpAddr::V4(v4) => IpVersion::V4(v4),
            IpAddr::V6(v6) => IpVersion::V6(v6),
        }
    }
}

pub fn parse(input: &str) -> Result<(IpVersion, Option<u16>), String> {
    let nospace: String = input.chars().filter(|c| !c.is_whitespace()).collect();
    let mut input: &str = nospace.as_str();

    // Handle protocol prefixes (http://, https://, ftp://, etc.)
    if let Some(pos) = input.find("://") {
        input = &input[pos + 3..];
    }

    // Handle network socket notation generically (inet:, tcp4:, tcp6:, inet_addr:, in_addr_t:, etc.)
    if let Some(colon_pos) = input.find(':') {
        let prefix = &input[..colon_pos];
        // Check if this looks like a socket notation prefix (letters, numbers, underscore)
        if prefix.chars().all(|c| c.is_alphanumeric() || c == '_')
            && !prefix.is_empty()
            && colon_pos < input.len() - 1
            && !input.contains('%')
        // Not scoped IPv6
        {
            let addr_part = &input[colon_pos + 1..];
            // Check if the part after colon looks like an IP address (not just a port number)
            if addr_part.contains('.')  // IPv4 pattern
                || addr_part.contains(':') // IPv6 pattern
                || addr_part.starts_with('[')
            // Bracketed IPv6
            {
                input = addr_part;
            }
        }
    }

    // Try to parse as a socket address first (with port)
    if let Ok(socket_addr) = SocketAddr::from_str(input) {
        let ip_version = IpVersion::from(socket_addr.ip());
        return Ok((ip_version, Some(socket_addr.port())));
    }

    // Handle IPv6 addresses with brackets but no port
    if input.starts_with('[') && input.ends_with(']') {
        let addr_str = &input[1..input.len() - 1];
        match Ipv6Addr::from_str(addr_str) {
            Ok(addr) => return Ok((IpVersion::V6(addr), None)),
            Err(_) => return Err(format!("Invalid IPv6 address in brackets: {addr_str}")),
        }
    }

    // Handle scoped IPv6 addresses (with zone identifier %)
    if input.contains('%') {
        // For scoped addresses, we need to strip the zone identifier for parsing
        let addr_part = if let Some(percent_pos) = input.find('%') {
            &input[..percent_pos]
        } else {
            input
        };

        if let Ok(addr) = Ipv6Addr::from_str(addr_part) {
            return Ok((IpVersion::V6(addr), None));
        }
    }

    // Try to parse as plain IP address (IPv4 or IPv6)
    match IpAddr::from_str(input) {
        Ok(addr) => Ok((IpVersion::from(addr), None)),
        Err(_) => Err(format!("Invalid IP address: {input}")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_ipv4() {
        let result = parse("10.0.0.1");
        assert_eq!(
            result,
            Ok((IpVersion::V4(Ipv4Addr::new(10, 0, 0, 1)), None))
        );
    }

    #[test]
    fn invalid_ipv4_number_too_high() {
        let result = parse("300.1.1.1");
        assert!(result.is_err());
    }

    #[test]
    fn error_cases() {
        let test_cases = vec![
            // Edge cases and invalid
            "invalid",             // Invalid
            "[invalid]",           // Invalid in brackets
            "192.168.1.1:99999",   // Invalid port (too high)
            "[2001:db8::1]:99999", // IPv6 with invalid port
            "2001:db8::1:60000",   // IPv6 with no brackets and a valid port
            "",                    // Empty string
        ];
        for input in test_cases {
            let result = parse(input);
            assert!(result.is_err());
        }
    }

    #[test]
    fn ok_cases() {
        let test_cases = vec![
            // IPv4 cases
            "192.168.1.1",    // Plain IPv4
            "192.168.1.1:80", // IPv4 with port
            "127.0.0.1:8080", // IPv4 localhost with port
            "0.0.0.0:443",    // IPv4 any address with port
            // IPv6 cases
            "2001:db8::1",                        // Plain IPv6
            "[2001:db8::1]",                      // IPv6 with brackets
            "[2001:db8::1]:80",                   // IPv6 with port
            "[::1]:8080",                         // IPv6 localhost with port
            "::1",                                // IPv6 localhost plain
            "[2001:db8:85a3::8a2e:370:7334]:443", // Full IPv6 with port
            "::ffff:192.168.1.1",                 // IPv4-mapped IPv6
            "[::ffff:192.168.1.1]:80",            // IPv4-mapped IPv6 with port
        ];
        for input in test_cases {
            let result = parse(input);
            assert!(result.is_ok());
        }
    }

    #[test]
    fn scoped_literal() {
        let test_cases = vec![
            "fe80::1ff:fe23:4567:890a%eth2", // Scoped literal IPv6 with zone index
            "fe80::1ff:fe23:4567:890a%3",    // Scoped literal IPv6 with Zone index - Windows style
        ];
        for input in test_cases {
            let result = parse(input);
            assert!(result.is_ok());
        }
    }

    #[test]
    fn with_protocol() {
        let test_cases = vec![
            "http://192.168.1.1:8080",
            "https://10.0.0.1:443",
            "ftp://192.168.1.1:21",
            "tcp://192.168.1.1:22",
            "udp://10.0.0.1:53",
            "ws://192.168.1.1:8080",
            "wss://[2001:db8::1]:443",
        ];
        for input in test_cases {
            let result = parse(input);
            assert!(result.is_ok());
        }
    }

    #[test]
    fn network_socket_notation() {
        let test_cases = vec![
            "inet:192.168.1.1:8080",
            "tcp4:192.168.1.1:22",
            "tcp6:[::1]:22",
        ];
        for input in test_cases {
            let result = parse(input);
            assert!(result.is_ok());
        }
    }

    #[test]
    fn with_spaces() {
        let test_cases = vec!["192.168.1.1 : 8080", "192 . 168 . 1 . 1", "[ ::1 ] : 22"];
        for input in test_cases {
            let result = parse(input);
            assert!(result.is_ok());
        }
    }
}
