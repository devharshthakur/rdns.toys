# DNS Toys - Porting Checklist

This document tracks the progress of porting the original Go-based
[dns.toys](https://github.com/knadh/dns.toys) project to Rust.

> **Note:** This checklist is generated with the assistance of AI and is subject to change or
> updates over time as the project evolves.

## Porting Overview

- **Original Project**: [dns.toys (Go)](https://github.com/knadh/dns.toys) by
  [@knadh](https://github.com/knadh)
- **Target**: Complete Rust implementation with feature parity
- **Status**: Active porting phase
- **Last Updated**: August 17 2025

## Core Infrastructure

- [x] Project structure setup
- [x] Cargo.toml configuration
- [x] Basic project documentation
- [x] Development tooling (Just, Prettier)
- [x] Contributing guidelines
- [x] Code of conduct
- [ ] Configuration system
- [ ] DNS server core
- [ ] Service trait definition
- [ ] Error handling framework
- [ ] Logging setup
- [ ] Request routing system
- [ ] Response formatting
- [ ] Caching system
- [ ] Rate limiting
- [ ] Health checks

## Services

> **Note:** The file names listed below are tentative and may change as the project evolves. Any
> updates to file names will be reflected here.

### Simple Services (No External Dependencies)

- [ ] Base conversion service (`base.rs`)
  - [ ] Hex to decimal conversion
  - [ ] Decimal to hex conversion
  - [ ] Binary conversion
  - [ ] Octal conversion
  - [ ] Input validation
  - [ ] Error handling

- [ ] Pi service (`pi.rs`)
  - [ ] Return digits of π
  - [ ] Configurable precision
  - [ ] Performance optimization

- [ ] IP echo service (`ip.rs`)
  - [ ] Return client IP address
  - [ ] IPv4/IPv6 support
  - [ ] Network interface detection

- [ ] Random services (`random.rs`)
  - [ ] Dice rolling
  - [ ] Coin tossing
  - [ ] Random number generation
  - [ ] Configurable ranges

### Data-Driven Services

- [ ] Units conversion service (`units.rs`)
  - [ ] Load units data from JSON
  - [ ] Length conversions
  - [ ] Weight conversions
  - [ ] Temperature conversions
  - [ ] Area conversions
  - [ ] Volume conversions

- [ ] Dictionary service (`dict.rs`)
  - [ ] WordNet integration
  - [ ] Word definitions
  - [ ] Synonyms and antonyms
  - [ ] File-based data loading

- [ ] Excuse service (`excuse.rs`)
  - [ ] Load excuses from text file
  - [ ] Random selection
  - [ ] Categorized excuses
  - [ ] Custom excuse generation

### External API Services

- [ ] Weather service (`weather.rs`)
  - [ ] OpenWeatherMap API integration
  - [ ] Location parsing
  - [ ] Temperature conversion
  - [ ] Caching system
  - [ ] Error handling
  - [ ] Rate limiting

- [ ] FX Currency service (`fx.rs`)
  - [ ] Exchange rate API integration
  - [ ] Currency conversion
  - [ ] Real-time rates
  - [ ] Historical data
  - [ ] Caching system
  - [ ] Snapshot functionality

- [ ] Timezone service (`timezones.rs`)
  - [ ] Geo location data loading
  - [ ] Timezone calculations
  - [ ] Current time retrieval
  - [ ] Timezone conversion
  - [ ] Location search

### Advanced Services

- [ ] UUID generation service (`uuid.rs`)
  - [ ] UUID v4 generation
  - [ ] Multiple results support
  - [ ] Custom formats
  - [ ] Performance optimization

- [ ] Sudoku solver service (`sudoku.rs`)
  - [ ] Algorithm implementation
  - [ ] Input parsing
  - [ ] Solution validation
  - [ ] Multiple solutions
  - [ ] Performance optimization

- [ ] Aerial distance service (`aerial.rs`)
  - [ ] Geographic calculations
  - [ ] Coordinate parsing
  - [ ] Distance algorithms
  - [ ] Multiple coordinate formats
  - [ ] Performance optimization

## Testing & Quality

- [ ] Unit tests for all services
- [ ] Integration tests
- [ ] Error handling tests
- [ ] API compatibility tests

## Documentation

- [x] README.md
- [x] CONTRIBUTING.md
- [x] CODE_OF_CONDUCT.md
- [x] Pull request template
- [ ] API documentation
- [ ] Service documentation
- [ ] Configuration guide
- [ ] Deployment instructions
- [ ] Performance tuning guide
- [ ] Troubleshooting guide

## Progress Tracking

- **Core Infrastructure**: 20% complete
- **Services**: 0% complete
- **Testing**: 0% complete
- **Documentation**: 40% complete
- **Deployment**: 0% complete

## Notes

- This checklist is updated as progress is made
- Priority is given to core functionality over advanced features
- Performance and security are key considerations
- Community feedback will influence the roadmap

---

**Last Updated**: January 2025  
**Maintainer**: [@devharshthakur](https://github.com/devharshthakur)
