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
- [x] Configuration system (basic)
- [x] DNS server core
- [x] Service trait definition (unified)
- [x] Error handling framework
- [x] Logging setup
- [x] Request routing system
- [x] Response formatting
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

- [x] Pi service (`pi/mod.rs`)
  - [x] Return digits of π (TXT records)
  - [x] IPv4 representation (A records)
  - [x] IPv6 representation (AAAA records)
  - [x] High precision support
  - [x] Performance optimization

- [x] IP echo service (built-in)
  - [x] Return client IP address
  - [x] IPv4/IPv6 support
  - [x] Network interface detection

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

- [x] Geolocation service (`geo/mod.rs`) (Not tested)
  - [x] Geo location data loading
  - [x] City/timezone lookups
  - [x] Population-based sorting
  - [x] Country filtering support
  - [x] Geonames.org data integration

- [x] IFSC service (`ifsc/mod.rs`)
  - [x] Indian bank branch data loading
  - [x] IFSC code lookups
  - [x] Bank, branch, address information
  - [x] State, city, district data
  - [x] JSON data integration

- [ ] Timezone service (`timezones.rs`)
  - [x] Geo location data loading (via geo service) (Not tested)
  - [ ] Timezone calculations
  - [ ] Current time retrieval
  - [ ] Timezone conversion
  - [ ] Location search

### Advanced Services

- [x] UUID generation service (`uuid/mod.rs`)
  - [x] UUID v4 generation
  - [x] Multiple results support
  - [x] Configurable count (1-10 UUIDs)
  - [x] Performance optimization

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

- **Core Infrastructure**: 85% complete
- **Services**: 40% complete
- **Testing**: 0% complete
- **Documentation**: 70% complete
- **Deployment**: 0% complete

## Notes

- This checklist is updated as progress is made
- Priority is given to core functionality over advanced features
- Performance and security are key considerations
- Community feedback will influence the roadmap
- **Unified Service Architecture**: All services now use a single `query()` method that handles both text-based queries and direct DNS record creation
- **Multi-Format Support**: Services can return different record types (TXT, A, AAAA) based on query type
- **Working Services**: Pi, UUID, IP Echo, Geolocation, and IFSC services are fully functional and testable

---

**Last Updated**: September 2025  
**Maintainer**: [@devharshthakur](https://github.com/devharshthakur)
