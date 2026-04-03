# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**FFmpeg Extension Library** - A Rust library for FFmpeg integration that provides high-level abstractions for FFmpeg functionality.

**Purpose**: This library wraps FFmpeg's complex C API to provide idiomatic Rust bindings with simplified interfaces for common FFmpeg operations.

## Architecture

### Module Structure

The project is organized around these core modules in `src/`:

1. **`lib.rs`** - Main library entry point, exports public API
2. **`main.rs`** - CLI application demonstration
3. **`commands.rs`** - Command-line interface implementation
4. **`chrono.rs`** - Time/date utilities for FFmpeg timestamp handling
5. **`duration.rs`** - Duration parsing and formatting (HH:MM:SS.mmm format)

### Key Design Patterns

- **FFmpeg FFI Wrapping**: The library uses Rust's FFI capabilities to wrap FFmpeg's C API
- **PathBuf-based I/O**: Input and output paths use `PathBuf` for platform-agnostic file handling
- **Focus Point Separation**: The architecture separates focus points (distinct functional areas) as indicated by the recent commit "segregate focus point of lib"

### Build Configuration

- Uses Rust toolchain file (`rust-toolchain`)
- Custom formatting rules in `rustfmt.toml`
- CI pipeline configured for release artifacts with compression

## Common Commands

### Build
```bash
cargo build
cargo build --release
```

### Run
```bash
# Run CLI application
cargo run

# Run library examples
cargo run --example <name>
```

### Test
```bash
cargo test
cargo test --lib       # Run only library tests
cargo test --bins      # Run only binary tests
cargo test --workspace # Run all tests
```

### Lint
```bash
cargo clippy
cargo fmt --check
```

### Release
```bash
cargo build --release
# CI handles artifact packaging and compression
```

## Code Structure Details

### FFmpeg Integration

The library provides safe Rust abstractions over FFmpeg's C API:
- Input/output path handling via `PathBuf`
- Stream processing utilities
- Video/audio codec detection and manipulation
- Filter graph construction

### Duration Handling (`duration.rs`)

Handles FFmpeg's time format: `HH:MM:SS.mmm`
- Parsing from string to `Duration` type
- Formatting back to string
- Arithmetic operations on durations

### Time Handling (`chrono.rs`)

Wraps `chrono` crate for timestamp operations:
- Conversion between Unix timestamps and FFmpeg timebase
- Duration calculations with timebase awareness

### CLI Interface (`commands.rs`)

Provides command-line interface:
- Subcommand architecture
- Argument parsing
- Output formatting

## Important Files

- `src/lib.rs` - Core library API
- `src/main.rs` - CLI demonstration and entry point
- `src/commands.rs` - CLI implementation
- `src/duration.rs` - Duration parsing/formatting
- `src/chrono.rs` - Time utilities
- `rust-toolchain` - Rust version requirements
- `rustfmt.toml` - Code formatting rules
- `.gitignore` - Git ignore rules

## Development Notes

- The project targets Windows MSVC (based on target fingerprints in build artifacts)
- Uses `anyhow` for error handling
- Recent refactoring focused on separating focus points in the library
- Input/output paths are now explicitly `PathBuf` rather than strings