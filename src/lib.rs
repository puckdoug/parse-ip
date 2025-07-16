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
            IpVersion::V4(addr) => write!(f, "{}", addr),
            IpVersion::V6(addr) => write!(f, "{}", addr),
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

impl Into<IpAddr> for IpVersion {
    fn into(self) -> IpAddr {
        match self {
            IpVersion::V4(v4) => IpAddr::V4(v4),
            IpVersion::V6(v6) => IpAddr::V6(v6),
        }
    }
}

pub fn parse(input: &str) -> Result<(IpVersion, Option<u16>), String> {
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
            Err(_) => return Err(format!("Invalid IPv6 address in brackets: {}", addr_str)),
        }
    }

    // Try to parse as plain IP address (IPv4 or IPv6)
    match IpAddr::from_str(input) {
        Ok(addr) => Ok((IpVersion::from(addr), None)),
        Err(_) => Err(format!("Invalid IP address: {}", input)),
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
}
