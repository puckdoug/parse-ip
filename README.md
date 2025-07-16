# parse-ip

A crate to make parsing IP addresses and hostnames from strings (e.g. the
command-line) simple and easy. It's often convenient to accept IP address and
Port in a single command. With IPv4, IPv6 and Port mixed in the formatting
options become complicated. This library provides a simple interface to simplify
parsing the information and providing back the details in an accesible form.

```rust
use parse_ip::parse;

let bind_to: IpInfo = parse("10.0.0.23:8000"); // returns a Result

```

If you find an IP format that this doesn't handle, please log an issue so I may
update this to handle it.
