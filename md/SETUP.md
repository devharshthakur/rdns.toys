# Setup Guide

This is the project setup guide which will help you get RDNS Toys up and running on your system.

## Getting Started

### Prerequisites

- [x] Rust 1.70+ (edition 2021)
- [x] Cargo package manager
- [x] Node.js 18+ (for formatting)
- [x] Just command runner (optional but recommended)

### Quick Start

1. Clone the repository:

   ```bash
   git clone https://github.com/devharshthakur/rdns_toys.git
   cd rdns_toys
   ```

2. Install Just code runner (optional but recommended):

   ```bash
   # macOS
   brew install just

   # Linux
   curl --proto '=https' --tlsv1.2 -sSf https://just.systems/install.sh | bash

   # Windows
   scoop install just
   ```

3. Setup and run:
   ```bash
   just setup
   just run
   ```

### Manual Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/devharshthakur/rdns_toys.git
   cd rdns_toys
   ```

2. Build the project:

   ```bash
   cargo build --release
   ```

3. Run the server:
   ```bash
   cargo run --release
   ```

### Configuration

Details will be added later as project is its early stages

````

## Development

### Using Just Code Runner (Recommended)

The project includes a `justfile` with common development commands. For now its is simple, but as project progresses it will eventualy become usefull

```bash
# Show all available commands
just

# Setup development environment
just setup

# Run the project
just run

# Run tests
just test

# Format code
just fmt

# Format Markdown files
just format

# Run all checks
just check

# Pre-commit checks
just pre-commit

# Development workflow
just dev
````

### Manual Commands

If you prefer not to use Just:

```bash
# Build
cargo build

# Run
cargo run

# Test
cargo test

# Format Rust code
cargo fmt

# Format Markdown
pnpm format

# Clippy checks
cargo clippy
```

## Testing

Run the test suite:

```bash
# Using Just
just test

# Or manually
cargo test
```

### Common Issues

1. **Rust version too old**: Update Rust using `rustup update`

### Getting Help

If you encounter any issues:

1. Check the [Issues](https://github.com/devharshthakur/rdns_toys/issues) page
2. Search existing discussions
3. Create a new issue with detailed information
