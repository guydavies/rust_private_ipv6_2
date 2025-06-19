# private_ipv6_2 (Rust Port)

A Rust implementation of a tool to create a random IPv6 private CIDR (/64).

## Overview

This program generates a random IPv6 address in the fd00::/8 prefix, which is reserved for private networks (similar to how 10.0.0.0/8, 172.16.0.0/12, and 192.168.0.0/16 are reserved in IPv4).

The generated address follows the format:
- First group: Always starts with 'fd' followed by a random byte
- Groups 2-4: Random hexadecimal values (0-65535)
- The CIDR suffix: Always /64

## Usage

```
cargo run
```

Example output:
```
fd3a:1c2f:b841:5e72:/64
```

## Building

```
cargo build --release
```

The compiled binary will be available at `target/release/private_ipv6_2`.

## Author

Originally written in Go by Guy Davies (aguydavies@gmail.com)
Ported to Rust using Copilot by Guy Davies (aguydavies@gmail.com)
