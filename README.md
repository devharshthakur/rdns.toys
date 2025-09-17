# DNS Toys - Rust Port

A Rust implementation of [dns.toys](https://dns.toys), a DNS server that provides various utility
services accessible via DNS queries made by [Kailash Nadh](https://github.com/knadh)

## Project Overview

This project is a complete rewrite of the original Go-based dns.toys server in Rust. DNS Toys
provides a collection of utility services that can be accessed through DNS queries, making it easy
to get information like weather, currency conversion, time zones, and more directly from the command
line or any DNS client.

## Getting Started

For installation, setup, and development instructions, please see the [Setup Guide](md/SETUP.md).

## Porting Status

See the [Porting Checklist](md/CHECKLIST.md) for detailed progress on the Go-to-Rust porting effort.

## Contributing

Please read my [Contributing Guidelines](CONTRIBUTING.md) before submitting any contributions.

**Important**: This project is currently in active porting phase. I am not accepting new feature
contributions at this time. Please focus on:

- Bug fixes
- Code quality improvements
- Documentation improvements
- Performance optimizations
- Testing improvements

## Acknowledgments

- Original [dns.toys](https://dns.toys) project by [@knadh](https://github.com/knadh)
- The Rust community for excellent tooling and libraries
- All contributors to this porting effort

## Support

If you encounter any issues or have questions:

1. Check the [Issues](https://github.com/devharshthakur/rdns_toys/issues) page
2. Search existing discussions
3. Create a new issue with detailed information

## Projects Acknowlagements

- [Original dns.toys (Go) repository](https://github.com/knadh/dns.toys)
- [dns.toys docs](https://dns.toys)

**Note**: This is a work in progress. The project is being actively ported from Go to Rust, and not
all features may be available yet. Check the [Porting Checklist](PORTING_CHECKLIST.md) for current
progress.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
