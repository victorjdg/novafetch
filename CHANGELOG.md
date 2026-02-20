# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.4.0] - 2026-02-20

### Added
- **Unit tests**: Added 26 unit tests for core functionality (`parser`, `memory`, `cpu`, `disk`, `error` modules)
- **Type-safe enums**: Introduced `MemoryType` enum to replace string parameters in `memory_info()`
- **Structured data types**: Created `MemoryInfo`, `CpuInfo`, and `DiskInfo` structs to separate data from presentation
- **Generic parser module**: Created `src/parser.rs` with reusable `parse_key_value()` function to reduce code duplication
- **Automatic version**: Version is now automatically derived from `Cargo.toml` using `env!("CARGO_PKG_VERSION")`

### Changed
- **Error handling**: All info functions now return `Result<T, FetchError>` instead of `String` with "N/A"
- **CLI parsing**: Replaced manual argument parsing with `clap` derive macros for better maintainability and help generation
- **Improved code organization**: Separated data fetching from formatting/presentation logic
- **Makefile**: Fixed typo `unistall` → `uninstall`

### Technical Details
- Added `clap` dependency for CLI argument parsing
- Restructured error handling with custom `FetchError` enum
- Implemented `Display` trait for data structs
- Parser functions are now pure and easily testable

## [0.3.0] - Previous Release

### Features
- System information display (OS, Host, Kernel, Uptime, Packages, Shell, Terminal, Desktop, Resolution, Theme, CPU, Temperature, GPU, Memory, Swap, Disk, Battery)
- Support for multiple Linux distributions (Fedora, Debian, Ubuntu, Mint, Arch, openSUSE, etc.)
- Colored output with categorized labels
- Selective information display via CLI flags
- Temperature detection via `sensors` or sysfs fallback
- Resolution detection for X11 and Wayland

[Unreleased]: https://github.com/UnversedBlood/novafetch/compare/v0.4.0...HEAD
[0.4.0]: https://github.com/UnversedBlood/novafetch/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/UnversedBlood/novafetch/releases/tag/v0.3.0
