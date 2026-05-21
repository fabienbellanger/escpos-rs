//! Windows USB print driver example.
//!
//! This example uses the built-in Windows `usbprint.sys` class driver through the Win32 API
//! (no Zadig / WinUSB / libusb replacement needed). Run it on Windows only.
//!
//! With the `graphics` feature, it also prints the sample logo from
//! `resources/images/rust-logo-small.png` (same asset as the `pictures` example).
//!
//! It queries printer status before printing: ESC/POS `DLE EOT` via `ReadFile` (one request at a
//! time; batching multiple `DLE EOT` in one write can mis-align bulk-IN on `usbprint.sys`). On many
//! setups the **first** bulk-IN byte is not a valid ESC/POS status (often `0x2B`); the example sends
//! a throwaway `DLE EOT` and read to discard it before the real queries. Invalid bytes are logged
//! instead of aborting. If reads return nothing, it falls back to `WindowsUsbPrintDriver::lpt_status`
//! (`IOCTL_USBPRINT_GET_LPT_STATUS`).
//!
//! ```shell
//! RUST_LOG=debug cargo run --example usbprint --features "usbprint,graphics"
//! ```

#[cfg(all(feature = "usbprint", target_os = "windows"))]
fn main() -> escpos::errors::Result<()> {
    use escpos::driver::*;
    use escpos::errors::PrinterError;
    use escpos::printer::Printer;
    use escpos::utils::*;
    use std::thread::sleep;
    use std::time::Duration;

    env_logger::init();

    let devices = WindowsUsbPrintDriver::list()?;
    if devices.is_empty() {
        eprintln!("No USB print devices found (is the printer connected and using usbprint.sys?)");
        return Ok(());
    }

    println!("Available Windows USB printers:");
    for (i, d) in devices.iter().enumerate() {
        println!(
            "  [{i}] VID=0x{:04x?} PID=0x{:04x?} path={}",
            d.vendor_id.unwrap_or(0),
            d.product_id.unwrap_or(0),
            d.device_path
        );
    }

    // Open the first discovered printer. You can also use `open_by_vid_pid(vid, pid)` or
    // `open(device_path)` for more control.
    let driver = WindowsUsbPrintDriver::open(&devices[0].device_path)?;

    println!("Status (before print):");
    // Prime bulk-IN: the first byte read after open is often not ESC/POS (commonly 0x2B on
    // usbprint.sys). Discard it so the next reads match the DLE EOT requests.
    {
        let (n, a): (u8, u8) = RealTimeStatusRequest::Printer.into();
        driver.write(&[DLE, EOT, n, a])?;
        driver.flush()?;
        sleep(Duration::from_millis(15));
        let mut discard = [0u8; 1];
        let _ = driver.read(&mut discard);
    }

    // One `DLE EOT` per write/read.
    let mut got_escpos = false;

    for (req, label) in [
        (RealTimeStatusRequest::Printer, "Printer"),
        (RealTimeStatusRequest::RollPaperSensor, "Roll paper sensor"),
    ] {
        let (n, a): (u8, u8) = req.into();
        driver.write(&[DLE, EOT, n, a])?;
        driver.flush()?;
        sleep(Duration::from_millis(15));

        let mut b = [0u8; 1];
        match driver.read(&mut b) {
            Ok(0) => println!("  {label}: ReadFile returned 0 bytes"),
            Ok(_) => match RealTimeStatusResponse::parse(req, b[0]) {
                Ok(status) => {
                    got_escpos = true;
                    match req {
                        RealTimeStatusRequest::Printer => {
                            println!(
                                "  ESC/POS — printer online: {}",
                                status.get(&RealTimeStatusResponse::Online).unwrap_or(&false)
                            );
                        }
                        RealTimeStatusRequest::RollPaperSensor => {
                            println!(
                                "  ESC/POS — roll paper near-end, paper adequate: {}",
                                status
                                    .get(&RealTimeStatusResponse::RollPaperNearEndSensorPaperAdequate)
                                    .unwrap_or(&false)
                            );
                        }
                        _ => {}
                    }
                }
                Err(e) => {
                    eprintln!(
                        "  {label}: status byte 0x{:02x} could not be parsed ({e}); ignoring",
                        b[0]
                    );
                }
            },
            Err(e) => eprintln!("  {label}: ReadFile failed: {e}"),
        }
    }

    if !got_escpos {
        println!("  No usable ESC/POS status bytes; trying USBPRINT IOCTL…");
        match driver.lpt_status() {
            Ok(dw) => println!(
                "  USBPRINT GET_LPT_STATUS: 0x{dw:08x} (IEEE-1284-style; see printer USB/status docs)"
            ),
            Err(e) => eprintln!("  GET_LPT_STATUS: {e}"),
        }
    }

    let mut printer = Printer::new(driver, Protocol::default(), None);
    printer
        .debug_mode(Some(DebugMode::Dec))
        .init()?
        .writeln("Windows usbprint test")?;

    #[cfg(feature = "graphics")]
    {
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("resources/images/rust-logo-small.png");
        let path_str = path
            .to_str()
            .ok_or_else(|| PrinterError::Input("image path is not valid UTF-8".to_string()))?;
        printer
            .justify(JustifyMode::CENTER)?
            .bit_image_option(path_str, BitImageOption::new(Some(128), None, BitImageSize::Normal)?)?
            .feed()?;
    }

    #[cfg(not(feature = "graphics"))]
    {
        eprintln!("Tip: pass `--features \"usbprint,graphics\"` to print the sample logo image.");
    }

    printer.print_cut()?;

    Ok(())
}

#[cfg(not(all(feature = "usbprint", target_os = "windows")))]
fn main() {
    eprintln!(
        "The `usbprint` example must be built for Windows with `--features usbprint`.\n\
         To print the sample image as well, also enable `graphics`:\n\
         cargo run --example usbprint --features \"usbprint,graphics\""
    );
}
