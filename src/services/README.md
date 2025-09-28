# DNS Services

This directory contains various DNS services that provide utilities and information through DNS queries.

## Available Services

### Geo Service (`geo/`)
Provides geographic information and timezone data for cities worldwide.
- **Query format**: `dig TXT <city>.geo.localhost`
- **Example**: `dig TXT mumbai.geo.localhost`

### IP Service (`ip/`)
Returns the client's IP address in various formats.
- **Query format**: `dig TXT ip.localhost` or `dig A ip.localhost`
- **Example**: `dig TXT ip.localhost`

### Pi Service (`pi/`)
Returns the mathematical constant π in different formats.
- **Query format**: `dig TXT pi.localhost`, `dig A pi.localhost`, or `dig AAAA pi.localhost`
- **Example**: `dig TXT pi.localhost`

### Random Service (`random/`)
Generates random numbers within a specified range.
- **Query format**: `dig TXT <min>-<max>.random.localhost`
- **Example**: `dig TXT 1-100.random.localhost`

### UUID Service (`uuid/`)
Generates random UUIDs.
- **Query format**: `dig TXT <count>.uuid.localhost`
- **Example**: `dig TXT 5.uuid.localhost`

## Implementation Notes

All regex expressions and core logic are adapted from the original [dns.toys](https://github.com/knadh/dns.toys) project by [@knadh](https://github.com/knadh).

