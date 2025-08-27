# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!--
## [Unreleased]

## `x.y.z` (YYYY-MM-DD) [CURRENT | YANKED]

### Added (for new features)
### Changed (for changes in existing functionality)
### Deprecated (for soon-to-be removed features)
### Removed (for now removed features)
### Fixed (for any bug fixes)
### Security
-->

## [Unreleased]

### Changed

- Bump `futures-lite` to `2.6.1`
- Bump `serialport` to `4.7.3`
- Bump `reqwest` to `0.12.23`

## `0.16.0` (2025-06-30) [CURRENT]

### Added

- Add `UsbOption` support for `UsbDriver` ([#44](https://github.com/fabienbellanger/escpos-rs/pull/44))

## `0.15.3` (2025-06-23)

### Changed

- Bump `nusb` to `0.1.14`
- Bump `reqwest` to `0.12.20`
- Bump `serialport` to `4.7.2`

### Fixed

- Update MSRV to `1.82.0` because a `reqwest` dependency needs this version (Tests in CI failed)
- Fix CI
- Fix examples failing when cd'd into the examples
  directory ([#43](https://github.com/fabienbellanger/escpos-rs/pull/43))

## `0.15.2` (2025-04-11)

### Fixed

- Push remaining bits when breaking from bitmap creation ([#42](https://github.com/fabienbellanger/escpos-rs/pull/42))

## `0.15.1` (2025-04-04)

### Changed

- Fix documentation ([#40](https://github.com/fabienbellanger/escpos-rs/pull/40))
- Bump `nusb` to `0.1.13`
- Bump `log` to `0.4.27`
- Bump `env_logger` to `0.11.8`
- Bump `image` to `0.25.6`
- Bump `reqwest` to `0.12.15`
- Bump `serialport` to `4.7.1`
- Update Tauri example

## `0.15.0` (2025-01-16)

### Added

- Allow non-utf8 encodings to be used for Encoder ([#37](https://github.com/fabienbellanger/escpos-rs/pull/37))
- Bump `futures-lite` to `2.6.0`
- Bump `log` to `0.4.25`
- Bump `serialport` to `4.7.0`

### Fixed

- Fix Tauri example in `examples/EXAMPLES.md`

## `0.14.0` (2025-01-08)

### Added

- Add a new UI `Line` component
- Add a state for printer styles
- Add a new example for using pictures (`examples/pictures.rs`)
- Add a new Tauri example ([#36](https://github.com/fabienbellanger/escpos-rs/issues/36))
- Set `rust_version` to `1.80` in `Cargo.toml`
- Bump `serialport` to `4.6.1`
- Bump `encoding_rs` to `0.8.35`
- Bump `reqwest` to `0.12.12`
- Bump `futures-lite` to `2.5.0`
- Bump `image` to `0.25.5`
- Bump `nusb` to `0.1.12`
- Bump `env_logger` to `0.11.6`

### Changed

- Update examples
- [BREAKING] Remove `debug()` in `Printer` and `debug.rs` example
- Update `CI.yml` to use MSRV Rust toolchain
- Replace `Rc<RefCell<T>>` by `Arc<Mutex<T>>` in drivers ([#36](https://github.com/fabienbellanger/escpos-rs/issues/36))

## `0.13.1` (2024-10-14)

### Changed

- Add `Clone` and `Copy` traits on `UnderlineMode` and
  `Font` ([#33](https://github.com/fabienbellanger/escpos-rs/pull/33))
- Bump `serialport` to `4.5.1`

## `0.13.0` (2024-08-08)

### Changed

- [BREAKING] Add `PrinterOptions` to `Printer` instead of `PageCode`  
  Before:
  ```rust
  let mut printer = Printer::new(driver, Protocol::default(), Some(PageCode::PC858));
  ```
  Now:
  ```rust
  let printer_options = PrinterOptions::new(Some(PageCode::PC858), None, 42);
  let mut printer = Printer::new(driver, Protocol::default(), Some(printer_options));
  ```
  Or with default options values:
  ```rust
  let mut printer = Printer::new(driver, Protocol::default(), None);
  ```
- Remove `lazy_static` and use standard library `LazyLock` instead
- Bump `image` to `0.25.2`
- Bump `nusb` to `0.1.10`
- Bump `env_logger` to `0.11.5`
- Bump `hidapi` to `2.6.3`
- Bump `serialport` to `4.5.0`

### Fixed

- Fix documentation and `README.md`

## `0.12.2` (2024-04-23)

### Fixed

- Fix documentation

## `0.12.1` (2024-04-23)

### Fixed

- Fix documentation

## `0.12.0` (2024-04-23)

### Added

- [BREAKING] Add timeout to `NetworkDriver`

### Changed

- Improve errors in `UsbDriver`
- Bump `nusb` to `0.1.8`

## `0.11.0` (2024-04-16)

### Added

- Add table for page codes:
    - `Katakana`
    - `PC850`
    - `PC851`
    - `PC853`
    - `PC857`
    - `PC737`
    - `PC863`
    - `PC866`
    - `WPC775`
    - `PC855`
    - `PC861`
    - `PC862`
    - `PC869`
    - `PC1118`
    - `PC1119`
    - `PC1125`
    - `WPC1250`
    - `WPC1251`
    - `WPC1253`
    - `WPC1254`
    - `WPC1257`
    - `KZ1048`

### Changed

- Bump `encoding_rs` to `0.8.34`

## `0.10.0` (2024-04-09)

### Added

- Add native USB driver support with [nusb](https://crates.io/crates/nusb)

### Changed

- [BREAKING] Rename `GraphicDensity::Hight` to `GraphicDensity::High`

### Fixed

- Fix typo

## `0.9.0` (2024-03-29)

### Added

- Add printer status (`DLE EOT` command)

### Fixed

- Fix USB driver interface number

## `0.8.3` (2024-03-14)

### Added

- Add tables for page codes `PC860` and `WPC1252`

## `0.8.2` (2024-03-14)

### Fixed

- Fix USB driver

## `0.8.1` (2024-03-12)

### Fixed

- Fix unsupported `detach_kernel_driver` function on Windows

## `0.8.0` (2024-03-11)

### Added

- Add tables for page codes `ISO8859_2`, `ISO8859_7` and `ISO8859_15`
- Add `USB`, `HidApi` and `Serial port` drivers

### Changed

- Bump `image` to `0.25.0`
- Bump `log` to `0.4.21`
- Bump `env_logger` to `0.11.3`

## `0.7.3` (2024-02-26)

### Added

- Add 2 new methods `bit_image_from_bytes` and `bit_image_from_bytes_option` to `Printer`

### Changed

- Bump `image` to `0.24.9`

## `0.7.2` (2024-02-24)

### Changed

- Add Page Code 852

## `0.7.1` (2024-02-23)

### Changed

- Implement `std::error::Error` trait for `PrinterError`

### Fixed

- Fix typo

## `0.7.0` (2024-02-22)

### Changed

- [BREAKING] Manage special characters by using Page Code tables (only `PC437`, `PC865` and `PC858` are currently
  implemented).  
  The `new` method for `Printer` has a third parameter to specify the Page Code to use.  
  Before:
  ```rust
  Printer::new(driver, Protocol::default())
  ```
  Now:
  ```rust
  Printer::new(driver, Protocol::default(), None)
  Printer::new(driver, Protocol::default(), Some(PageCode::PC858))
  ```
- Bump `env_logger` to `0.11.2`

### Fixed

- Fix typo

## `0.6.2` (2024-01-29)

### Changed

- Add Copy and Clone traits to JustifyMode and CashDrawer
  enums [#4](https://github.com/fabienbellanger/escpos-rs/pull/4)
- Bump `env_logger` to `0.11.1`
- Bump `image` to `0.24.8`

## `0.6.1` (2023-12-30)

### Fixed

- Fix Barcode options [#2](https://github.com/fabienbellanger/escpos-rs/pull/2)

## `0.6.0` (2023-11-17)

### Added

- Add Aztec 2D code

### Changed

- [BREAKING] Change barcodes and 2D codes option signature

### Fixed

- Fix `lib.rs` documentation

## `0.5.0` (2023-11-15)

### Added

- Add 2 new methods `custom` and `custom_with_page_code` to `Printer`
- Add MaxiCode 2D code
- Add DataMatrix 2D code

### Changed

- [BREAKING] Merge `qrcode`, `gs1_databar` and `pdf417` into `codes_2d` feature
- Improve `receipt.rs` and `codes.rs` examples

## `0.4.0` (2023-11-13)

### Added

- Add PDF417

### Changed

- Bump `env_logger` to `0.10.1`

## `0.3.0` (2023-11-09)

### Added

- Add Select character code table command
- Add Select an international character set
- Add 2D GS1 DataBar

## `0.2.1` (2023-11-06)

### Added

- Add new example

### Changed

- Change description in `Cargo.toml`
- Add features information on [docs.rs](https://docs.rs/escpos)

## `0.2.0` (2023-11-06)

### Added

- Add `CHANGELOG.md` file
- Add GitHub action
- Add examples (in `examples` directory)

### Changed

- Improve documentation and `README.md`
- Add "option" to all barcodes
- `barcode` and `qrcode` features are now enabled by default
- [BREAKING] Remove unused `PrinterError::Network` item
- [BREAKING] Change `Printer` functions signature from `fn(self) -> Result<Self>`
  to `fn(&mut self) -> Result<&mut Self>`
