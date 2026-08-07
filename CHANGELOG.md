# Changelog - guise-profiles

All notable changes to this project will be documented in this file.
The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.2] - 2026-08-07

### Added
- Standard metadata, status declaration (`beta`), and documentation (`README.md`, `SPEC.md`, `CHANGELOG.md`).
- Dedicated adversarial test suite (`tests/adversarial.rs`) testing User-Agent parser boundaries, header casing, named profile normalizations, and network TTL edge cases.

### Fixed
- Hardened compile-time hardware table non-emptiness check in `lib.rs` to dynamically iterate over `ALL_PROFILES` at compile time.

### Changed
- Standardized lint preamble in `lib.rs` with `clippy::pedantic` warnings and forbidden unsafe code.

## [0.1.1] - 2026-07-17

### Added
- Pure `os_network` transport-layer TCP/IP SYN fingerprint projections (`OsNetworkStack`, initial TTL de-hopping, p0f signature rendering, and JA4T fingerprint matching).
- `profile_platform` exact OS family mapping.

## [0.1.0] - 2026-06-01

### Added
- Initial release of `guise-profiles`: `StealthProfile` selector, `ProfileFacts`, catalog headers, User-Agent parser, and hardware display specs.
