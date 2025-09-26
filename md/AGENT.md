# AGENT.md

This file provides guidance to WARP (warp.dev) when working with code in this repository.

## Project Overview

This is a Rust port of the original [dns.toys](https://dns.toys) project - a DNS server that
provides utility services accessible via DNS queries. The project is **currently in active porting
phase** from Go to Rust with core infrastructure and foundational services implemented.

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

- **`src/main.rs`** - DNS server entry point with UDP socket binding (~148 lines)
- **`src/lib.rs`** - Library exports (geo, handlers, ifsc, services modules)
- **`src/handlers.rs`** - Core DNS request handler and service registry (~426 lines)
- **`src/services/`** - Service implementations directory
  - **`src/services/mod.rs`** - Service registration and help system (~68 lines)
  - **`src/services/pi/mod.rs`** - Pi constant service with multi-format support (~161 lines)
  - **`src/services/uuid/mod.rs`** - UUID generation service (~106 lines)
- **`src/geo/mod.rs`** - Geolocation service for timezone lookups (~222 lines)
- **`src/ifsc/mod.rs`** - IFSC (Indian Financial System Code) service (~116 lines)
- **`data/`** - Contains external data files required by services
- **`justfile`** - Task runner configuration

### Core Architecture Components

#### DNS Service Registry (`src/handlers.rs`)

The main architectural pattern is a **plugin-based service registry**:

- **`DnsHandlers`** - Central coordinator that manages service registration and request routing
- **`Service` trait** - Unified async trait with single `query()` method for all DNS services
- **Service Registration** - Services register themselves with a DNS suffix (e.g., "ip", "pi", "uuid")
- **Request Routing** - Queries like "3.uuid.localhost" get routed to the "uuid" service
- **Unified Query Interface** - Single method handles both text-based and direct DNS record creation

#### Key Methods

- `DnsHandlers::new()` - Creates handler with help records
- `register()` - Adds services to the registry
- `handle_ip_query()` - Built-in IP echo service (returns client's IP)
- `process_service_request()` - Routes dynamic service queries using unified interface
- `clean_query()` - Sanitizes and extracts meaningful query portions
- `process_dns_query()` - Main entry point for DNS query processing

#### Service Trait Interface

```rust
#[async_trait]
pub trait Service: Send + Sync {
    /// Unified query method handling all DNS record types
    async fn query(
        &self,
        request: &Request,
        query_name: &Name,
        query_type: RecordType,
        cleaned_query: &str,
    ) -> Option<Vec<Record>>;
    
    async fn dump(&self) -> Result<Vec<u8>>;
}
```

#### Geolocation Service (`src/geo/mod.rs`)

- **`Location` struct** - Represents geographic locations with coordinates, timezone, population
- **`Geo` struct** - Manages location database with timezone-based indexing
- **Data Source** - Parses geonames.org cities15000.txt file (tab-delimited)
- **Search Strategy** - Indexes by city name and timezone aliases, sorted by population
- **Query Support** - Location search with optional country filtering (e.g., "london/gb")

#### IFSC Service (`src/ifsc/mod.rs`)

- **`Branch` struct** - Represents Indian bank branch information
- **`IFSC` struct** - Manages IFSC code database with fast lookups
- **Data Source** - Loads JSON files containing IFSC code mappings
- **Features** - Bank, branch, address, state, city, district information

### Service Architecture Pattern

Each DNS service follows this unified pattern:

1. Implement the async `Service` trait with unified `query()` method
2. Register with a DNS suffix via `DnsHandlers::register()`
3. Handle queries through the single `query()` method that receives:
   - Full DNS request context
   - Query name and record type
   - Cleaned query string (for text-based services)
4. Return `Vec<Record>` directly or `None` if unsupported

#### Service Implementation Examples

**Pi Service** - Uses `query_type` to determine record format:
```rust
async fn query(&self, _request: &Request, query_name: &Name, query_type: RecordType, _cleaned_query: &str) -> Option<Vec<Record>> {
    match query_type {
        RecordType::TXT => Some(vec![/* Pi as text */]),
        RecordType::A => Some(vec![/* Pi as IPv4 */]),
        RecordType::AAAA => Some(vec![/* Pi as IPv6 */]),
        _ => None,
    }
}
```

**UUID Service** - Uses `cleaned_query` for number of UUIDs:
```rust
async fn query(&self, _request: &Request, query_name: &Name, query_type: RecordType, cleaned_query: &str) -> Option<Vec<Record>> {
    if query_type != RecordType::TXT { return None; }
    // Generate UUIDs based on cleaned_query number
    Some(vec![/* UUID records */])
}
```

### Dependencies

- **hickory-dns** - DNS server implementation and protocol handling
- **tokio** - Async runtime
- **anyhow** - Error handling
- **regex** - Query cleaning and validation
- **chrono-tz** - Timezone handling for geolocation
- **csv** - Parsing geonames.org data files
- **serde/serde_json** - JSON serialization for IFSC data
- **async-trait** - Async trait support for service implementations
- **once_cell** - Lazy static initialization
- **tracing** - Structured logging

## Project Status & Porting Context

### Current Phase

The project is **actively being ported from Go to Rust** with significant progress made:

- **Core Infrastructure**: 85% complete
  - ✅ Service trait definition with unified query interface
  - ✅ DNS request handling and routing system
  - ✅ Response formatting and error handling
  - ✅ Query cleaning and sanitization
  - ✅ Help system and service registry
  - ✅ DNS server core with UDP socket binding
  - ✅ Unified service architecture
  - ⏳ Configuration system (basic)
  - ⏳ Caching and rate limiting

- **Services**: 40% complete
  - ✅ IP echo service (built-in, IPv4/IPv6 support)
  - ✅ Pi constant service (TXT, A, AAAA records)
  - ✅ UUID generation service (configurable count)
  - ✅ Geolocation service (full implementation)
  - ✅ IFSC service (data loading and indexing)
  - ⏳ Timezone service (geo data ready, service wrapper needed)
  - ⏳ All other planned services

- **Testing**: 0% complete
  - No test files found
  - Unit tests for services
  - Integration tests

- **Documentation**: 70% complete
  - Comprehensive code documentation
  - Architecture documentation
  - Setup and contributing guides
  - API documentation
  - Service-specific documentation

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

### Implemented vs Planned Services

#### Currently Implemented

- **IP Echo Service** - Returns client's IP address (IPv4/IPv6 support)
- **Pi Service** - Mathematical constant π in multiple formats (TXT, A, AAAA records)
- **UUID Service** - Generates configurable number of random UUIDs (TXT records)
- **Geolocation Service** - City/timezone lookups with population-based sorting
- **IFSC Service** - Indian bank branch information lookup

#### Partially Implemented

- **Timezone Service** - Geo data loaded, needs service wrapper implementation

#### Planned Services (from CHECKLIST.md)

- **Simple**: base conversion, random numbers/dice
- **Data-driven**: units conversion, dictionary (WordNet), excuses
- **External API**: weather (OpenWeatherMap), currency (FX rates)
- **Advanced**: Sudoku solver, aerial distance calculations

### Working Services & Usage Examples

The following services are currently functional and can be tested:

#### Pi Service
```bash
# Get Pi as text
dig @127.0.0.1 -p 8053 TXT pi.localhost
# Returns: "3.141592653589793238462643383279502884197169"

# Get Pi as IPv4 address
dig @127.0.0.1 -p 8053 A pi.localhost
# Returns: 3.141.59.27

# Get Pi as IPv6 address
dig @127.0.0.1 -p 8053 AAAA pi.localhost
# Returns: 3141:5926:5358:9793:2384:6264:3383:2795
```

#### UUID Service
```bash
# Generate 1 UUID (default)
dig @127.0.0.1 -p 8053 TXT uuid.localhost
# Returns: 1 random UUID

# Generate 3 UUIDs
dig @127.0.0.1 -p 8053 TXT 3.uuid.localhost
# Returns: 3 random UUIDs
```

#### IP Echo Service
```bash
# Get your IP as text
dig @127.0.0.1 -p 8053 TXT ip.localhost
# Returns: Your client IP address

# Get your IP as A record (IPv4 only)
dig @127.0.0.1 -p 8053 A ip.localhost
# Returns: Your client IP as A record
```

#### Help Service
```bash
# Get available services
dig @127.0.0.1 -p 8053 TXT help.localhost
# Returns: List of available services and usage examples
```

### Data Dependencies

Services require external data files in `data/`:

#### Currently Used

- `cities15000.txt` - Geonames city database for geolocation service
- `ifsc/` directory - Indian banking IFSC codes (JSON format)

#### Planned Dependencies

- `wordnet/` directory - WordNet lexical database for dictionary service
- `excuses.txt` - Developer excuse collection
- `vitamins.json` - Vitamin/nutrition information
- `units.json` - Unit conversion data

## Development Workflow

### Formatting & Code Style

- **Rust**: Uses `cargo fmt` with default rustfmt settings
- **Other files**: Uses Prettier via `pnpm format` for Markdown, JSON, TOML
- **Linting**: Uses `cargo clippy` for Rust best practices

### Prerequisites

- Rust 1.70+ (edition 2024)
- Node.js 18+ (for Prettier formatting)
- Cargo package manager
- Just command runner (recommended)

### Branching & Commits

- Use conventional commits: `fix:`, `feat:`, `docs:`, `test:`, `refactor:`
- Branch format: `<type>/<issue-title>` (e.g., `fix/memory-leak-in-weather-service`)
- Issue-first workflow required for significant changes
