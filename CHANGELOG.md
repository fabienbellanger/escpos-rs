# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!--
## `x.y.z` (YYYY-MM-DD) [CURRENT | YANKED]

### Added (for new features)
### Changed (for changes in existing functionality)
### Deprecated (for soon-to-be removed features)
### Removed (for now removed features)
### Fixed (for any bug fixes)
### Security
-->

## [Unreleased]

### Added

- Add PDF417

## `0.3.0` (2023-11-09) [CURRENT]

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
