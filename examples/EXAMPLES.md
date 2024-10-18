# ESC/POS Rust implementation - Examples

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

## Picture example

```shell
RUST_LOG=debug cargo run --example pictures --features graphics
```

## Page codes examples

```shell
RUST_LOG=debug cargo run --example page_codes
```

## Drivers examples

```shell
RUST_LOG=debug cargo run --example usb --features usb
RUST_LOG=debug cargo run --example native_usb --features native_usb
RUST_LOG=debug cargo run --example hidapi --features hidapi
RUST_LOG=debug cargo run --example serial_port --features serial_port
```

## UI examples

```shell
RUST_LOG=debug cargo run --example ui --features ui
```

## Printer status example

```shell
RUST_LOG=debug cargo run --example status --all-features
```
