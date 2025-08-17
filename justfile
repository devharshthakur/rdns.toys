# DNS Toys Rust Port - Just Commands
# Run with: just <command>

# Default command - show all available commands
default:
    @just run

# Build the project
build:
    cargo build

# Build in release mode
build-release:
    cargo build --release

# Run the project
run:
    cargo run

# Run in release mode
run-release:
    cargo run --release

# Check Rust formatting
format-check:
    pnpm format:check
    cargo fmt -- --check

# Format Markdown and config files
format:
    cargo fmt
    pnpm format

# Clean build artifacts
clean:
    cargo clean
    pnpm clean

# Show project info
info:
    @echo "📦 Project: DNS Toys Rust Port"
    @echo "🔧 Rust version: $(rustc --version)"
    @echo "📦 Cargo version: $(cargo --version)"
