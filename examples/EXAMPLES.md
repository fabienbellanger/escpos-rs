# ESC/POS Rust implementation – Examples

## Full example

```shell
RUST_LOG=debug cargo run --example full --features full
```

## Receipt example

```shell
RUST_LOG=debug cargo run --example receipt -F full
```

## Barcodes and QR code examples

```shell
RUST_LOG=debug cargo run --example codes
```

## `no_std`-compatible codes example

Demonstrates a custom `Driver` (in-memory) and the use of barcodes/2D codes with
APIs that work in `no_std`. The example itself runs on a standard target.

```shell
cargo run --example no_std_codes
```

To verify the crate builds for a real `no_std` target (library only, this
example needs `std` to run as a binary):

```shell
cargo build --no-default-features --features barcodes,codes_2d
```

## Picture example

```shell
RUST_LOG=debug cargo run --example pictures --features graphics
```

## Page code examples

```shell
RUST_LOG=debug cargo run --example page_codes
```

## Drivers’ examples

```shell
RUST_LOG=debug cargo run --example usb --features usb
RUST_LOG=debug cargo run --example native_usb --features native_usb
RUST_LOG=debug cargo run --example hidapi --features hidapi
RUST_LOG=debug cargo run --example serial_port --features serial_port
RUST_LOG=debug cargo run --example usbprint --features "usbprint,graphics"  # Windows only (usbprint.sys + sample image)
```

## UI examples

```shell
RUST_LOG=debug cargo run --example ui --features ui
```

## Printer status example

```shell
RUST_LOG=debug cargo run --example status --all-features
```

## Tauri example

```shell
cd examples/tauri-app
npm i
npm run tauri dev
```