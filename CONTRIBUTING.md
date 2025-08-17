# Contributing to DNS Toys Rust Port

Thank you for your interest in contributing to the DNS Toys Rust port! This document provides
guidelines for contributing to this project.

## 🎯 Project Status

**Important**: This project is currently in **active porting phase** from Go to Rust. I am **NOT**
accepting new feature contributions at this time.

## 📋 What I Accept

### ✅ Welcome Contributions

- **Bug Fixes**: Fixing issues in existing ported code
- **Code Quality Improvements**:
  - Better error handling
  - Improved type safety
  - Code refactoring for better maintainability
  - Performance optimizations
  - Memory usage improvements
- **Documentation**:
  - Improving code comments
  - Updating README and other docs
  - Adding examples
  - Fixing typos or unclear explanations
- **Testing**:
  - Adding unit tests
  - Adding integration tests
  - Improving test coverage
  - Fixing failing tests
- **Configuration**:
  - Improving configuration validation
  - Adding configuration options for existing features

### ❌ Not Accepted

- **New Services**: Do not add new DNS services that weren't in the original Go version
- **New Features**: Do not add features that weren't present in the original dns.toys
- **Architecture Changes**: Do not propose major architectural changes unless they're necessary for
  the Rust port
- **Breaking Changes**: Do not make changes that break existing functionality

## 🚀 Getting Started

### Prerequisites

- Rust 1.70+ (edition 2021)
- Cargo package manager
- Git
- Node.js 18+ (for formatting)
- Just command runner (optional but recommended)

### Development Setup

1. Fork the repository
2. Clone your fork:

   ```bash
   git clone https://github.com/devharshthakur/rdns_toys.git
   cd rdns_toys
   ```

3. Install Just (optional but recommended):

   ```bash
   # macOS
   brew install just

   # Linux
   curl --proto '=https' --tlsv1.2 -sSf https://just.systems/install.sh | bash

   # Windows
   scoop install just
   ```

4. Setup the development environment:

   ```bash
   just setup
   ```

5. Create a new branch for your changes:

   ```bash
   git checkout -b fix/your-fix-description
   # or
   git checkout -b docs/your-docs-improvement
   # or
   git checkout -b test/your-test-addition
   ```

6. Make your changes
7. Run tests and format code:

   ```bash
   # Using Just (recommended)
   just check

   # Or manually:
   # Rust formatting and checks
   cargo test
   cargo clippy
   cargo fmt

   # Markdown and config file formatting
   pnpm format
   ```

8. Commit your changes with a clear, descriptive commit message
9. Push to your fork and create a pull request

## 🛠️ Development Commands

### Using Just (Recommended)

The project includes a `justfile` with common development commands:

```bash
# Run the project (default command)
just

# Build the project
just build

# Build in release mode
just build-release

# Run the project
just run

# Run in release mode
just run-release

# Check formatting
just fmt-check

# Format code and Markdown files
just format

# Clean build artifacts
just clean

# Show project info
just info
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

### Markdown and Config File Formatting

- Use Prettier for formatting Markdown, JSON, and TOML files
- Install dependencies: `pnpm install`
- Format all files: `pnpm format`
- Format only Markdown: `pnpm format:md`
- Check formatting: `pnpm format:check`
- VS Code users: Install the Prettier extension for automatic formatting on save

## 📋 Pull Request Process

### 1. Issue-First Workflow

**Important**: Before making any significant changes, please:

1. **Create an Issue First**: Open an issue describing the problem or improvement you want to make
2. **Discuss Implementation**: Communicate with me about your proposed approach
3. **Get Approval**: Wait for my approval before starting implementation
4. **Reference Issue**: Link your PR to the approved issue

This helps me understand what PRs to expect and ensures important changes are properly planned.

### 2. Branching Convention

Create branches using the format: `<type>/<issue-title>`

```bash
# Examples:
git checkout -b fix/memory-leak-in-weather-service
git checkout -b docs/improve-error-handling-docs
git checkout -b test/add-base-conversion-tests
git checkout -b refactor/improve-type-safety
```

### 3. Conventional Commits

Follow the [Conventional Commits](https://www.conventionalcommits.org/) specification:

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

#### Commit Types: Also follow's Conventional Commits guidelines

- `fix:` - Bug fixes
- `feat:` - New features (not applicable during porting phase)
- `docs:` - Documentation changes
- `style:` - Code style changes (formatting, missing semicolons, etc.)
- `refactor:` - Code refactoring
- `test:` - Adding or updating tests
- `chore:` - Maintenance tasks

#### Examples:

```bash
git commit -m "fix(weather): resolve memory leak in cache implementation"
git commit -m "docs(config): improve configuration validation documentation"
git commit -m "test(base): add unit tests for hex conversion"
git commit -m "refactor(services): improve error handling in service trait"
```

### 4. Pull Request Creation

#### PR Title Format:

```
<type>: <brief description>

Examples:
fix: resolve memory leak in weather service
docs: improve error handling documentation
test: add unit tests for base conversion service
refactor: improve type safety in config loading
```

#### PR Template:

When creating a PR, please use the [Pull Request Template](.github/pull_request_template.md).

### 5. PR Review Process

1. **Check the Porting Status**: Ensure your changes align with the current porting phase
2. **Provide Clear Description**: Explain what you changed and why
3. **Include Tests**: Add tests for any bug fixes or new functionality
4. **Update Documentation**: Update relevant documentation if needed
5. **Check CI**: Ensure all CI checks pass (If any)
6. **Self-Review**: Review your own code before submitting. It is a good practice

## 🐛 Reporting Issues

When reporting issues:

1. **Check Existing Issues**: Search for similar issues first
2. **Provide Reproducible Steps**: Include clear steps to reproduce the issue
3. **Include Environment**: Mention your Rust version, OS, etc.
4. **Add Logs**: Include relevant error messages or logs
5. **Describe Expected vs Actual**: What did you expect vs what happened

## 📚 Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [Tokio Documentation](https://tokio.rs/)
- [Original dns.toys (Go)](https://github.com/knadh/dns.toys)

## 🎉 Recognition

Contributors will be recognized in:

- The project README
- Release notes
- GitHub contributors list

## 📞 Questions?

If you have questions about contributing:

1. Check this document first
2. Look at existing issues and discussions
3. Create an issue with the "question" label

Thank you for helping make this Rust port better! 🦀
