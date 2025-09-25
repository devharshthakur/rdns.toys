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
runr:
    cargo run --release

# Test the project
test:
    cargo test

# Alias for test
t: test

# Check Rust formatting
format-check:
    cargo fmt -- --check
    biome format . --skip-parse-errors

# Format Markdown and config files
format:
    cargo fmt
    biome format --write .

# Alias for format
fmt: format

#Alias for format-check
fmtc: format-check

# Lint the project
lint:
    cargo clippy

# Clean build artifacts
clean:
    cargo clean
    pnpm clean
# Alias for clean
c: clean

# Show project info
info:
    @echo "💻 Project: rdns-toys"
    @echo "🦀 Rust version: $(rustc --version)"
    @echo "📦 Cargo version: $(cargo --version)"
    @echo "📦 pnpm version: $(pnpm --version)"
    @echo "📦 Node.js version: $(node --version)"