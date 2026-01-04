# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

`escpos-rs` is a Rust implementation of the ESC/POS protocol for thermal receipt printers. It generates commands for printing text, barcodes, QR codes, and raster images on compatible printers.

## Build and Development Commands

### Building
```bash
cargo build --release              # Build with default features (barcodes, codes_2d)
cargo build --release --features full  # Build with all features
make build                         # Run lint, audit, test, then build release
make build-no-audit                # Build release without audit
```

### Testing
```bash
cargo test --all-features -- --nocapture  # Run all tests
make test                          # Run tests (single-threaded with nocapture)
cargo tarpaulin --all-features     # Run code coverage
make coverage                      # Run coverage (requires cargo-tarpaulin)
```

### Code Quality
```bash
make lint                          # Run rustfmt and clippy
make lint-audit                    # Run lint plus cargo audit
cargo clippy --all-features -- -D warnings  # Clippy with all features
cargo fmt                          # Format code
```

### Running Examples
```bash
# Run with specific features and logging
RUST_LOG=debug cargo run --example full --features full
RUST_LOG=debug cargo run --example receipt --features graphics
cargo run --example codes --features codes_2d
cargo run --example page_codes

# See examples/EXAMPLES.md for a complete list
```

### Documentation
```bash
make doc                           # Open docs without dependencies
make doc-public                    # Open docs including private items
make doc-deps                      # Open docs with dependencies
cargo doc --open --no-deps --all-features
```

### Other Commands
```bash
make check                         # Run lint-audit and test together
make prepare                       # Run lint, test, and verify-msrv
make upgrade                       # Upgrade and update dependencies
cargo msrv find                    # Find minimum supported Rust version
cargo msrv verify                  # Verify MSRV (currently 1.85)
```

## Code Architecture

### Module Structure

The codebase is organized into three main layers:

1. **`io/` - I/O Layer**: Driver implementations for communicating with printers
   - `driver.rs`: Defines the `Driver` trait and implementations
   - Available drivers: Console (debug), Network (TCP), USB (`usb` feature), Native USB (`native_usb` feature), HidApi (`hidapi` feature), Serial Port (`serial_port` feature), File
   - All drivers implement the `Driver` trait with `write()`, `read()`, and `flush()` methods
   - `encoder.rs`: Character encoding utilities using `encoding_rs`

2. **`domain/` - Domain Layer**: ESC/POS protocol implementation
   - `protocol.rs`: Core ESC/POS command generation (the heart of the library)
   - `page_codes.rs`: Character encoding tables (PC437, PC850, WPC1252, etc.)
   - `character.rs`: Character sets and related types
   - `constants.rs`: ESC/POS command byte sequences
   - `types.rs`: Common type definitions (JustifyMode, Font, UnderlineMode, etc.)
   - `codes/`: Barcode and 2D code implementations (barcodes.rs, qrcode.rs, pdf417.rs, etc.)
   - `bit_image.rs`: Raster image processing (`graphics` feature)
   - `graphics.rs`: Graphics-related functionality
   - `status.rs`: Printer status query and response parsing
   - `ui/`: UI components like lines, tables (`ui` feature)

3. **`printer.rs` - Printer API**: High-level fluent interface
   - Main `Printer<D: Driver>` struct that users interact with
   - Builder pattern with chained method calls (`.bold()?.writeln()?.print()?`)
   - Maintains instruction queue and style state
   - `printer_options.rs`: Configuration options

### Key Design Patterns

**Instruction Queue Pattern**: The `Printer` accumulates `Instruction` enums internally instead of sending commands immediately. Commands are only sent when `print()` or `print_cut()` is called.

**Protocol Abstraction**: The `Protocol` struct (in `domain/protocol.rs`) handles all ESC/POS command byte sequence generation. It's separate from the printer logic, making it easier to test and maintain.

**Feature-Gated Modules**: Heavy dependencies are feature-gated:
- `graphics` feature gates image processing (requires `image` crate)
- `barcodes`, `codes_2d` features gate barcode/QR code functionality
- `usb`, `native_usb`, `hidapi`, `serial_port` features gate hardware interfaces
- Default features: `barcodes`, `codes_2d`

**Driver Trait**: Custom printer drivers can be implemented by implementing the `Driver` trait, allowing flexibility in how data is sent to printers.

### Page Code Implementation

Page codes are character encoding tables used for international character support. Implementation pattern:

1. Add encoding constant to `src/domain/page_codes.rs` (e.g., `PC864_TO_UNICODE`)
2. Update `PageCode` enum in `src/domain/character.rs`
3. Implement `From<PageCode>` to return the ESC/POS byte value
4. Update `PageCodeTable::get_unicode()` to map the enum to the encoding table
5. Add tests in the `page_codes` module

Unimplemented page codes: Hiragana, PC720, PC1098, WPC1255, WPC1256, WPC1258

### Style State Tracking

The `Printer` maintains a `PrinterStyleState` that tracks current text formatting (bold, underline, font, size, etc.). This allows inspection of the current style and ensures consistent state management.

## Testing Strategy

- Unit tests are co-located with implementation code using `#[cfg(test)]` modules
- Integration tests via examples that can be run with real or mock printers
- Use `ConsoleDriver` for debugging and testing without hardware
- Tests should use `--all-features` to cover feature-gated code
- Coverage goal: ~58% (as of 2025-09-15)

## Common Patterns

### Creating a Printer
```rust
let driver = ConsoleDriver::open(true);  // or NetworkDriver, UsbDriver, etc.
let mut printer = Printer::new(driver, Protocol::default(), None);
```

### Fluent API Usage
Methods return `Result<&mut Self>` to enable chaining:
```rust
printer.init()?
    .bold(true)?
    .writeln("Hello")?
    .print_cut()?;  // Must call print() or print_cut() to execute
```

### Custom Commands
Use `custom()` or `custom_with_page_code()` for ESC/POS commands not yet implemented in the library.

### Debug Mode
Enable debug output to see raw ESC/POS commands:
```rust
printer.debug_mode(Some(DebugMode::Dec))  // or DebugMode::Hex
```

## Project Maintenance Notes

- MSRV is actively tracked and verified (currently Rust 1.85)
- Edition 2024 is used
- Security audits are part of the build process (`cargo audit`)
- Format settings in `rustfmt.toml`
- CI runs on GitHub Actions (see `.github/workflows/CI.yml`)
- Changelog maintained in `CHANGELOG.md`
- Published to crates.io as `escpos`
