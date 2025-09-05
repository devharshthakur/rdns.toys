# WARP.md

This file provides guidance to WARP (warp.dev) when working with code in this repository.

## Project Overview

This is a Rust port of the original [dns.toys](https://dns.toys) project - a DNS server that
provides utility services accessible via DNS queries. The project is **currently in active porting
phase** from Go to Rust and not all features are implemented yet.

## Development Commands

The project uses [Just](https://github.com/casey/just) as the primary task runner. Install with
`brew install just` (macOS) or `scoop install just` (Windows).

### Primary Commands

```bash
# Run the DNS server (default command)
just run

# Build the project
just build

# Build in release mode
just build-release

# Run tests
just test

# Format all code and config files
just format

# Check formatting without applying
just format-check

# Run Rust linter (clippy)
just lint

# Clean build artifacts
just clean

# Show project information
just info
```

### Manual Commands (if not using Just)

```bash
# Rust commands
cargo build --release
cargo run --release
cargo test
cargo fmt
cargo clippy

# Format markdown and config files
pnpm format
pnpm format:check
```

### Single Test Execution

```bash
# Run specific test
cargo test test_name

# Run tests in a specific module
cargo test module_name

# Run tests with output
cargo test -- --nocapture
```

## Code Architecture

### Project Structure

- **`src/main.rs`** - Entry point (minimal during porting phase)
- **`src/handlers.rs`** - Core DNS request handler and service registry (~430 lines)
- **`src/geo/mod.rs`** - Geolocation service for timezone lookups (~147 lines)
- **`data/`** - Contains external data files required by services
- **`justfile`** - Task runner configuration

### Core Architecture Components

#### DNS Service Registry (`src/handlers.rs`)

The main architectural pattern is a **plugin-based service registry**:

- **`DnsHandlers`** - Central coordinator that manages service registration and request routing
- **`Service` trait** - Async trait defining the interface for all DNS services (`query()` and
  `dump()` methods)
- **Service Registration** - Services register themselves with a DNS suffix (e.g., "ip", "time",
  "pi")
- **Request Routing** - Queries like "mumbai.time.example.com" get routed to the "time" service

#### Key Methods

- `DnsHandlers::new()` - Creates handler with help records
- `register()` - Adds services to the registry
- `handle_ip_query()` - Built-in IP echo service (returns client's IP)
- `handle_pi_query()` - Built-in Pi constant service

#### Geolocation Service (`src/geo/mod.rs`)

- **`Location` struct** - Represents geographic locations with coordinates, timezone, population
- **`Geo` struct** - Manages location database with timezone-based indexing
- **Data Source** - Parses geonames.org cities15000.txt file (tab-delimited)
- **Search Strategy** - Indexes by city name and timezone aliases, sorted by population

### Service Architecture Pattern

Each DNS service follows this pattern:

1. Implement the async `Service` trait
2. Register with a DNS suffix via `DnsHandlers::register()`
3. Handle queries through the `query()` method
4. Return DNS-compatible response strings

### Dependencies

- **hickory-dns** - DNS server implementation and protocol handling
- **tokio** - Async runtime
- **anyhow** - Error handling
- **regex** - Query cleaning and validation
- **chrono-tz** - Timezone handling for geolocation
- **csv** - Parsing geonames.org data files

## Project Status & Porting Context

### Current Phase

The project is **actively being ported from Go to Rust**. According to the checklist:

- Core Infrastructure: 20% complete
- Services: 0% complete
- Testing: 0% complete
- Documentation: 40% complete

### Contribution Guidelines

**During porting phase, only accept:**

- Bug fixes in existing ported code
- Code quality improvements (error handling, type safety, performance)
- Documentation improvements
- Testing additions

**Do NOT add:**

- New DNS services not in original
- New features beyond the original scope
- Major architectural changes
- Breaking changes

### Planned Services (from CHECKLIST.md)

The project will eventually implement these services:

- **Simple**: base conversion, pi, IP echo, random numbers/dice
- **Data-driven**: units conversion, dictionary (WordNet), excuses
- **External API**: weather (OpenWeatherMap), currency (FX rates), timezones
- **Advanced**: UUID generation, Sudoku solver, aerial distance calculations

### Data Dependencies

Services require external data files in `data/`:

- `cities15000.txt` - Geonames city database for timezone service
- `wordnet/` directory - WordNet lexical database for dictionary service
- `ifsc/` directory - Indian banking IFSC codes
- `excuses.txt` - Developer excuse collection
- `vitamins.json` - Vitamin/nutrition information

## Development Workflow

### Formatting & Code Style

- **Rust**: Uses `cargo fmt` with default rustfmt settings
- **Other files**: Uses Prettier via `pnpm format` for Markdown, JSON, TOML
- **Linting**: Uses `cargo clippy` for Rust best practices

### Prerequisites

- Rust 1.70+ (edition 2021)
- Node.js 18+ (for Prettier formatting)
- Cargo package manager
- Just command runner (recommended)

### Branching & Commits

- Use conventional commits: `fix:`, `feat:`, `docs:`, `test:`, `refactor:`
- Branch format: `<type>/<issue-title>` (e.g., `fix/memory-leak-in-weather-service`)
- Issue-first workflow required for significant changes
