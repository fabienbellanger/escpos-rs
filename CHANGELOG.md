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

## `0.7.3` (2024-02-26) [CURRENT]

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

- [Breaking] Manage special characters by using Page Code tables (only `PC437`, `PC865` and `PC858` are currently
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

- [Breaking] Change barcodes and 2D codes option signature

### Fixed

- Fix `lib.rs` documentation

## `0.5.0` (2023-11-15)

### Added

- Add 2 new methods `custom` and `custom_with_page_code` to `Printer`
- Add MaxiCode 2D code
- Add DataMatrix 2D code

### Changed

- [Breaking] Merge `qrcode`, `gs1_databar` and `pdf417` into `codes_2d` feature
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
- [Breaking] Remove unused `PrinterError::Network item`
- [Breaking] Change `Printer` functions signature from `fn(self) -> Result<Self>`
  to `fn(&mut self) -> Result<&mut Self>`
