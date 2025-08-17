# DNS Toys - Rust Port

A Rust implementation of [dns.toys](https://dns.toys), a DNS server that provides various utility
services accessible via DNS queries.

## 🎯 Project Overview

This project is a complete rewrite of the original Go-based dns.toys server in Rust. DNS Toys
provides a collection of utility services that can be accessed through DNS queries, making it easy
to get information like weather, currency conversion, time zones, and more directly from the command
line or any DNS client.

## 📋 Porting Status

See the [Porting Checklist](PORTING_CHECKLIST.md) for detailed progress on the Go-to-Rust porting
effort.

## 🤝 Contributing

Please read my [Contributing Guidelines](CONTRIBUTING.md) before submitting any contributions.

**Important**: This project is currently in active porting phase. I am not accepting new feature
contributions at this time. Please focus on:

- Bug fixes
- Code quality improvements
- Documentation improvements
- Performance optimizations
- Testing improvements

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Original [dns.toys](https://dns.toys) project by [@knadh](https://github.com/knadh)
- The Rust community for excellent tooling and libraries
- All contributors to this porting effort

## 📞 Support

If you encounter any issues or have questions:

1. Check the [Issues](https://github.com/devharshthakur/rdns_toys/issues) page
2. Search existing discussions
3. Create a new issue with detailed information

## 🔗 Projects Acknowlagements

- [Original dns.toys (Go)](https://github.com/knadh/dns.toys)
- [dns.toys Website/docs](https://dns.toys)

---

**Note**: This is a work in progress. The project is being actively ported from Go to Rust, and not
all features may be available yet. Check the [Porting Checklist](PORTING_CHECKLIST.md) for current
progress.

## 🚀 Getting Started

### Prerequisites

- Rust 1.70+ (edition 2021)
- Cargo package manager
- Node.js 18+ (for formatting)
- Just command runner (optional but recommended)

### Quick Start

1. Clone the repository:

   ```bash
   git clone https://github.com/devharshthakur/rdns_toys.git
   cd rdns_toys
   ```

2. Install Just (optional but recommended):

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

Create a `config.toml` file in the project root:

```toml
[server]
address = "0.0.0.0:53"
domain = "dns.toys"

[weather]
api_key = "your_openweathermap_api_key"
cache_duration = 300

[fx]
cache_duration = 60
```

## 🛠️ Development

### Using Just (Recommended)

The project includes a `justfile` with common development commands:

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
```

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

## 🧪 Testing

Run the test suite:

```bash
# Using Just
just test

# Or manually
cargo test
```

## 📖 Usage Examples

Once project parts are finished it will be updated .
